use std::io;
use std::io::prelude::*;

use std::fmt;
use std::fmt::Write;

use std::str::FromStr;

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq)]
enum Search {
    Ongoing,
    CharSeen(char),
    Done,
}

struct Maze {
    maze: Vec<Vec<char>>,
    curr_pos: (usize, usize),
    curr_dir: Dir,
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(maze_str: &str) -> Result<Self, Self::Err> {
        let maze = maze_str
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut curr_pos = None;

        for (row_idx, line) in maze.iter().enumerate() {
            for (col_idx, ch) in line.iter().enumerate() {
                if *ch == '|' {
                    curr_pos = Some((row_idx, col_idx));
                    break;
                }
            }

            if curr_pos.is_some() {
                break;
            }
        }

        Ok(Maze {
            maze,
            curr_pos: curr_pos.ok_or(())?,
            curr_dir: Dir::Down,
        })
    }
}

impl Maze {
    fn do_step(&mut self) -> Result<Search, ()> {
        let (curr_row, curr_col) = self.curr_pos;

        match *self.maze.get(curr_row).ok_or(())?.get(curr_col).ok_or(())? {
            '|' | '-' => {
                match self.curr_dir {
                    Dir::Up => {
                        self.curr_pos = (curr_row - 1, curr_col);
                    }
                    Dir::Right => {
                        self.curr_pos = (curr_row, curr_col + 1);
                    }
                    Dir::Down => {
                        self.curr_pos = (curr_row + 1, curr_col);
                    }
                    Dir::Left => {
                        self.curr_pos = (curr_row, curr_col - 1);
                    }
                }

                Ok(Search::Ongoing)
            }
            '+' => {
                match self.curr_dir {
                    Dir::Up | Dir::Down => {
                        let left = self.maze.get(curr_row).and_then(
                            |row| row.get(curr_col - 1),
                        );
                        let right = self.maze.get(curr_row).and_then(
                            |row| row.get(curr_col + 1),
                        );

                        if let Some(left_ch) = left {
                            if *left_ch == '-' || *left_ch == '+' || left_ch.is_alphabetic() {
                                self.curr_pos = (curr_row, curr_col - 1);
                                self.curr_dir = Dir::Left;

                                return Ok(Search::Ongoing);
                            }
                        }

                        if let Some(right_ch) = right {
                            if *right_ch == '-' || *right_ch == '+' || right_ch.is_alphabetic() {
                                self.curr_pos = (curr_row, curr_col + 1);
                                self.curr_dir = Dir::Right;

                                return Ok(Search::Ongoing);
                            }
                        }

                        Err(())
                    }
                    Dir::Right | Dir::Left => {
                        let top = self.maze.get(curr_row - 1).and_then(
                            |row| row.get(curr_col),
                        );
                        let bottom = self.maze.get(curr_row + 1).and_then(
                            |row| row.get(curr_col),
                        );

                        if let Some(top_ch) = top {
                            if *top_ch == '|' || *top_ch == '+' || top_ch.is_alphabetic() {
                                self.curr_pos = (curr_row - 1, curr_col);
                                self.curr_dir = Dir::Up;

                                return Ok(Search::Ongoing);
                            }
                        }

                        if let Some(bottom_ch) = bottom {
                            if *bottom_ch == '|' || *bottom_ch == '+' || bottom_ch.is_alphabetic() {
                                self.curr_pos = (curr_row + 1, curr_col);
                                self.curr_dir = Dir::Down;

                                return Ok(Search::Ongoing);
                            }
                        }

                        Err(())
                    }
                }
            }
            ch if ch.is_alphabetic() => {
                match self.curr_dir {
                    Dir::Up => {
                        self.curr_pos = (curr_row - 1, curr_col);
                    }
                    Dir::Right => {
                        self.curr_pos = (curr_row, curr_col + 1);
                    }
                    Dir::Down => {
                        self.curr_pos = (curr_row + 1, curr_col);
                    }
                    Dir::Left => {
                        self.curr_pos = (curr_row, curr_col - 1);
                    }
                }

                Ok(Search::CharSeen(ch))
            }
            _ => Ok(Search::Done),
        }
    }

    fn walk_maze(&mut self) -> Result<(usize, String), ()> {
        let mut out = String::new();
        let mut step_cnt = 0;

        loop {
            let step_res = self.do_step()?;

            match step_res {
                Search::CharSeen(ch) => out.push(ch),
                Search::Done => {
                    break;
                }
                _ => {}
            }

            step_cnt += 1;
        }


        Ok((step_cnt, out))
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (row_idx, line) in self.maze.iter().enumerate() {

            for (col_idx, ch) in line.iter().enumerate() {
                if self.curr_pos == (row_idx, col_idx) {
                    f.write_char('x')?;
                } else {
                    f.write_char(*ch)?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}


fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect(
        "input error",
    );

    let mut maze: Maze = input_str.parse().expect("parse error");

    println!("Hello, world! {:?}", maze.walk_maze());
}

#[cfg(test)]
mod tests {

    use super::Maze;

    #[test]
    fn navigation_test() {
        let maze_str = "     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+ ";

        let mut maze: Maze = maze_str.parse().expect("parse error");

        let res = maze.walk_maze().expect("maze walk error");

        assert_eq!((38, "ABCDEF".to_string()), res);
    }
}
