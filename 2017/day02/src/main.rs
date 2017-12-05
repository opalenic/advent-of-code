
use std::io;
use std::io::prelude::*;


fn sum_of_differences(input_str: &str) -> usize {
    input_str
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val_str| val_str.parse::<usize>().expect("expected number"))
                .fold((None, None), |(min, max), val| {
                    let new_min = if let Some(v) = min {
                        if val < v { val } else { v }
                    } else {
                        val
                    };

                    let new_max = if let Some(v) = max {
                        if val > v { val } else { v }
                    } else {
                        val
                    };

                    (Some(new_min), Some(new_max))
                })
        })
        .map(|(min_w, max_w)| max_w.unwrap() - min_w.unwrap())
        .sum()
}

fn sum_of_divisible(input_str: &str) -> usize {
    input_str
        .lines()
        .map(|line| {
            let vals = line.split_whitespace().map(|val_str| val_str.parse().expect("expected number")).collect::<Vec<usize>>();

            for (i, divisor) in vals.iter().enumerate() {
                for (j, dividend) in vals.iter().enumerate() {
                    if i != j && dividend % divisor == 0 {
                        return dividend / divisor;
                    }
                }
            }

            panic!("divisible values not found");
        })
        .sum()
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect(
        "cannot read input",
    );

    println!("Sum of differences: {}", sum_of_differences(&input_str));
    println!("Sum of divisible: {}", sum_of_divisible(&input_str));
}


#[cfg(test)]
mod tests {
    use super::sum_of_differences;
    use super::sum_of_divisible;

    #[test]
    fn test_sum_of_differences() {
        let test_input = "5 1 9 5\n\
                          7 5 3\n\
                          2 4 6 8";

        assert_eq!(18, sum_of_differences(&test_input));
    }

    #[test]
    fn test_sum_of_divisible() {
        let test_input = "5 9 2 8\n\
                          9 4 7 3\n\
                          3 8 6 5";

        assert_eq!(9, sum_of_divisible(&test_input));
    }
}
