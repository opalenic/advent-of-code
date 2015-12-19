#![feature(iter_arith)]

use std::io;
use std::io::prelude::*;


fn main() {
    let mut total_area = 0;
    let mut total_ribbon = 0;


    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let dimensions = line.unwrap()
                             .split("x")
                             .map(|i| i.parse::<u32>().unwrap())
                             .collect::<Vec<u32>>();

        let faces = [dimensions[0] * dimensions[1],
                     dimensions[0] * dimensions[2],
                     dimensions[1] * dimensions[2]];

        let perimeters = [2 * (dimensions[0] + dimensions[1]),
                          2 * (dimensions[0] + dimensions[2]),
                          2 * (dimensions[1] + dimensions[2])];

        let volume = dimensions[0] * dimensions[1] * dimensions[2];

        total_area += 2 * faces.iter().sum::<u32>() + faces.iter().min().unwrap();
        total_ribbon += perimeters.iter().min().unwrap() + volume;
    }

    println!("The total area needed (in ft2) is: {}", total_area);
    println!("The total length of ribbon needed (in ft) is: {}",
             total_ribbon);
}
