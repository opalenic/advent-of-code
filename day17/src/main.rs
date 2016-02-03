use std::io;
use std::io::prelude::*;


#[derive(Debug)]
struct Combination<'a> {
    containers: &'a Vec<u32>,
    current: Vec<bool>,
    expected_sum: u32,
}

impl<'a> Combination<'a> {
    fn new(containers: &'a Vec<u32>, expected_sum: u32) -> Combination {
        Combination {
            containers: containers,
            current: vec![false; containers.len()],
            expected_sum: expected_sum,
        }
    }

    fn next(&mut self) -> bool {

        let mut carry = true;

        for pos in (0..self.current.len()).rev() {
            if carry {
                self.current[pos] ^= carry;
                carry = !self.current[pos];
            }
        }

        !carry
    }

    fn sum_valid(&self) -> bool {
        let sum = self.current.iter().enumerate().fold(0, |acc, (i, &is_contained)| {
            if is_contained {
                acc + self.containers[i]
            } else {
                acc
            }
        });

        sum == self.expected_sum
    }

    fn get_num_containers(&self) -> usize {
        self.current.iter().filter(|&val| *val).count()
    }
}

fn get_num_valid_combinations(combination: &mut Combination) -> u32 {
    let mut total = 0;

    loop {
        if combination.sum_valid() {
            total += 1;
        }

        if !combination.next() {
            break;
        }
    }

    return total;
}

fn get_min_num_containers(combination: &mut Combination) -> (usize, u32) {
    let mut min = combination.containers.len();
    let mut min_count = 0;

    loop {
        if combination.sum_valid() {
            let num_count = combination.get_num_containers();

            if num_count == min {
                min_count += 1;
            }

            if num_count < min {
                min = num_count;
                min_count = 1;
            }
        }

        if !combination.next() {
            break;
        }
    }

    (min, min_count)
}


fn main() {

    let mut containers = Vec::new();

    let stdin = io::stdin();
    for wline in stdin.lock().lines() {
        let line = wline.unwrap();

        containers.push(line.parse::<u32>().unwrap());
    }

    let total = 150;

    let mut c = Combination::new(&containers, total);

    let num_valid = get_num_valid_combinations(&mut c);
    println!("The required total is {} liters", total);
    println!("Total number of valid combinations: {}", num_valid);

    let (min_containers, num_min) = get_min_num_containers(&mut c);
    println!("The minimum number of containers is: {}", min_containers);
    println!("The number of these combinations is: {}", num_min);
}


#[cfg(test)]
mod tests {
    use super::Combination;
    use super::get_num_valid_combinations;
    use super::get_min_num_containers;

    #[test]
    fn combination_test() {
        let containers = vec![20, 15, 10, 5, 5];

        let mut comb = Combination::new(&containers, 25);

        assert_eq!(4, get_num_valid_combinations(&mut comb));
    }

    #[test]
    fn min_container_test() {
        let containers = vec![20, 15, 10, 5, 5];

        let mut comb = Combination::new(&containers, 25);

        assert_eq!((2, 3), get_min_num_containers(&mut comb));
    }
}
