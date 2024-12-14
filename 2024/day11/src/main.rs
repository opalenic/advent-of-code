use std::collections::HashMap;

use rayon::prelude::*;

fn split_even_digit_number(n: u64) -> Option<(u64, u64)> {
    let n_as_str = n.to_string();
    let len = n_as_str.len();
    if len % 2 == 0 {
        let half = len / 2;
        let first = n_as_str[..half].parse().unwrap();
        let second = n_as_str[half..].parse().unwrap();
        Some((first, second))
    } else {
        None
    }
}

enum Number {
    Single(u64),
    Double(u64, u64),
}

fn expand_number(n: u64) -> Number {
    if n == 0 {
        Number::Single(1)
    } else if let Some((first, second)) = split_even_digit_number(n) {
        Number::Double(first, second)
    } else {
        Number::Single(n * 2024)
    }
}

fn run_puzzle(num_iterations: usize, input: Vec<u64>) -> Vec<u64> {
    let mut input = input;

    for i in 0..num_iterations {
        println!("Running iteration {}", i);

        let mut output = Vec::new();
        
        for num in &input {
            match expand_number(*num) {
                Number::Single(n) => output.push(n),
                Number::Double(first, second) => {
                    output.push(first);
                    output.push(second);
                }
            }
        }
        
        input = output;
    }

    input
}

fn run_copilots_solution(num_iterations: usize, input: Vec<u64>) -> Vec<u64> {
    let mut stones = input.iter().map(|&n| n.to_string()).collect::<Vec<String>>();

    let mut memo = HashMap::new();

    for _ in 0..num_iterations {
        stones = transform_stones(&stones, &mut memo);
    }

    stones.iter().map(|s| s.parse().unwrap()).collect()
}


fn transform_stones(stones: &[String], memo: &mut HashMap<Vec<String>, Vec<String>>) -> Vec<String> {
    if let Some(cached) = memo.get(stones) {
        return cached.clone();
    }

    let mut new_stones = Vec::new();

    for stone in stones {
        let transformed = transform_stone(stone);
        new_stones.extend(transformed);
    }

    memo.insert(stones.to_vec(), new_stones.clone());
    new_stones
}

fn transform_stone(stone: &String) -> Vec<String> {
    let num: u32 = stone.parse().unwrap();

    if num == 0 {
        return vec!["1".to_string()];
    }

    let digits = stone.chars().count();

    if digits % 2 == 0 {
        let mid = digits / 2;
        let left = &stone[..mid];
        let right = &stone[mid..];
        return vec![left.to_string(), right.to_string()];
    }

    let new_value = num * 2024;
    return vec![new_value.to_string()];
}




fn main() {
    let input = vec![27, 10647, 103, 9, 0, 5524, 4594227, 902936];
    let result = run_puzzle(25, input);
    println!("Part 1: I have {} stones.", result.len());

    let input = vec![27, 10647, 103, 9, 0, 5524, 4594227, 902936];
    let result = run_puzzle(75, input);
    println!("Part 2: I have {} stones.", result.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input() {
        let input = vec![125, 17];

        let result = super::run_puzzle(6, input);

        assert_eq!(
            result,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }

    #[test]
    fn test_copilots_solution() {
        let input = vec![125, 17];

        let result = super::run_copilots_solution(6, input);

        assert_eq!(
            result,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );

    }
}
