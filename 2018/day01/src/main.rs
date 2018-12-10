use std::num::ParseIntError;
use std::str::FromStr;

use std::collections::HashSet;

use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct AocError(String);

#[derive(Debug)]
struct Frequencies(Vec<isize>);

impl FromStr for Frequencies {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Frequencies(
            s.lines()
                .map(|line| {
                    line.parse()
                        .map_err(|e: ParseIntError| AocError(format!("Parsing error: {}", e)))
                })
                .collect::<Result<Vec<isize>, AocError>>()?,
        ))
    }
}

impl Frequencies {
    fn get_sum(&self) -> isize {
        self.0.iter().sum()
    }

    fn get_first_seen_twice(&self) -> isize {
        let mut already_seen = HashSet::new();
        already_seen.insert(0);

        let mut sum = 0;

        loop {
            for val in &self.0 {
                sum += val;

                if !already_seen.insert(sum) {
                    return sum;
                }
            }
        }
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let frequencies: Frequencies = input_str.parse().expect("parse error");

    println!("The sum of all frequencies is: {}", frequencies.get_sum());
    println!("The first frequency seen twice is: {}", frequencies.get_first_seen_twice());
}


#[cfg(test)]
mod tests {
    use super::Frequencies;

    const TEST_PAIRS_A: [(&'static str, isize); 4] = [
        ("+1\n-2\n+3\n+1", 3),
        ("+1\n+1\n+1", 3),
        ("+1\n+1\n-2", 0),
        ("-1\n-2\n-3", -6)
    ];

    const TEST_PAIRS_B: [(&'static str, isize); 5] = [
        ("+1\n-2\n+3\n+1", 2),
        ("+1\n-1", 0),
        ("+3\n+3\n+4\n-2\n-4", 10),
        ("-6\n+3\n+8\n+5\n-6", 5),
        ("+7\n+7\n-2\n-7\n-4", 14)
    ];

    #[test]
    fn part_a_test() {
        for (input_str, expected_sum) in &TEST_PAIRS_A {
            let frequencies: Frequencies = input_str.parse().unwrap();
            assert_eq!(expected_sum, &frequencies.get_sum());
        }
    }

    #[test]
    fn part_b_test() {
        for (input_str, expected_seen_twice) in &TEST_PAIRS_B {
            let frequencies: Frequencies = input_str.parse().unwrap();
            assert_eq!(expected_seen_twice, &frequencies.get_first_seen_twice());
        }
    }
}
