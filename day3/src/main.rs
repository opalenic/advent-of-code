#![feature(io)]

use std::collections::HashSet;

use std::io;
use std::io::Read;

fn main() {
    let mut visited = HashSet::new();

    let mut curr_loc = (0, 0);
    visited.insert(curr_loc);

    for ch in io::stdin().chars() {

        match ch.unwrap() {
            '<' => curr_loc.0 -= 1,
            '>' => curr_loc.0 += 1,
            'v' => curr_loc.1 -= 1,
            '^' => curr_loc.1 += 1,
            other_dir => panic!("Unknown direction: {}", other_dir),
        }

        visited.insert(curr_loc);
    }

    println!("Number of unique houses: {}", visited.len());
}
