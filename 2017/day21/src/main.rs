#![feature(iterator_step_by)]

extern crate regex;

#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::prelude::*;

use std::str::FromStr;

use std::collections::HashSet;
use std::collections::HashMap;

use std::mem;

use std::rc::Rc;

use regex::Regex;

lazy_static! {
    static ref PATTERN_2_RE: Regex = Regex::new(r"^([\.#])([\.#])/([\.#])([\.#]) => ([\.#])([\.#])([\.#])/([\.#])([\.#])([\.#])/([\.#])([\.#])([\.#])$").unwrap();
    static ref PATTERN_3_RE: Regex = Regex::new(r"^([\.#])([\.#])([\.#])/([\.#])([\.#])([\.#])/([\.#])([\.#])([\.#]) => ([\.#])([\.#])([\.#])([\.#])/([\.#])([\.#])([\.#])([\.#])/([\.#])([\.#])([\.#])([\.#])/([\.#])([\.#])([\.#])([\.#])$").unwrap();
}


#[derive(Debug, Copy, Clone)]
enum PatternSize {
    Size2 = 2,
    Size3 = 3,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Pattern {
    Size2([[bool; 2]; 2]),
    Size3([[bool; 3]; 3]),
}

#[derive(Debug, PartialEq, Eq)]
enum Output {
    Size3([[bool; 3]; 3]),
    Size4([[bool; 4]; 4]),
}


struct InputLine(Pattern, Output);

impl FromStr for InputLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = PATTERN_2_RE.captures(s) {

            let mut cap_iter = caps.iter().skip(1);

            let mut pat_arr: [[bool; 2]; 2] = unsafe { mem::uninitialized() };

            for (i, cap) in cap_iter.by_ref().take(4).enumerate() {
                pat_arr[i / 2][i % 2] = match cap.ok_or(())?.as_str() {
                    "#" => true,
                    "." => false,
                    _ => return Err(()),
                };
            }


            let mut out_arr: [[bool; 3]; 3] = unsafe { mem::uninitialized() };

            for (i, cap) in cap_iter.by_ref().take(9).enumerate() {
                out_arr[i / 3][i % 3] = match cap.ok_or(())?.as_str() {
                    "#" => true,
                    "." => false,
                    _ => return Err(()),
                }
            }

            Ok(InputLine(Pattern::Size2(pat_arr), Output::Size3(out_arr)))

        } else if let Some(caps) = PATTERN_3_RE.captures(s) {

            let mut cap_iter = caps.iter().skip(1);

            let mut pat_arr: [[bool; 3]; 3] = unsafe { mem::uninitialized() };

            for (i, cap) in cap_iter.by_ref().take(9).enumerate() {
                pat_arr[i / 3][i % 3] = match cap.ok_or(())?.as_str() {
                    "#" => true,
                    "." => false,
                    _ => return Err(()),
                };
            }


            let mut out_arr: [[bool; 4]; 4] = unsafe { mem::uninitialized() };

            for (i, cap) in cap_iter.by_ref().take(16).enumerate() {
                out_arr[i / 4][i % 4] = match cap.ok_or(())?.as_str() {
                    "#" => true,
                    "." => false,
                    _ => return Err(()),
                }
            }

            Ok(InputLine(Pattern::Size3(pat_arr), Output::Size4(out_arr)))

        } else {
            Err(())
        }
    }
}


impl Pattern {
    fn generate_variants(&self) -> HashSet<Pattern> {
        let mut patterns = HashSet::new();

        for rot in 0..4 {
            let new_pat = rotate_coords(self, rot);
            let new_flipped_pat = flip_coords(&new_pat);

            patterns.insert(new_pat);
            patterns.insert(new_flipped_pat);
        }

        patterns
    }
}

fn rotate_coords(pat: &Pattern, rotate_by: usize) -> Pattern {
    let mut out = *pat;

    for _ in 0..rotate_by {
        out = match out {
            Pattern::Size2(pat_arr) => {
                Pattern::Size2(
                    [
                        [pat_arr[1][0], pat_arr[0][0]],
                        [pat_arr[1][1], pat_arr[0][1]],
                    ],
                )
            }
            Pattern::Size3(pat_arr) => {
                Pattern::Size3(
                    [
                        [pat_arr[2][0], pat_arr[1][0], pat_arr[0][0]],
                        [pat_arr[2][1], pat_arr[1][1], pat_arr[0][1]],
                        [pat_arr[2][2], pat_arr[1][2], pat_arr[0][2]],
                    ],
                )
            }
        };

    }

    out
}

