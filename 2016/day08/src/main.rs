
#[macro_use]
extern crate lazy_static;

extern crate regex;


use std::io;
use std::io::prelude::*;

use std::str::FromStr;
use std::fmt;

use regex::Regex;

#[derive(Debug)]
struct ParseErr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl FromStr for Instruction {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RECT_RE: Regex =
                Regex::new("rect (?P<x_size>[0-9]+)x(?P<y_size>[0-9]+)").unwrap();
            static ref ROT_ROW_RE: Regex =
                Regex::new("rotate row y=(?P<row_num>[0-9]+) by (?P<rot_by>[0-9]+)").unwrap();
            static ref ROT_COL_RE: Regex =
                Regex::new("rotate column x=(?P<col_num>[0-9]+) by (?P<rot_by>[0-9]+)").unwrap();
        }

        if RECT_RE.is_match(s) {
            let caps = try!(RECT_RE.captures(s).ok_or(ParseErr));
            let x = try!(caps.name("x_size")
                .ok_or(ParseErr)
                .and_then(|x_str| x_str.parse().map_err(|_| ParseErr)));
            let y = try!(caps.name("y_size")
                .ok_or(ParseErr)
                .and_then(|y_str| y_str.parse().map_err(|_| ParseErr)));

            Ok(Instruction::Rect(x, y))
        } else if ROT_ROW_RE.is_match(s) {
            let caps = try!(ROT_ROW_RE.captures(s).ok_or(ParseErr));
            let row_num = try!(caps.name("row_num")
                .ok_or(ParseErr)
                .and_then(|row_num_str| row_num_str.parse().map_err(|_| ParseErr)));
            let rot_by = try!(caps.name("rot_by")
                .ok_or(ParseErr)
                .and_then(|rot_by_str| rot_by_str.parse().map_err(|_| ParseErr)));

            Ok(Instruction::RotateRow(row_num, rot_by))
        } else if ROT_COL_RE.is_match(s) {
            let caps = try!(ROT_COL_RE.captures(s).ok_or(ParseErr));
            let col_num = try!(caps.name("col_num")
                .ok_or(ParseErr)
                .and_then(|row_num_str| row_num_str.parse().map_err(|_| ParseErr)));
            let rot_by = try!(caps.name("rot_by")
                .ok_or(ParseErr)
                .and_then(|rot_by_str| rot_by_str.parse().map_err(|_| ParseErr)));

            Ok(Instruction::RotateCol(col_num, rot_by))
        } else {
            Err(ParseErr)
        }
    }
}

struct Screen {
    dims: (usize, usize),
    fb: Vec<Vec<bool>>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {

        Screen {
            dims: (width, height),
            fb: vec![vec![false; height]; width],
        }
    }

    fn execute_instruction(&mut self, instr: Instruction) {
        match instr {
            Instruction::Rect(width, height) => {
                for x in 0..width {
                    for y in 0..height {
                        self.fb[x][y] = true;
                    }
                }
            }
            Instruction::RotateRow(row_idx, offset) => {
                let rot_by = offset % self.dims.0;

                let mut rotated = Vec::with_capacity(self.dims.0);
                for col_idx in ((self.dims.0 - rot_by)..self.dims.0)
                    .chain(0..(self.dims.0 - rot_by)) {
                    rotated.push(self.fb[col_idx][row_idx])
                }

                for (col_idx, state) in rotated.iter().enumerate() {
                    self.fb[col_idx][row_idx] = *state;
                }
            }
            Instruction::RotateCol(col_idx, offset) => {
                let rot_by = offset % self.dims.1;

                let mut rotated = Vec::with_capacity(self.dims.1);
                for row_idx in ((self.dims.1 - rot_by)..self.dims.1)
                    .chain(0..(self.dims.1 - rot_by)) {
                    rotated.push(self.fb[col_idx][row_idx])
                }

                for (row_idx, state) in rotated.iter().enumerate() {
                    self.fb[col_idx][row_idx] = *state;
                }
            }
        }
    }

    fn get_num_pixels_lit(&self) -> u64 {
        self.fb.iter().map(|col| col.iter().filter(|state| **state).count() as u64).sum()
    }
}


impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_idx in 0..self.dims.1 {
            for col_idx in 0..self.dims.0 {
                try!(write!(f, "{}", if self.fb[col_idx][row_idx] { '#' } else { '.' }));
            }
            try!(write!(f, "\n"));
        }

        Ok(())
    }
}


fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, ParseErr>>()
        .unwrap()
}

fn main() {
    let mut instruction_str = String::new();
    io::stdin().read_to_string(&mut instruction_str).expect("Invalid input string!");

    let instructions = parse_instructions(&instruction_str);

    let mut screen = Screen::new(50, 6);
    for instr in instructions {
        println!("--------------------------");
        println!("{:?}", instr);
        screen.execute_instruction(instr);
        println!("{}", screen);
    }

    println!("The number of lit pixels at the end is: {}",
             screen.get_num_pixels_lit());
}

#[cfg(test)]
mod tests {
    use super::parse_instructions;
    use super::Instruction;
    use super::Screen;

    const TEST_STR: &'static str = "rect 3x2\n\
                                    rotate column x=1 by 1\n\
                                    rotate row y=0 by 4\n\
                                    rotate column x=1 by 1";
    #[test]
    fn parse_test() {
        assert_eq!(parse_instructions(TEST_STR),
                   vec![Instruction::Rect(3, 2),
                        Instruction::RotateCol(1, 1),
                        Instruction::RotateRow(0, 4),
                        Instruction::RotateCol(1, 1)]);
    }

    #[test]
    fn instructions_test() {
        let instructions = parse_instructions(TEST_STR);

        let mut screen = Screen::new(7, 3);

        assert_eq!(format!("{}", screen),
                   ".......\n\
                    .......\n\
                    .......\n");

        screen.execute_instruction(instructions[0]);
        assert_eq!(format!("{}", screen),
                   "###....\n\
                    ###....\n\
                    .......\n");

        screen.execute_instruction(instructions[1]);
        assert_eq!(format!("{}", screen),
                   "#.#....\n\
                    ###....\n\
                    .#.....\n");

        screen.execute_instruction(instructions[2]);
        assert_eq!(format!("{}", screen),
                   "....#.#\n\
                    ###....\n\
                    .#.....\n");

        screen.execute_instruction(instructions[3]);
        assert_eq!(format!("{}", screen),
                   ".#..#.#\n\
                    #.#....\n\
                    .#.....\n");

    }
}
