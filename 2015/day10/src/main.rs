extern crate itertools;

use std::env::args;

use itertools::Itertools;

fn main() {

    let mut a = args();

    a.next(); // The first argument is the binary name/path

    let start = a.next().unwrap().to_string(); // The puzzle input
    let num_iterations = a.next().unwrap().parse::<u32>().unwrap(); // The number of iterations


    let mut current = start.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();

    for i in 0..num_iterations {
        if i % 5 == 0 {
            println!("{} - {}", i, current.len());
        }

        current = current.into_iter()
                         .group_by(|&ch| ch)
                         .flat_map(|(key, matched)| vec![matched.len() as u8, key])
                         .collect();

    }


    println!("The length of the string after {} iterations is: {}",
             num_iterations,
             current.len());
}