fn flip_coords(pat: &Pattern) -> Pattern {
    match *pat {
        Pattern::Size2(pat_arr) => {
            Pattern::Size2(
                [
                    [pat_arr[0][1], pat_arr[0][0]],
                    [pat_arr[1][1], pat_arr[1][0]],
                ],
            )
        }
        Pattern::Size3(pat_arr) => {
            Pattern::Size3(
                [
                    [pat_arr[0][2], pat_arr[0][1], pat_arr[0][0]],
                    [pat_arr[1][2], pat_arr[1][1], pat_arr[1][0]],
                    [pat_arr[2][2], pat_arr[2][1], pat_arr[2][0]],
                ],
            )
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Rulebook(HashMap<Pattern, Rc<Output>>);

impl FromStr for Rulebook {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_lines = s.lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<InputLine>, ()>>()?;

        let mut out = HashMap::new();

        for parsed_line in parsed_lines.into_iter() {

            let output_pat = Rc::new(parsed_line.1);

            for pat in parsed_line.0.generate_variants().into_iter() {
                if out.insert(pat, output_pat.clone()).is_some() {
                    return Err(());
                }
            }
        }

        Ok(Rulebook(out))
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Grid(Vec<Vec<bool>>);

impl Grid {
    fn new(initial: &Vec<Vec<bool>>) -> Result<Grid, ()> {
        let len = initial.first().ok_or(())?.len();

        if len % 2 != 0 && len % 3 != 0 {
            return Err(());
        }

        for col in initial {
            if col.len() != len {
                return Err(());
            }
        }

        Ok(Grid(initial.clone()))
    }

    fn get_pattern_at(&self, pos_x: usize, pos_y: usize, square_size: PatternSize) -> Pattern {
        match square_size {
            PatternSize::Size2 => {
                Pattern::Size2(
                    [
                        [self.0[pos_y][pos_x], self.0[pos_y][pos_x + 1]],
                        [self.0[pos_y + 1][pos_x], self.0[pos_y + 1][pos_x + 1]],
                    ],
                )
            }
            PatternSize::Size3 => {
                Pattern::Size3(
                    [
                        [
                            self.0[pos_y][pos_x],
                            self.0[pos_y][pos_x + 1],
                            self.0[pos_y][pos_x + 2],
                        ],
                        [
                            self.0[pos_y + 1][pos_x],
                            self.0[pos_y + 1][pos_x + 1],
                            self.0[pos_y + 1][pos_x + 2],
                        ],
                        [
                            self.0[pos_y + 2][pos_x],
                            self.0[pos_y + 2][pos_x + 1],
                            self.0[pos_y + 2][pos_x + 2],
                        ],
                    ],
                )
            }
        }
    }

    fn write_output_pattern(&mut self, pos_x: usize, pos_y: usize, out_pat: &Output) {

        match *out_pat {
            Output::Size3(out_pat) => {
                for y in 0..out_pat.len() {
                    for x in 0..out_pat[y].len() {
                        self.0[pos_y + y][pos_x + x] = out_pat[y][x];
                    }
                }
            }
            Output::Size4(out_pat) => {
                for y in 0..out_pat.len() {
                    for x in 0..out_pat[y].len() {
                        self.0[pos_y + y][pos_x + x] = out_pat[y][x];
                    }
                }
            }
        }
    }

    fn expand(&self, rulebook: &Rulebook) -> Result<Grid, ()> {

        let curr_size = self.0.len();

        let square_size = if curr_size % 2 == 0 {
            PatternSize::Size2
        } else if curr_size % 3 == 0 {
            PatternSize::Size3
        } else {
            return Err(());
        };

        let new_size = curr_size / square_size as usize * (square_size as usize + 1);
        let mut new_grid = Grid(vec![vec![false; new_size]; new_size]);

        for square_y in 0..(curr_size / square_size as usize) {
            for square_x in 0..(curr_size / square_size as usize) {
                let curr_pos_x = square_x * square_size as usize;
                let curr_pos_y = square_y * square_size as usize;
                let new_pos_x = square_x * (square_size as usize + 1);
                let new_pos_y = square_y * (square_size as usize + 1);

                let curr_square = self.get_pattern_at(curr_pos_x, curr_pos_y, square_size);

                let new_square = rulebook.0.get(&curr_square).ok_or(())?;

                new_grid.write_output_pattern(new_pos_x, new_pos_y, &new_square);
            }
        }

        Ok(new_grid)
    }

    fn count_enabled_pixels(&self) -> usize {
        self.0.iter().fold(0, |outer_acc, row| {
            outer_acc +
                row.iter().fold(
                    0,
                    |acc, pixel_state| if *pixel_state { acc + 1 } else { acc },
                )
        })
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect(
        "input error",
    );

    let rulebook = input_str.parse().expect("parse error");

    let mut grid = Grid::new(&vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ]).expect("grid creation error");


    for _ in 0..5 {
        grid = grid.expand(&rulebook).expect("grid exparsion error");
    }

    println!("Number of enabled pixels after five iterations: {}", grid.count_enabled_pixels());

    for _ in 5..18 {
        grid = grid.expand(&rulebook).expect("grid exparsion error");
    }

    println!("Number of enabled pixels after eightteen iterations: {}", grid.count_enabled_pixels());
}


#[cfg(test)]
mod tests {
    use super::Pattern;
    use super::Output;
    use super::Rulebook;
    use super::Grid;
    use std::collections::HashMap;
    use std::rc::Rc;

    const TEST_STR: &str = "../.# => ##./#../...\n\
                            .#./..#/### => #..#/..../..../#..#";

    #[test]
    fn parse_test() {
        let rulebook: Rulebook = TEST_STR.parse().expect("parse error");

        let mut test_book = Rulebook(HashMap::new());

        let pattern_1_0 = Pattern::Size2([[false, false], [false, true]]);
        let pattern_1_1 = Pattern::Size2([[false, false], [true, false]]);
        let pattern_1_2 = Pattern::Size2([[true, false], [false, false]]);
        let pattern_1_3 = Pattern::Size2([[false, true], [false, false]]);

        let output_1 = Rc::new(Output::Size3(
            [
                [true, true, false],
                [true, false, false],
                [false, false, false],
            ],
        ));

        test_book.0.insert(pattern_1_0, output_1.clone());
        test_book.0.insert(pattern_1_1, output_1.clone());
        test_book.0.insert(pattern_1_2, output_1.clone());
        test_book.0.insert(pattern_1_3, output_1.clone());


        let pattern_2_0 = Pattern::Size3(
            [
                [false, true, false],
                [false, false, true],
                [true, true, true],
            ],
        );

        let pattern_2_0f = Pattern::Size3(
            [
                [false, true, false],
                [true, false, false],
                [true, true, true],
            ],
        );

        let pattern_2_1 = Pattern::Size3(
            [
                [true, false, false],
                [true, false, true],
                [true, true, false],
            ],
        );

        let pattern_2_1f = Pattern::Size3(
            [
                [false, false, true],
                [true, false, true],
                [false, true, true],
            ],
        );

        let pattern_2_2 = Pattern::Size3(
            [
                [true, true, true],
                [true, false, false],
                [false, true, false],
            ],
        );

        let pattern_2_2f = Pattern::Size3(
            [
                [true, true, true],
                [false, false, true],
                [false, true, false],
            ],
        );

        let pattern_2_3 = Pattern::Size3(
            [
                [false, true, true],
                [true, false, true],
                [false, false, true],
            ],
        );

        let pattern_2_3f = Pattern::Size3(
            [
                [true, true, false],
                [true, false, true],
                [true, false, false],
            ],
        );


        let output_2 = Rc::new(Output::Size4(
            [
                [true, false, false, true],
                [false, false, false, false],
                [false, false, false, false],
                [true, false, false, true],
            ],
        ));

        test_book.0.insert(pattern_2_0, output_2.clone());
        test_book.0.insert(pattern_2_0f, output_2.clone());
        test_book.0.insert(pattern_2_1, output_2.clone());
        test_book.0.insert(pattern_2_1f, output_2.clone());
        test_book.0.insert(pattern_2_2, output_2.clone());
        test_book.0.insert(pattern_2_2f, output_2.clone());
        test_book.0.insert(pattern_2_3, output_2.clone());
        test_book.0.insert(pattern_2_3f, output_2.clone());

        assert_eq!(test_book, rulebook);
    }

    #[test]
    fn expansion_test() {
        let rulebook: Rulebook = TEST_STR.parse().expect("parse error");

        let grid = Grid::new(&vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ]).expect("grid creation error");

        let grid = grid.expand(&rulebook).expect("grid expansion error");
        assert_eq!(
            Grid(vec![
                vec![true, false, false, true],
                vec![false, false, false, false],
                vec![false, false, false, false],
                vec![true, false, false, true],
            ]),
            grid
        );


        let grid = grid.expand(&rulebook).expect("grid expansion error");
        assert_eq!(
            Grid(vec![
                vec![true, true, false, true, true, false],
                vec![true, false, false, true, false, false],
                vec![false, false, false, false, false, false],
                vec![true, true, false, true, true, false],
                vec![true, false, false, true, false, false],
                vec![false, false, false, false, false, false],
            ]),
            grid
        );

        assert_eq!(12, grid.count_enabled_pixels());
    }
}
