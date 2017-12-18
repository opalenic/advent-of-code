
use std::io;
use std::io::prelude::*;

use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(Direction::North),
            "ne" => Ok(Direction::NorthEast),
            "se" => Ok(Direction::SouthEast),
            "s" => Ok(Direction::South),
            "sw" => Ok(Direction::SouthWest),
            "nw" => Ok(Direction::NorthWest),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Position(isize, isize, isize);

impl Position {
    fn new() -> Position {
        Position(0, 0, 0)
    }

    fn do_move(&self, dir: Direction) -> Position {
        match dir {
            Direction::North => Position(self.0, self.1 + 1, self.2 - 1),
            Direction::NorthEast => Position(self.0 + 1, self.1, self.2 - 1),
            Direction::SouthEast => Position(self.0 + 1, self.1 - 1, self.2),
            Direction::South => Position(self.0, self.1 - 1, self.2 + 1),
            Direction::SouthWest => Position(self.0 - 1, self.1, self.2 + 1),
            Direction::NorthWest => Position(self.0 - 1, self.1 + 1, self.2),
        }
    }

    fn distance_to(&self, other_pos: &Position) -> usize {
        ((self.0 - other_pos.0).abs() as usize + (self.1 - other_pos.1).abs() as usize +
             (self.2 - other_pos.2).abs() as usize) / 2
    }
}

struct Path(Vec<Direction>);

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Path(s.split(",")
            .map(|dir_str| dir_str.parse())
            .collect::<Result<Vec<Direction>, ()>>()?))
    }
}

impl Path {
    fn walk_and_remember(&self, start_pos: &Position) -> (Position, usize) {
        let mut pos = *start_pos;
        let mut max_dist = 0;

        for dir in &self.0 {
            pos = pos.do_move(*dir);

            let dist = start_pos.distance_to(&pos);
            if dist > max_dist {
                max_dist = dist;
            }
        }

        (pos, max_dist)
    }
}


fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let path: Path = input_str.trim().parse().expect("parse error");
    let start_pos = Position::new();
    let (end_pos, max_dist) = path.walk_and_remember(&start_pos);

    println!(
        "The distance from {:?} to {:?} is {}. The maximum distance from {:?} was {}.",
        start_pos,
        end_pos,
        start_pos.distance_to(&end_pos),
        start_pos,
        max_dist
    );
}


#[cfg(test)]
mod tests {
    use super::Position;
    use super::Path;

    const TEST_INPUT: [(&str, usize); 4] = [
        ("ne,ne,ne", 3),
        ("ne,ne,sw,sw", 0),
        ("ne,ne,s,s", 2),
        ("se,sw,se,sw,sw", 3),
    ];

    #[test]
    fn distance_test() {
        for &(input_str, expected_dist) in &TEST_INPUT {
            let path: Path = input_str.parse().expect("parse error");

            let start_pos = Position::new();
            let (end_pos, _) = path.walk_and_remember(&start_pos);

            assert_eq!(expected_dist, start_pos.distance_to(&end_pos));
        }
    }
}
