#![feature(iterator_step_by)]

use std::process::Command;
use std::io;
use std::io::prelude::*;

use std::mem;
use std::u8;

use std::fmt;

use std::collections::HashSet;

const HASHER_PATH: &str = "../day10/target/release/day10";

struct Grid {
    grid: [[bool; 128]; 128],
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.grid.iter() {

            for square in line.iter() {
                f.write_str(if *square { "#" } else { "." })?;
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(init_hash: &str) -> Grid {
        let mut grid: Grid = unsafe { mem::uninitialized() };

        for row_idx in 0..128 {
            let cmd_output = Command::new(HASHER_PATH)
                .arg("-b")
                .arg(format!("{}-{}", init_hash, row_idx))
                .output()
                .expect("failed to run hasher");

            let hash_vec = &cmd_output.stdout[0..32]
                .windows(2)
                .step_by(2)
                .map(|nibbles| {
                    let byte_str = format!("{}{}", nibbles[0] as char, nibbles[1] as char);

                    u8::from_str_radix(&byte_str, 16).expect("invalid hash")
                })
                .collect::<Vec<u8>>();

            for col_idx in 0..128 {
                let byte_pos = col_idx / 8;
                let offset = col_idx % 8;

                grid.grid[row_idx][col_idx] = (hash_vec[byte_pos] & (1 << (7 - offset))) != 0;
            }
        }

        grid
    }

    fn get_used_square_count(&self) -> usize {
        self.grid.iter().fold(0, |total_acc, line| {
            total_acc +
                line.iter().fold(0, |row_acc, square| if *square {
                    row_acc + 1
                } else {
                    row_acc
                })
        })
    }

    fn expand_region(
        &self,
        initial_square: (usize, usize),
        squares_seen: &mut HashSet<(usize, usize)>,
    ) {
        squares_seen.insert(initial_square);
        let mut open_squares = vec![initial_square];

        while let Some(curr_square) = open_squares.pop() {

            // row up
            if curr_square.0 > 0 && self.grid[curr_square.0 - 1][curr_square.1] {
                let square_tuple = (curr_square.0 - 1, curr_square.1);

                if squares_seen.insert(square_tuple) {
                    open_squares.push(square_tuple);
                }
            }

            // col left
            if curr_square.1 > 0 && self.grid[curr_square.0][curr_square.1 - 1] {
                let square_tuple = (curr_square.0, curr_square.1 - 1);

                if squares_seen.insert(square_tuple) {
                    open_squares.push(square_tuple);
                }
            }

            // row down
            if curr_square.0 < self.grid.len() - 1 && self.grid[curr_square.0 + 1][curr_square.1] {
                let square_tuple = (curr_square.0 + 1, curr_square.1);

                if squares_seen.insert(square_tuple) {
                    open_squares.push(square_tuple);
                }
            }

            // col right
            if curr_square.1 < self.grid[curr_square.0].len() - 1 &&
                self.grid[curr_square.0][curr_square.1 + 1]
            {
                let square_tuple = (curr_square.0, curr_square.1 + 1);

                if squares_seen.insert(square_tuple) {
                    open_squares.push(square_tuple);
                }
            }
        }
    }

    fn get_region_count(&self) -> usize {


        let mut region_count = 0;
        let mut squares_seen = HashSet::new();
        for row_idx in 0..self.grid.len() {
            for col_idx in 0..self.grid[row_idx].len() {

                let curr_square_val = self.grid[row_idx][col_idx];
                let curr_square = (row_idx, col_idx);

                if curr_square_val && !squares_seen.contains(&curr_square) {
                    self.expand_region(curr_square, &mut squares_seen);

                    region_count += 1;
                }
            }
        }

        region_count
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let grid = Grid::new(input_str.trim());

    println!(
        "The following grid has {} squares used, and {} contiguous regions.",
        grid.get_used_square_count(),
        grid.get_region_count()
    );
    println!("{}", grid);
}


#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn grid_creation_test() {
        let expected_grid = "##.#.#..\n\
                             .#.#.#.#\n\
                             ....#.#.\n\
                             #.#.##.#\n\
                             .##.#...\n\
                             ##..#..#\n\
                             .#...#..\n\
                             ##.#.##.";

        let grid = Grid::new("flqrgnkx");
        let grid_str = format!("{}", grid);
        let grid_str_split = grid_str.lines().collect::<Vec<&str>>();

        for (i, line) in expected_grid.lines().enumerate() {
            assert_eq!(line, &grid_str_split[i][0..line.len()]);
        }
    }

    #[test]
    fn grid_count_test() {
        let grid = Grid::new("flqrgnkx");

        assert_eq!(8108, grid.get_used_square_count());
    }

    #[test]
    fn grid_region_count_test() {
        let grid = Grid::new("flqrgnkx");

        assert_eq!(1242, grid.get_region_count());
    }
}
