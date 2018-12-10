use std::io;
use std::io::prelude::*;

use std::num::ParseIntError;
use std::str::FromStr;

use std::collections::{HashMap, HashSet, VecDeque};

use lazy_static::lazy_static;

use regex::Regex;

use itertools::{Itertools, MinMaxResult};

#[derive(Debug)]
struct AocError(String);

#[derive(Debug)]
enum GroupType {
    Unknown,
    Finite,
    Infinite,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Group {
    start_location: (isize, isize),
    core_locations: HashSet<(isize, isize)>,
    last_expanded_locations: HashSet<(isize, isize)>,
}

lazy_static! {
    static ref GROUP_RE: Regex = { Regex::new("^(?P<x>-?[0-9]+), (?P<y>-?[0-9]+)$").unwrap() };
}

fn distance(loc_a: (isize, isize), loc_b: (isize, isize)) -> usize {
    ((loc_b.0 - loc_a.0).abs() + (loc_b.1 - loc_a.1).abs()) as usize
}

impl FromStr for Group {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = GROUP_RE
            .captures(s.trim())
            .ok_or_else(|| AocError(format!("Invalid input: {:?}", s.trim())))?;

        let x = caps
            .name("x")
            .ok_or_else(|| AocError(format!("Missing X coordinate: {:?}", s.trim())))?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| AocError(format!("Error while parsing: {:?}", e)))?;

        let y = caps
            .name("y")
            .ok_or_else(|| AocError(format!("Missing Y coordinate: {:?}", s.trim())))?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| AocError(format!("Error while parsing: {:?}", e)))?;

        let mut start_set = HashSet::new();
        start_set.insert((x, y));

        Ok(Group {
            start_location: (x, y),
            core_locations: HashSet::new(),
            last_expanded_locations: start_set,
        })
    }
}

impl Group {
    fn get_candidate_locs(
        &self,
        already_visited: &HashSet<(isize, isize)>,
    ) -> HashSet<(isize, isize)> {
        // Just expand in every possible direction.
        // Possible locations will be filtered later.
        self.last_expanded_locations
            .iter()
            .fold(HashSet::new(), |mut acc, loc| {
                acc.insert((loc.0 - 1, loc.1));
                acc.insert((loc.0, loc.1 - 1));
                acc.insert((loc.0, loc.1 + 1));
                acc.insert((loc.0 + 1, loc.1));

                acc
            })
            .difference(already_visited)
            .cloned()
            .collect()
    }

    fn expand(
        &mut self,
        newly_expanded: HashSet<(isize, isize)>,
        starting_locs: &HashSet<(isize, isize)>,
    ) -> GroupType {
        self.core_locations.extend(&self.last_expanded_locations);

        // The group is deemed infinite if all newly expanded locations are further from all starting locations
        // than all expended locations expanded in the previous round.

        // Get closest distances to each starting location
        let current_closest = starting_locs
            .iter()
            .filter_map(|loc| {
                let min_distance = self
                    .last_expanded_locations
                    .iter()
                    .map(|new_loc| distance(*loc, *new_loc))
                    .min();

                if let Some(md) = min_distance {
                    Some((*loc, md))
                } else {
                    None
                }
            })
            .collect::<HashMap<(isize, isize), usize>>();

        // Do the same for the newly expanded locations
        let new_closest = starting_locs
            .iter()
            .filter_map(|loc| {
                let min_distance = newly_expanded
                    .iter()
                    .map(|new_loc| distance(*loc, *new_loc))
                    .min();

                if let Some(md) = min_distance {
                    Some((*loc, md))
                } else {
                    None
                }
            })
            .collect::<HashMap<(isize, isize), usize>>();

        self.last_expanded_locations = newly_expanded;

        // No new locations - easy - finite
        if self.last_expanded_locations.is_empty() {
            GroupType::Finite

        // All new locations further than all previous - infinite
        } else if current_closest
            .into_iter()
            .all(|(loc, curr_dist)| curr_dist < new_closest[&loc])
        {
            GroupType::Infinite

        // Could not determine at this time
        } else {
            GroupType::Unknown
        }
    }

    fn get_area(&self) -> HashSet<(isize, isize)> {
        self.core_locations
            .union(&self.last_expanded_locations)
            .cloned()
            .collect()
    }

