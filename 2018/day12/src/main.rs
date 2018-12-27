use lazy_static::lazy_static;

use regex::Regex;

use std::str::FromStr;

use std::collections::HashSet;

use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct AocError(String);

lazy_static! {
    static ref INITIAL_STATE_RE: Regex = Regex::new(r"^initial state: (?P<state>[\.#]+)$").unwrap();
    static ref PATTERN_RE: Regex =
        Regex::new(r"^(?P<pattern>[\.#]+) => (?P<output>[\.#])$").unwrap();
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Plants {
    plant_locations: Vec<bool>,
    possible_patterns: HashSet<Vec<bool>>,
    left_offset: isize,
}

impl FromStr for Plants {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let initial_state_line = lines
            .next()
            .ok_or_else(|| AocError("missing line with initial state".into()))?;

        let state_str = INITIAL_STATE_RE
            .captures(initial_state_line)
            .ok_or_else(|| {
                AocError(format!(
                    "invalid initial state line: {:?}",
                    initial_state_line
                ))
            })?
            .name("state")
            .unwrap()
            .as_str();

        let mut plant_locations = Vec::new();
        for state_ch in state_str.chars() {
            let plant_state = match state_ch {
                '#' => true,
                '.' => false,
                other => return Err(AocError(format!("invalid plant state: {:?}", other))),
            };

            plant_locations.push(plant_state);
        }

        let mut possible_patterns = HashSet::new();
        for pattern_line in lines.filter(|line| !line.is_empty()) {
            let cap = PATTERN_RE.captures(pattern_line).ok_or_else(|| {
                AocError(format!(
                    "invalid plant pattern string: {:?}",
                    pattern_line.trim()
                ))
            })?;

            let pattern = cap.name("pattern").unwrap().as_str();

            let mut pat = Vec::new();
            for pat_ch in pattern.chars() {
                let plant_state = match pat_ch {
                    '#' => true,
                    '.' => false,
                    other => return Err(AocError(format!("invalid plant state: {:?}", other))),
                };

                pat.push(plant_state);
            }

            if cap.name("output").unwrap().as_str() == "#" {
                possible_patterns.insert(pat);
            }
        }

        Ok(Plants {
            plant_locations,
            possible_patterns,
            left_offset: 0,
        })
    }
}

impl Plants {
    fn do_step(&mut self) {
        let (padding_added, curr_state) = pad_vec(&self.plant_locations, 3);
        self.left_offset += padding_added;

        let mut new_state = vec![false; 2];

        for plant_neighborhood in curr_state.windows(5) {
            let curr_neighbours = plant_neighborhood.to_vec();

            if self.possible_patterns.contains(&curr_neighbours) {
                new_state.push(true);
            } else {
                new_state.push(false);
            }
        }

        new_state.extend(vec![false; 3]);

        self.plant_locations = new_state;
    }

    fn run_simulation(&mut self, num_iterations: usize) {
        for _ in 0..num_iterations {
            self.do_step();
        }
    }

    fn count_plants(&self) -> isize {
        self.plant_locations
            .iter()
            .enumerate()
            .filter(|(_, plant_present)| **plant_present)
            .map(|(pos, _)| pos as isize - self.left_offset as isize)
            .sum()
    }
}

