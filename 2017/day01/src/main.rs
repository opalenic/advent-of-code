extern crate itertools;

use itertools::Itertools;

use std::io;
use std::io::prelude::*;

fn get_captcha(input: &Vec<u8>) -> usize {
    if input.len() >= 2 {
        let mut sum = input
            .iter()
            .group_by(|digit| *digit)
            .into_iter()
            .map(|(key, vals)| (*key as usize) * (vals.count() - 1))
            .sum();

        if input.first().unwrap() == input.last().unwrap() {
            sum += *input.first().unwrap() as usize;
        }

        sum

    } else {
        0
    }
}

fn get_captcha_2(input: &Vec<u8>) -> usize {
    let mut sum = 0;
    for (i, digit) in input.iter().enumerate() {
        let check_pos = (i + input.len() / 2) % input.len();

        if input[check_pos] == *digit {
            sum += *digit as usize;
        }
    }

    sum
}

fn main() {

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect(
        "cannot read input",
    );

    let input = input_str
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .map(|d| d as u8)
        .collect();

    println!("Captcha 1: {}", get_captcha(&input));
    println!("Captcha 2: {}", get_captcha_2(&input));
}

#[cfg(test)]
mod tests {
    use super::get_captcha;
    use super::get_captcha_2;

    #[test]
    fn captcha_test() {
        let test_inputs = vec![
            vec![1, 1, 2, 2],
            vec![1, 1, 1, 1],
            vec![1, 2, 3, 4],
            vec![9, 1, 2, 1, 2, 1, 2, 9],
        ];

        assert_eq!(
            vec![3, 4, 0, 9],
            test_inputs
                .into_iter()
                .map(|input| get_captcha(&input))
                .collect::<Vec<usize>>()
        );
    }

    #[test]
    fn captcha_test_2() {
        let test_inputs = vec![
            vec![1, 2, 1, 2],
            vec![1, 2, 2, 1],
            vec![1, 2, 3, 4, 2, 5],
            vec![1, 2, 3, 1, 2, 3],
            vec![1, 2, 1, 3, 1, 4, 1, 5],
        ];

        assert_eq!(
            vec![6, 0, 4, 12, 4],
            test_inputs
                .into_iter()
                .map(|input| get_captcha_2(&input))
                .collect::<Vec<usize>>()
        );
    }
}