    fn len(&self) -> usize {
        self.core_locations.len() + self.last_expanded_locations.len()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Groups {
    open_groups: VecDeque<Group>,
    finite_groups: Vec<Group>,
    infinite_groups: Vec<Group>,
}

impl FromStr for Groups {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut groups = VecDeque::new();

        for line in s.lines() {
            groups.push_back(line.parse()?);
        }

        Ok(Groups {
            open_groups: groups,
            finite_groups: Vec::new(),
            infinite_groups: Vec::new(),
        })
    }
}

impl Groups {
    fn expand_groups(&mut self) {
        let mut all_visited_locations =
            self.open_groups
                .iter()
                .fold(HashSet::new(), |mut acc, group| {
                    acc.extend(group.get_area());
                    acc
                });

        let starting_locs = self
            .open_groups
            .iter()
            .map(|group| group.start_location)
            .collect::<HashSet<(isize, isize)>>();

        let mut next_open_groups = VecDeque::with_capacity(self.open_groups.len());

        while !self.open_groups.is_empty() {
            // Get possible (not final) new locations that could be visited
            let newly_visited = self
                .open_groups
                .iter()
                .map(|group| {
                    (
                        group.start_location,
                        group.get_candidate_locs(&all_visited_locations),
                    )
                })
                .collect::<HashMap<(isize, isize), HashSet<(isize, isize)>>>();

            // Sum of all locations that appear in multiple newly_visited sets
            let contested = newly_visited
                .iter()
                .map(|(group_id, visited_locs)| {
                    newly_visited
                        .iter()
                        .filter(|(id, _)| group_id != *id)
                        .map(|(_, other_grp_visited)| {
                            visited_locs
                                .intersection(other_grp_visited)
                                .cloned()
                                .collect::<HashSet<(isize, isize)>>()
                        })
                        .fold(HashSet::new(), |mut acc, curr_set| {
                            acc.extend(curr_set);
                            acc
                        })
                })
                .fold(HashSet::new(), |mut acc, curr_set| {
                    acc.extend(curr_set);
                    acc
                });

            // Expand each group and try to determine its type
            while let Some(mut group) = self.open_groups.pop_front() {
                let curr_visited_locs = &newly_visited[&group.start_location];

                all_visited_locations.extend(curr_visited_locs);

                let group_type = group.expand(
                    curr_visited_locs.difference(&contested).cloned().collect(),
                    &starting_locs,
                );

                match group_type {
                    GroupType::Finite => {
                        self.finite_groups.push(group);
                    }
                    GroupType::Infinite => {
                        self.infinite_groups.push(group);
                    }
                    GroupType::Unknown => {
                        next_open_groups.push_back(group);
                    }
                }
            }

            self.open_groups = next_open_groups;
            next_open_groups = VecDeque::with_capacity(self.open_groups.len());
        }
    }

    fn get_largest_finite_size(&self) -> Option<usize> {
        self.finite_groups.iter().map(|group| group.len()).max()
    }

    fn find_acceptable_region(&self, max_distance: usize) -> HashSet<(isize, isize)> {
        let starting_locs = self
            .open_groups
            .iter()
            .chain(&self.infinite_groups)
            .chain(&self.finite_groups)
            .map(|group| group.start_location)
            .collect::<Vec<(isize, isize)>>();

        let imax_distance = max_distance as isize;

        if let MinMaxResult::MinMax(min, max) = starting_locs.iter().minmax() {
            let mut out = HashSet::new();

            for x in (max.0 - imax_distance)..=(min.0 + imax_distance) {
                for y in (max.1 - imax_distance)..=(min.1 + imax_distance) {
                    if starting_locs
                        .iter()
                        .map(|loc| distance((x, y), *loc))
                        .sum::<usize>()
                        < max_distance
                    {
                        out.insert((x, y));
                    }
                }
            }

            out
        } else {
            panic!("");
        }
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let mut groups: Groups = input_str.parse().expect("parsing error");
    groups.expand_groups();

    println!(
        "The largest finite group size is: {}",
        groups.get_largest_finite_size().expect("no finite group")
    );

    println!(
        "The size of the allowed region is: {}",
        groups.find_acceptable_region(10000).len()
    );
}

#[cfg(test)]
mod tests {
    use std::collections::{HashSet, VecDeque};

    use lazy_static::lazy_static;

    use super::Group;
    use super::Groups;

    const INPUT_STR: &str = "1, 1\n\
                             1, 6\n\
                             8, 3\n\
                             3, 4\n\
                             5, 5\n\
                             8, 9";

    lazy_static! {
        static ref GROUPS: Groups = {
            let groups_start_positions = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];

            let mut groups = VecDeque::new();

            for start_pos in groups_start_positions {
                let mut start_set = HashSet::new();
                start_set.insert(start_pos);

                groups.push_back(Group {
                    start_location: start_pos,
                    core_locations: HashSet::new(),
                    last_expanded_locations: start_set,
                });
            }

            Groups {
                open_groups: groups,
                finite_groups: Vec::new(),
                infinite_groups: Vec::new(),
            }
        };
    }

    #[test]
    fn parse_test() {
        assert_eq!(*GROUPS, INPUT_STR.parse().unwrap());
    }

    #[test]
    fn expand_groups_test() {
        let mut groups = GROUPS.clone();
        groups.expand_groups();

        assert_eq!(Some(17), groups.get_largest_finite_size());
    }

    #[test]
    fn acceptable_region_test() {
        let groups = GROUPS.clone();

        assert_eq!(16, groups.find_acceptable_region(32).len());
    }
}
