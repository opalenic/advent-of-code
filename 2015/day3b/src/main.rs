#![feature(io)]

use std::collections::HashSet;

use std::io;
use std::io::Read;

fn main() {
    let mut santa_visited = HashSet::new();
    let mut robosanta_visited = HashSet::new();

    let mut santa_curr_loc = (0, 0);
    santa_visited.insert(santa_curr_loc);

    let mut robosanta_curr_loc = (0, 0);
    robosanta_visited.insert(robosanta_curr_loc);


    for (i, ch) in io::stdin().chars().enumerate() {

        let mut loc;
        let mut visited_set;

        if i % 2 == 0 {
            loc = &mut santa_curr_loc;
            visited_set = &mut santa_visited;
        } else {
            loc = &mut robosanta_curr_loc;
            visited_set = &mut robosanta_visited;
        }

        match ch.unwrap() {
            '<' => loc.0 -= 1,
            '>' => loc.0 += 1,
            'v' => loc.1 -= 1,
            '^' => loc.1 += 1,
            other_dir => panic!("Unknown direction: {}", other_dir),
        }

        visited_set.insert(loc.clone());
    }

    println!("Number of unique houses visited by Santa: {}",
             santa_visited.len());
    println!("Number of unique houses visited by Robosanta: {}",
             robosanta_visited.len());
    println!("Total unique houses visited: {}",
             santa_visited.union(&robosanta_visited).count());
}
