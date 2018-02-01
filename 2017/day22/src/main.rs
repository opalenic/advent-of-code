
extern crate colored;

use colored::*;

use std::collections::HashMap;

use std::str::FromStr;

use std::io;
use std::io::prelude::*;

use std::fmt;

use std::cmp;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
enum Node {
    Weakened,
    Infected,
    Flagged,
}


impl Direction {
    fn turn_right(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_left(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    infected_nodes: HashMap<(isize, isize), Node>,
    curr_loc: (isize, isize),
    curr_dir: Direction,
    infection_ctr: usize,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input_str: &str) -> Result<Grid, Self::Err> {
        let mut size = None;
        let mut nodes = HashMap::new();

        for (row_idx, line_str) in input_str.lines().enumerate() {
            if let Some(s) = size {
                if s != line_str.len() {
                    return Err(());
                }
            } else {
                size = Some(line_str.len());
            }

            for (col_idx, ch) in line_str.chars().enumerate() {
                match ch {
                    '#' => {
                        nodes.insert((row_idx as isize, col_idx as isize), Node::Infected);
                    }
                    '.' => {}
                    _ => {
                        return Err(());
                    }
                }
            }
        }

        let s = size.ok_or(())?;

        if input_str.lines().count() != s {
            return Err(());
        }

        let pos = ((s / 2) as isize, (s / 2) as isize);

        Ok(Grid {
            infected_nodes: nodes,
            curr_loc: pos,
            curr_dir: Direction::Up,
            infection_ctr: 0,
        })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {

        let dimensions: Option<((isize, isize), (isize, isize))> = self.infected_nodes
            .keys()
            .fold(None, |dims, node_coords| if let Some(d) = dims {
                Some((
                    (
                        cmp::min((d.0).0, node_coords.0),
                        cmp::min((d.0).1, node_coords.1),
                    ),
                    (
                        cmp::max((d.1).0, node_coords.0),
                        cmp::max((d.1).1, node_coords.1),
                    ),
                ))
            } else {
                Some((
                    (node_coords.0 - 1, node_coords.1 - 1),
                    (node_coords.0 + 1, node_coords.1 + 1)
                ))
            });

        if let Some(d) = dimensions {
            for row_idx in ((d.0).0)..((d.1).0 + 1) {
                for col_idx in ((d.0).1)..((d.1).1 + 1) {
                    let node = self.infected_nodes.get(&(row_idx, col_idx));

                    let out = match node {
                        Some(&Node::Infected) => "#",
                        Some(&Node::Weakened) => "W",
                        Some(&Node::Flagged) => "F",
                        None => ".",
                    };

                    let out = if (row_idx, col_idx) == self.curr_loc {
                        out.on_red()
                    } else {
                        out.normal()
                    };

                    write!(w, "{}", out)?;
                }

                writeln!(w)?;
            }
        }

        Ok(())
    }
}


impl Grid {
    fn do_step(&mut self) {
        let node = self.infected_nodes.remove(&self.curr_loc);

        match node {
            Some(Node::Infected) => {
                self.curr_dir = self.curr_dir.turn_right();
            }
            None => {
                self.curr_dir = self.curr_dir.turn_left();
                self.infected_nodes.insert(self.curr_loc, Node::Infected);
                self.infection_ctr += 1;
            }
            Some(state) => {
                panic!("invalid state: {:?}", state);
            }
        }

        self.curr_loc = match self.curr_dir {
            Direction::Up => (self.curr_loc.0 - 1, self.curr_loc.1),
            Direction::Down => (self.curr_loc.0 + 1, self.curr_loc.1),
            Direction::Left => (self.curr_loc.0, self.curr_loc.1 - 1),
            Direction::Right => (self.curr_loc.0, self.curr_loc.1 + 1),
        }
    }

    fn do_step_b(&mut self) {
        let node = self.infected_nodes.remove(&self.curr_loc);

        let new_node = match node {
            None => Some(Node::Weakened),
            Some(Node::Weakened) => {
                self.infection_ctr += 1;

                Some(Node::Infected)
            }
            Some(Node::Infected) => Some(Node::Flagged),
            Some(Node::Flagged) => None,
        };

        match node {
            None => {
                self.curr_dir = self.curr_dir.turn_left();
            }
            Some(Node::Weakened) => {}
            Some(Node::Infected) => {
                self.curr_dir = self.curr_dir.turn_right();
            }
            Some(Node::Flagged) => {
                self.curr_dir = self.curr_dir.turn_right().turn_right();
            }
        }

        if let Some(nn) = new_node {
            self.infected_nodes.insert(self.curr_loc, nn);
        }

        self.curr_loc = match self.curr_dir {
            Direction::Up => (self.curr_loc.0 - 1, self.curr_loc.1),
            Direction::Down => (self.curr_loc.0 + 1, self.curr_loc.1),
            Direction::Left => (self.curr_loc.0, self.curr_loc.1 - 1),
            Direction::Right => (self.curr_loc.0, self.curr_loc.1 + 1),
        }
    }


    fn get_num_infection_events(&self) -> usize {
        self.infection_ctr
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect(
        "input error",
    );

    let mut grid: Grid = input_str.parse().expect("parsing error");

    for _ in 0..10_000 {
        grid.do_step();
    }

    println!(
        "Total number of infection events, part A: {}",
        grid.get_num_infection_events()
    );


    let mut grid_b: Grid = input_str.parse().expect("parsing error");

    for _ in 0..10_000_000 {
        grid_b.do_step_b();
    }

    println!(
        "Total number of infection events, part B: {}",
        grid_b.get_num_infection_events()
    );
}


#[cfg(test)]
mod tests {
    use super::Grid;
    use super::Node;
    use super::Direction;
    use std::collections::HashMap;

    const TEST_INPUT: &str = "..#\n#..\n...\n";

    #[test]
    fn parse_test() {
        let grid: Grid = TEST_INPUT.parse().expect("parsing error");

        let mut infected = HashMap::new();
        infected.insert((0, 2), Node::Infected);
        infected.insert((1, 0), Node::Infected);

        let expected = Grid {
            infected_nodes: infected,
            curr_loc: (1, 1),
            curr_dir: Direction::Up,
            infection_ctr: 0,
        };

        assert_eq!(expected, grid);
    }


    #[test]
    fn infection_count_test() {
        let mut grid: Grid = TEST_INPUT.parse().expect("parsing error");

        for _ in 0..10_000 {
            grid.do_step();
        }

        assert_eq!(5587, grid.get_num_infection_events());
    }

    #[test]
    fn infection_count_b_test() {
        let mut grid: Grid = TEST_INPUT.parse().expect("parsing error");

        for _ in 0..10_000_000 {
            grid.do_step_b();
        }

        assert_eq!(2511944, grid.get_num_infection_events());
    }
}