fn pad_vec(input: &[bool], pad_count: usize) -> (isize, Vec<bool>) {
    let leading_false = input.iter().take_while(|val| !*val).count();
    let trailing_false = input.iter().rev().take_while(|val| !*val).count();

    let start_pos = leading_false;
    let end_pos = input.len() - trailing_false;

    let mut out = Vec::new();

    out.extend(vec![false; pad_count]);
    out.extend(&input[start_pos..end_pos]);
    out.extend(vec![false; pad_count]);

    (pad_count as isize - leading_false as isize, out)
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let mut plants: Plants = input_str.parse().expect("parsing error");

    plants.run_simulation(20);
    println!(
        "The sum of the numbers of all pots is: {}",
        plants.count_plants()
    );

    // Print out some values for analysis
    let mut plants: Plants = input_str.parse().expect("parsing error");
    for i in 0..1_000_000 {
        if i % 10_000 == 0 {
            println!("{} {}", i, plants.count_plants());
        }

        plants.do_step();
    }

    // After a while, the result is f(x) = 22 * x + 475
    println!("{}", 22i64 * 50_000_000_000i64 + 475i64)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::pad_vec;
    use super::Plants;

    use lazy_static::lazy_static;

    const INPUT_STR: &str = "initial state: #..#.#..##......###...###\n\
                             \n\
                             ...## => #\n\
                             ..#.. => #\n\
                             .#... => #\n\
                             .#.#. => #\n\
                             .#.## => #\n\
                             .##.. => #\n\
                             .#### => #\n\
                             #.#.# => #\n\
                             #.### => #\n\
                             ##.#. => #\n\
                             ##.## => #\n\
                             ###.. => #\n\
                             ###.# => #\n\
                             ####. => #";

    const EXPECTED_STATE_STRS: [&str; 21] = [
        "...#..#.#..##......###...###...........",
        "...#...#....#.....#..#..#..#...........",
        "...##..##...##....#..#..#..##..........",
        "..#.#...#..#.#....#..#..#...#..........",
        "...#.#..#...#.#...#..#..##..##.........",
        "....#...##...#.#..#..#...#...#.........",
        "....##.#.#....#...#..##..##..##........",
        "...#..###.#...##..#...#...#...#........",
        "...#....##.#.#.#..##..##..##..##.......",
        "...##..#..#####....#...#...#...#.......",
        "..#.#..#...#.##....##..##..##..##......",
        "...#...##...#.#...#.#...#...#...#......",
        "...##.#.#....#.#...#.#..##..##..##.....",
        "..#..###.#....#.#...#....#...#...#.....",
        "..#....##.#....#.#..##...##..##..##....",
        "..##..#..#.#....#....#..#.#...#...#....",
        ".#.#..#...#.#...##...#...#.#..##..##...",
        "..#...##...#.#.#.#...##...#....#...#...",
        "..##.#.#....#####.#.#.#...##...##..##..",
        ".#..###.#..#.#.#######.#.#.#..#.#...#..",
        ".#....##....#####...#######....#.#..##.",
    ];

    lazy_static! {
        static ref PLANTS: Plants = {
            let plant_locations = vec![
                true, false, false, true, false, true, false, false, true, true, false, false,
                false, false, false, false, true, true, true, false, false, false, true, true,
                true,
            ];

            let mut patterns = HashSet::new();
            patterns.insert(vec![false, false, false, true, true]);
            patterns.insert(vec![false, false, true, false, false]);
            patterns.insert(vec![false, true, false, false, false]);
            patterns.insert(vec![false, true, false, true, false]);
            patterns.insert(vec![false, true, false, true, true]);
            patterns.insert(vec![false, true, true, false, false]);
            patterns.insert(vec![false, true, true, true, true]);
            patterns.insert(vec![true, false, true, false, true]);
            patterns.insert(vec![true, false, true, true, true]);
            patterns.insert(vec![true, true, false, true, false]);
            patterns.insert(vec![true, true, false, true, true]);
            patterns.insert(vec![true, true, true, false, false]);
            patterns.insert(vec![true, true, true, false, true]);
            patterns.insert(vec![true, true, true, true, false]);

            Plants {
                plant_locations,
                possible_patterns: patterns,
                left_offset: 0,
            }
        };
        static ref EXPECTED_STATES: Vec<Vec<bool>> = {
            let mut out = Vec::new();

            for state_str in EXPECTED_STATE_STRS.iter() {
                let mut curr_state = Vec::new();

                for ch in state_str.chars() {
                    let plant_state = match ch {
                        '#' => true,
                        '.' => false,
                        _ => panic!("bad plant state"),
                    };

                    curr_state.push(plant_state);
                }

                out.push(curr_state);
            }

            out
        };
    }

    #[test]
    fn parse_test() {
        let parsed_plants: Plants = INPUT_STR.parse().unwrap();

        assert_eq!(*PLANTS, parsed_plants);
    }

    #[test]
    fn generation_step() {
        let mut plants = PLANTS.clone();

        let expected = pad_vec(&EXPECTED_STATES[0], 2).1;
        let current = pad_vec(&plants.plant_locations, 2).1;

        assert_eq!(expected, current);

        for expected_state in EXPECTED_STATES.iter().skip(1) {
            plants.do_step();

            let expected = pad_vec(expected_state, 2).1;
            let current = pad_vec(&plants.plant_locations, 2).1;

            assert_eq!(expected, current);
        }

        assert_eq!(325, plants.count_plants());
    }
}
