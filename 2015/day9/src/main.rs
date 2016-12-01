
extern crate regex;
extern crate permutohedron;

use regex::Regex;

use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use permutohedron::Heap;

fn main() {

    let mut distances = HashMap::new();

    let re = Regex::new("^(?P<from>[:alpha:]+) to (?P<to>[:alpha:]+) = (?P<dist>[:digit:]+)$").unwrap();

    let stdin = io::stdin();

    for wline in stdin.lock().lines() {
        let line = wline.unwrap();

        let cap = re.captures(&line).unwrap();

        let from = cap.name("from").unwrap().to_string();
        let to = cap.name("to").unwrap().to_string();
        let dist = cap.name("dist").unwrap().parse::<usize>().unwrap();

        {
            let from_dist = distances.entry(from.clone()).or_insert(HashMap::new());
            from_dist.insert(to.clone(), dist);
        }

        {
            let to_dist = distances.entry(to).or_insert(HashMap::new());
            to_dist.insert(from, dist);
        }
    }

    let mut cities: Vec<String> = distances.keys().cloned().collect();

    let path_permutations = Heap::new(&mut cities);

    let mut max = 0;
    let mut min = std::usize::MAX;

    for path in path_permutations {

        let mut dist = 0;

        for city_pair in path.windows(2) {
            let from = &city_pair[0];
            let to = &city_pair[1];

            dist += distances[from][to];
        }

        if dist > max {
            println!("MAX - {} - {:?}", dist, path);
            max = dist;
        }

        if dist < min {
            println!("MIN - {} - {:?}", dist, path);
            min = dist;
        }
    }
}
