
#[macro_use]
extern crate itertools;


use std::str::FromStr;
use std::num::ParseIntError;

use std::io;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Shape {
    sides: Vec<u32>,
}

impl FromStr for Shape {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sides = try!(s.split_whitespace()
            .map(|side| side.parse::<u32>())
            .collect());

        Ok(Shape { sides: sides })
    }
}

impl<'a> From<(u32, u32, u32)> for Shape {
    fn from(sides_tuple: (u32, u32, u32)) -> Shape {
        Shape { sides: vec![sides_tuple.0, sides_tuple.1, sides_tuple.2] }
    }
}

impl Shape {
    fn is_valid_triangle(&self) -> bool {
        if self.sides.len() != 3 {
            return false;
        }

        (self.sides[0] + self.sides[1] > self.sides[2]) &&
        (self.sides[0] + self.sides[2] > self.sides[1]) &&
        (self.sides[1] + self.sides[2] > self.sides[0])
    }
}

fn parse_input(input: &str) -> Result<Vec<Shape>, ParseIntError> {
    Ok(try!(input.lines().map(|line| line.parse()).collect()))
}

fn parse_input_vertical(input: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();

    let mut lines = input.lines().peekable();

    while lines.peek().is_some() {
        let mut three_lines = Vec::with_capacity(3);
        for _ in 0..3 {
            three_lines.push(lines.next()
                .unwrap()
                .split_whitespace()
                .map(|side| side.parse::<u32>().unwrap())
                .collect::<Vec<u32>>());
        }

        let transposed = izip!(three_lines[0].iter().cloned(),
                               three_lines[1].iter().cloned(),
                               three_lines[2].iter().cloned())
            .collect::<Vec<(u32, u32, u32)>>();

        for i in 0..3 {
            shapes.push(transposed[i].into());
        }
    }

    shapes
}


fn main() {
    let mut shape_str = String::new();
    io::stdin().read_to_string(&mut shape_str).expect("Invalid input string!");

    let shapes = parse_input(&shape_str).unwrap();
    let num_valid_triangles = shapes.iter().fold(0, |acc, shape| if shape.is_valid_triangle() {
        acc + 1
    } else {
        acc
    });

    println!("A: The number of valid triangles is: {}",
             num_valid_triangles);

    let shapes_b = parse_input_vertical(&shape_str);
    let num_valid_triangles_b = shapes_b.iter().fold(0, |acc, shape| if shape.is_valid_triangle() {
        acc + 1
    } else {
        acc
    });

    println!("B: The number of valid triangles is: {}",
             num_valid_triangles_b);
}


#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::Shape;
    use super::parse_input_vertical;

    const TEST_STRING: &'static str = "566  477  376\n\
                                       575  488  365\n\
                                        50   18  156\n\
                                       558  673  498\n\
                                       133  112  510\n\
                                       670  613   25\n";

    const TEST_STRING_2: &'static str = "101 301 501\n\
                                         102 302 502\n\
                                         103 303 503\n\
                                         201 401 601\n\
                                         202 402 602\n\
                                         203 403 603\n";

    #[test]
    fn parse_test() {
        assert_eq!(parse_input(TEST_STRING),
                   Ok(vec![Shape { sides: vec![566, 477, 376] },
                           Shape { sides: vec![575, 488, 365] },
                           Shape { sides: vec![50, 18, 156] },
                           Shape { sides: vec![558, 673, 498] },
                           Shape { sides: vec![133, 112, 510] },
                           Shape { sides: vec![670, 613, 25] }]));
    }

    #[test]
    fn valid_triangle_test() {
        let shapes = parse_input(TEST_STRING).unwrap();

        assert_eq!(shapes.iter()
                       .map(|shape| shape.is_valid_triangle())
                       .collect::<Vec<bool>>(),
                   vec![true, true, false, true, false, false]);
    }

    #[test]
    fn vertical_shape_test() {
        let shapes = parse_input_vertical(TEST_STRING_2);

        assert_eq!(shapes,
                   vec![Shape { sides: vec![101, 102, 103] },
                        Shape { sides: vec![301, 302, 303] },
                        Shape { sides: vec![501, 502, 503] },
                        Shape { sides: vec![201, 202, 203] },
                        Shape { sides: vec![401, 402, 403] },
                        Shape { sides: vec![601, 602, 603] }]);
    }
}
