use std::str::FromStr;

use std::collections::BTreeMap;
use std::collections::HashMap;

use std::io;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct AocError(String);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TrackType {
    Vertical,
    TurnSlash,
    TurnBackslash,
    Horizontal,
    Intersection,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Train {
    dir: Direction,
    intersection_cnt: usize,
}

impl Train {
    fn new(dir: Direction) -> Train {
        Train {
            dir,
            intersection_cnt: 0
        }
    }
}

type Location = (usize, usize);

#[derive(Debug, Eq, PartialEq, Clone)]
struct Railroad {
    tracks: HashMap<Location, TrackType>,
    trains: BTreeMap<Location, Train>,
}

impl FromStr for Railroad {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tracks = HashMap::new();
        let mut trains = BTreeMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '|' => {
                        tracks.insert((y, x), TrackType::Vertical);
                    }
                    '/' => {
                        tracks.insert((y, x), TrackType::TurnSlash);
                    }
                    '\\' => {
                        tracks.insert((y, x), TrackType::TurnBackslash);
                    }
                    '-' => {
                        tracks.insert((y, x), TrackType::Horizontal);
                    }
                    '+' => {
                        tracks.insert((y, x), TrackType::Intersection);
                    }
                    '^' => {
                        trains.insert((y, x), Train::new(Direction::North));
                    }
                    '>' => {
                        trains.insert((y, x), Train::new(Direction::East));
                    }
                    'v' => {
                        trains.insert((y, x), Train::new(Direction::South));
                    }
                    '<' => {
                        trains.insert((y, x), Train::new(Direction::West));
                    }
                    ' ' | '\n' => {}
                    _ => {
                        return Err(AocError(format!(
                            "Invalid character in input map '{}' at line {}, column {}.",
                            ch, y, x
                        )))
                    }
                }
            }
        }

        for ((y, x), train) in &trains {
            let track_type = match train.dir {
                Direction::North | Direction::South => TrackType::Vertical,
                Direction::East | Direction::West => TrackType::Horizontal,
            };

            tracks.insert((*y, *x), track_type);
        }

        Ok(Railroad { tracks, trains })
    }
}

impl Railroad {
    fn run_until_last(&mut self) -> Result<(Option<Location>, Option<Location>), AocError> {
        let mut first_crash_loc = None;

        loop {
            if self.trains.len() <= 1 {
                let last_train_loc = self.trains.keys().next().and_then(|val| Some(*val));

                return Ok((last_train_loc, first_crash_loc));
            }

            let train_order = self.trains.keys().cloned().collect::<Vec<Location>>();

            for curr_train_loc in train_order {
                let mut curr_train = if let Some(ct) = self.trains.get(&curr_train_loc) {
                    *ct
                } else {
                    continue;
                };

                let next_loc = match curr_train.dir {
                    Direction::North => (curr_train_loc.0.checked_sub(1).ok_or_else(|| AocError("train ran off the tracks".to_string()))?, curr_train_loc.1),
                    Direction::East => (curr_train_loc.0, curr_train_loc.1 + 1),
                    Direction::South => (curr_train_loc.0 + 1, curr_train_loc.1),
                    Direction::West => (curr_train_loc.0, curr_train_loc.1.checked_sub(1).ok_or_else(|| AocError("train ran off the tracks".to_string()))?),
                };

                let next_track = self.tracks.get(&next_loc).ok_or_else(|| AocError("train ran off the tracks".to_string()))?;

                let new_dir = match curr_train.dir {
                    Direction::North => {
                        match next_track {
                            TrackType::Vertical => Direction::North,
                            TrackType::TurnSlash => Direction::East,
                            TrackType::TurnBackslash => Direction::West,
                            TrackType::Horizontal => return Err(AocError("encountered crossing track with no intersection".to_string())),
                            TrackType::Intersection => {
                                let dir = match curr_train.intersection_cnt {
                                    0 => Direction::West, // left
                                    1 => Direction::North, // straight
                                    2 => Direction::East, // right
                                    _ => panic!()
                                };

                                curr_train.intersection_cnt += 1;
                                curr_train.intersection_cnt %= 3;

                                dir
                            },
                        }
                    },
                    Direction::East => {
                        match next_track {
                            TrackType::Vertical => return Err(AocError("encountered crossing track with no intersection".to_string())),
                            TrackType::TurnSlash => Direction::North,
                            TrackType::TurnBackslash => Direction::South,
                            TrackType::Horizontal => Direction::East,
                            TrackType::Intersection => {
                                let dir = match curr_train.intersection_cnt {
                                    0 => Direction::North, // left
                                    1 => Direction::East, // straight
                                    2 => Direction::South, // right
                                    _ => panic!()
                                };

                                curr_train.intersection_cnt += 1;
                                curr_train.intersection_cnt %= 3;

                                dir
                            },
                        }
                    },
                    Direction::South => {
                        match next_track {
                            TrackType::Vertical => Direction::South,
                            TrackType::TurnSlash => Direction::West,
                            TrackType::TurnBackslash => Direction::East,
                            TrackType::Horizontal => return Err(AocError("encountered crossing track with no intersection".to_string())),
                            TrackType::Intersection => {
                                let dir = match curr_train.intersection_cnt {
                                    0 => Direction::East, // left
                                    1 => Direction::South, // straight
                                    2 => Direction::West, // right
                                    _ => panic!()
                                };

                                curr_train.intersection_cnt += 1;
                                curr_train.intersection_cnt %= 3;

                                dir
                            },
                        }
                    },
                    Direction::West => {
                        match next_track {
                            TrackType::Vertical => return Err(AocError("encountered crossing track with no intersection".to_string())),
                            TrackType::TurnSlash => Direction::South,
                            TrackType::TurnBackslash => Direction::North,
                            TrackType::Horizontal => Direction::West,
                            TrackType::Intersection => {
                                let dir = match curr_train.intersection_cnt {
                                    0 => Direction::South, // left
                                    1 => Direction::West, // straight
                                    2 => Direction::North, // right
                                    _ => panic!()
                                };

                                curr_train.intersection_cnt += 1;
                                curr_train.intersection_cnt %= 3;

                                dir
                            },
                        }
                    },
                };

                curr_train.dir = new_dir;

                self.trains.remove(&curr_train_loc);
                if self.trains.insert(next_loc, curr_train).is_some() {
                    if first_crash_loc.is_none() {
                        first_crash_loc = Some(next_loc);
                    }

                    self.trains.remove(&next_loc);
                }
            }
        }
    }
}


fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let mut railroad: Railroad = input_str.parse().expect("parsing error");

    let (last_train_at, first_crash_at) = railroad.run_until_last().unwrap();

    println!("The first crash occurred at ({}, {})", first_crash_at.unwrap().1, first_crash_at.unwrap().0);
    println!("Last train remaining is at ({}, {})", last_train_at.unwrap().1, last_train_at.unwrap().0);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    use super::Direction;
    use super::Railroad;
    use super::TrackType;
    use super::Train;

    use lazy_static::lazy_static;

    //             1111
    //   01234567890123
    // 0 /->-\
    // 1 |   |  /----\
    // 2 | /-+--+-\  |
    // 3 | | |  | v  |
    // 4 \-+-/  \-+--/
    // 5   \------/"
    const INPUT_STR: &str =
"/->-\\
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/";

    lazy_static! {
        static ref EXPECTED: Railroad = {
            let mut tracks = HashMap::new();
            tracks.insert((0, 0), TrackType::TurnSlash);
            tracks.insert((0, 1), TrackType::Horizontal);
            tracks.insert((0, 2), TrackType::Horizontal);
            tracks.insert((0, 3), TrackType::Horizontal);
            tracks.insert((0, 4), TrackType::TurnBackslash);

            tracks.insert((1, 0), TrackType::Vertical);
            tracks.insert((1, 4), TrackType::Vertical);
            tracks.insert((1, 7), TrackType::TurnSlash);
            tracks.insert((1, 8), TrackType::Horizontal);
            tracks.insert((1, 9), TrackType::Horizontal);
            tracks.insert((1, 10), TrackType::Horizontal);
            tracks.insert((1, 11), TrackType::Horizontal);
            tracks.insert((1, 12), TrackType::TurnBackslash);

            tracks.insert((2, 0), TrackType::Vertical);
            tracks.insert((2, 2), TrackType::TurnSlash);
            tracks.insert((2, 3), TrackType::Horizontal);
            tracks.insert((2, 4), TrackType::Intersection);
            tracks.insert((2, 5), TrackType::Horizontal);
            tracks.insert((2, 6), TrackType::Horizontal);
            tracks.insert((2, 7), TrackType::Intersection);
            tracks.insert((2, 8), TrackType::Horizontal);
            tracks.insert((2, 9), TrackType::TurnBackslash);
            tracks.insert((2, 12), TrackType::Vertical);

            tracks.insert((3, 0), TrackType::Vertical);
            tracks.insert((3, 2), TrackType::Vertical);
            tracks.insert((3, 4), TrackType::Vertical);
            tracks.insert((3, 7), TrackType::Vertical);
            tracks.insert((3, 9), TrackType::Vertical);
            tracks.insert((3, 12), TrackType::Vertical);

            tracks.insert((4, 0), TrackType::TurnBackslash);
            tracks.insert((4, 1), TrackType::Horizontal);
            tracks.insert((4, 2), TrackType::Intersection);
            tracks.insert((4, 3), TrackType::Horizontal);
            tracks.insert((4, 4), TrackType::TurnSlash);
            tracks.insert((4, 7), TrackType::TurnBackslash);
            tracks.insert((4, 8), TrackType::Horizontal);
            tracks.insert((4, 9), TrackType::Intersection);
            tracks.insert((4, 10), TrackType::Horizontal);
            tracks.insert((4, 11), TrackType::Horizontal);
            tracks.insert((4, 12), TrackType::TurnSlash);

            tracks.insert((5, 2), TrackType::TurnBackslash);
            tracks.insert((5, 3), TrackType::Horizontal);
            tracks.insert((5, 4), TrackType::Horizontal);
            tracks.insert((5, 5), TrackType::Horizontal);
            tracks.insert((5, 6), TrackType::Horizontal);
            tracks.insert((5, 7), TrackType::Horizontal);
            tracks.insert((5, 8), TrackType::Horizontal);
            tracks.insert((5, 9), TrackType::TurnSlash);

            let mut trains = BTreeMap::new();
            trains.insert((0, 2), Train::new(Direction::East));
            trains.insert((3, 9), Train::new(Direction::South));

            Railroad { tracks, trains }
        };
    }

    #[test]
    fn parse_test() {
        let railroad: Railroad = INPUT_STR.parse().unwrap();

        assert_eq!(*EXPECTED, railroad);
    }

    #[test]
    fn run_test() {
        let mut railroad: Railroad = EXPECTED.clone();

        let (last_train_loc, first_crash_loc) = railroad.run_until_last().unwrap();

        assert_eq!(Some((3, 7)), first_crash_loc);
        assert_eq!(None, last_train_loc);
    }
}
