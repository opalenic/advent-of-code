
use std::io;
use std::io::prelude::*;

use std::collections::BTreeSet;


#[derive(PartialEq, Eq, Debug)]
enum Decision {
    Left(u32),
    Right(u32)
}

enum Heading {
    North,
    East,
    South,
    West
}

impl<'a> From<&'a str> for Decision {
    fn from(input: &'a str) -> Decision {
        let trimmed = input.trim();

        let (dir, dist) = (&trimmed[0..1], trimmed[1..].parse::<u32>().unwrap());

        match dir {
            "L" => { Decision::Left(dist) }
            "R" => { Decision::Right(dist) }
            _ => {
                panic!("Bad Decision: {}", trimmed);
            }
        }
    }
}

struct Path {
    path: Vec<Decision>
}

impl<'a> From<&'a str> for Path {
    fn from(input: &'a str) -> Path {
        Path {
            path: input.split(",").map(|dir_str| dir_str.into()).collect::<Vec<Decision>>(),
        }
    }
}

impl Path {
    fn step(curr_heading: &Heading, next_step: &Decision) -> (Heading, i64, i64) {

        match *curr_heading {
            Heading::North => {
                match *next_step {
                    Decision::Left(dist) => {
                        (Heading::West, -(dist as i64), 0)
                    }
                    Decision::Right(dist) => {
                        (Heading::East, dist as i64, 0)
                    }
                }
            },
            Heading::East => {
                match *next_step {
                    Decision::Left(dist) => {
                        (Heading::North, 0, dist as i64)
                    },
                    Decision::Right(dist) => {
                        (Heading::South, 0, -(dist as i64))
                    }
                }
            },
            Heading::South => {
                match *next_step {
                    Decision::Left(dist) => {
                        (Heading::East, dist as i64, 0)
                    }
                    Decision::Right(dist) => {
                        (Heading::West, -(dist as i64), 0)
                    }
                }
            }
            Heading::West => {
                match *next_step {
                    Decision::Left(dist) => {
                        (Heading::South, 0, -(dist as i64))
                    }
                    Decision::Right(dist) => {
                        (Heading::North, 0, dist as i64)
                    }
                }
            }
        }
    }

    fn get_dist(&self) -> u64 {
        let mut pos: (i64, i64) = (0, 0);

        let mut heading = Heading::North;

        for step in &self.path {
            let (new_heading, diff_x, diff_y) = Path::step(&heading, step);

            heading = new_heading;
            pos.0 += diff_x;
            pos.1 += diff_y
        }

        pos.0.abs() as u64 + pos.1.abs() as u64
    }

    fn get_twice_visited_dist(&self) -> Option<u64> {
        let mut pos: (i64, i64) = (0, 0);
        let mut heading = Heading::North;
        let mut already_visited = BTreeSet::new();

        for step in &self.path {
            let (new_heading, diff_x, diff_y) = Path::step(&heading, step);

            heading = new_heading;

            if diff_x > 0 {
                for x_pos in (pos.0)..(pos.0 + diff_x) {
                    let new_pos = (x_pos, pos.1);

                    if already_visited.contains(&new_pos) {
                        return Some(new_pos.0.abs() as u64 + new_pos.1.abs() as u64);
                    }

                    already_visited.insert(new_pos);
                }

            } else if diff_x < 0 {
                for x_pos in ((pos.0 + diff_x + 1)..(pos.0 + 1)).rev() {
                    let new_pos = (x_pos, pos.1);

                    if already_visited.contains(&new_pos) {
                        return Some(new_pos.0.abs() as u64 + new_pos.1.abs() as u64);
                    }

                    already_visited.insert(new_pos);
                }
            }

            if diff_y > 0 {
                for y_pos in (pos.1)..(pos.1 + diff_y) {
                    let new_pos = (pos.0, y_pos);

                    if already_visited.contains(&new_pos) {
                        return Some(new_pos.0.abs() as u64 + new_pos.1.abs() as u64);
                    }

                    already_visited.insert(new_pos);
                }

            } else if diff_y < 0 {
                for y_pos in ((pos.1 + diff_y + 1)..(pos.1 + 1)).rev() {
                    let new_pos = (pos.0, y_pos);

                    if already_visited.contains(&new_pos) {
                        return Some(new_pos.0.abs() as u64 + new_pos.1.abs() as u64);
                    }

                    already_visited.insert(new_pos);
                }
            }

            pos.0 += diff_x;
            pos.1 += diff_y;
        }

        None
    }
}

fn main() {
    let mut path_str = String::new();
    io::stdin().read_to_string(&mut path_str).expect("Invalid input string!");

    let path = Path::from(path_str.as_str());

    println!("The final distance is: {}", path.get_dist());
    if let Some(dist) = path.get_twice_visited_dist() {
        println!("The distance of the first location visited twice is: {}", dist);
    } else {
        println!("No place was visited twice.");
    }
}


#[cfg(test)]
mod tests {
    use super::Decision;
    use super::Path;

    const INPUT_1: &'static str = "R2, L3";
    const INPUT_2: &'static str = "R2, R2, R2";
    const INPUT_3: &'static str = "R5, L5, R5, R3";

    const INPUT_4: &'static str = "R8, R4, R4, R8";

    #[test]
    fn parse_test() {
        assert_eq!(
            Path::from(INPUT_1).path,
            vec![Decision::Right(2), Decision::Left(3)]);

        assert_eq!(
            Path::from(INPUT_2).path,
            vec![Decision::Right(2), Decision::Right(2), Decision::Right(2)]);

        assert_eq!(
            Path::from(INPUT_3).path,
            vec![Decision::Right(5), Decision::Left(5), Decision::Right(5), Decision::Right(3)]);
    }

    #[test]
    fn dist_test() {
        assert_eq!(Path::from(INPUT_1).get_dist(), 5);

        assert_eq!(Path::from(INPUT_2).get_dist(), 2);

        assert_eq!(Path::from(INPUT_3).get_dist(), 12);
    }

    #[test]
    fn twice_visited() {
        assert_eq!(Path::from(INPUT_4).get_twice_visited_dist(), Some(4));
    }
}