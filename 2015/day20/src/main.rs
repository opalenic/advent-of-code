#![feature(iter_arith)]

use std::iter::Iterator;

use std::env;


#[derive(Debug)]
struct Divisors {
    num_divided: u64,
    divisor_complement: Option<u64>,
    pos: u64,
    max: u64,
}

impl Divisors {
    fn new(num_divided: u64) -> Divisors {
        Divisors {
            num_divided: num_divided,
            divisor_complement: None,
            pos: 0,
            max: (num_divided as f64).sqrt() as u64,
        }
    }
}

impl Iterator for Divisors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(last) = self.divisor_complement {

            self.divisor_complement = None;

            let div = self.num_divided / last;

            if div != last {
                return Some(div);
            }
        }


        while self.pos < self.max {
            self.pos += 1;

            if self.num_divided % self.pos == 0 {
                self.divisor_complement = Some(self.pos);

                return Some(self.pos);
            }
        }

        None
    }
}


fn part1(num_presents_min: u64, guess_start: bool) -> (u64, u64) {

    let mut current_sum = 0;

    let mut i = if guess_start {
        num_presents_min / 50
    } else {
        1
    };

    while current_sum < num_presents_min {
        current_sum = Divisors::new(i).sum::<u64>() * 10;
        i += 1;
    }

    (i - 1, current_sum)
}


fn part2(num_presents_min: u64, guess_start: bool) -> (u64, u64) {

    let mut current_sum = 0;

    let mut i = if guess_start {
        num_presents_min / 50
    } else {
        1
    };

    while current_sum < num_presents_min {
        current_sum = Divisors::new(i)
                          .filter(|elf_num| i / elf_num <= 50)
                          .sum::<u64>() * 11;
        i += 1;
    }

    (i - 1, current_sum)
}


fn main() {
    let mut args = env::args();


    args.next();

    let input = args.next().unwrap().parse::<u64>().unwrap();
    let should_guess = if let Some(guess_str) = args.next() {
        match guess_str.as_ref() {
            "guess" => true,
            _ => false,
        }
    } else {
        false
    };

    if should_guess {
        println!("Trying to reduce the search time by guessing at which house to start searching.");
    }

    let (part_1_house, part_1_num_presents) = part1(input, should_guess);
    println!("Part 1. House #{} is the first to receive at least {} presents. ({})",
             part_1_house,
             input,
             part_1_num_presents);


    let (part_2_house, part_2_num_presents) = part2(input, should_guess);
    println!("Part 2. House #{} is the first to receive at least {} presents. ({})",
             part_2_house,
             input,
             part_2_num_presents);
}


#[cfg(test)]
mod tests {
    use super::Divisors;
    use super::part1;

    const NUM_ITER_TESTS: u64 = 1000;

    const HOUSE_PRESENTS: [u64; 9] = [10, 30, 40, 70, 60, 120, 80, 150, 130];
    const MAX_PRESENTS: u64 = 150;

    #[test]
    fn divisors_iter_test() {
        for i in 1..NUM_ITER_TESTS {
            let mut divisors = Divisors::new(i).collect::<Vec<u64>>();
            divisors.sort();

            let expected = (1..(i + 1)).filter(|num| i % num == 0).collect::<Vec<u64>>();

            assert_eq!(divisors, expected);
        }
    }

    #[test]
    fn house_presents_test() {
        for input_presents in 1..MAX_PRESENTS {
            let mut ex_house_num = 0;
            let mut ex_num_presents = 0;

            for house_num in 0..HOUSE_PRESENTS.len() {
                if HOUSE_PRESENTS[house_num] >= input_presents {
                    ex_house_num = house_num as u64 + 1;
                    ex_num_presents = HOUSE_PRESENTS[house_num];

                    break;
                }
            }

            let (house, num_presents) = part1(input_presents, false);

            assert_eq!(house, ex_house_num);
            assert_eq!(num_presents, ex_num_presents);


            let (house_w_guess, num_presents_w_guess) = part1(num_presents, true);

            assert_eq!(house_w_guess, ex_house_num);
            assert_eq!(num_presents_w_guess, ex_num_presents);
        }
    }
}
