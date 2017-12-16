
use std::io;
use std::io::prelude::*;

fn score_groups(test_str: &str) -> (usize, usize) {
    let mut ignore_next = false;
    let mut group_depth = 0;
    let mut score = 0;
    let mut in_garbage = false;
    let mut garbage_count = 0;

    for ch in test_str.chars() {
        if ignore_next {
            ignore_next = false;

            continue;
        }

        if in_garbage {
            match ch {
                '>' => in_garbage = false,
                '!' => ignore_next = true,
                _ => garbage_count += 1
            }
        } else {
            match ch {
                '{' => group_depth += 1,
                '}' => {
                    score += group_depth;
                    group_depth -= 1;
                }
                '<' => in_garbage = true,
                _ => {}
            }
        }
    }

    (score, garbage_count)
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let (score, garbage_count) = score_groups(&input_str);

    println!(
        "The score of the input data is {}. The garbage count is {}",
        score,
        garbage_count
    );
}


#[cfg(test)]
mod tests {
    use super::score_groups;

    #[test]
    fn group_score_test() {
        let test_input = vec![
            ("{}", 1),
            ("{{{}}}", 6),
            ("{{},{}}", 5),
            ("{{{},{},{{}}}}", 16),
            ("{<{},{},{{}}>}", 1),
            ("{<a>,<a>,<a>,<a>}", 1),
            ("{{<a>},{<a>},{<a>},{<a>}}", 9),
            ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
            ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
            ("{{<!>},{<!>},{<!>},{<a>}}", 3),
            ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
        ];

        for &(test_str, expected_score) in &test_input {
            assert_eq!(expected_score, score_groups(test_str).0);
        }
    }

    #[test]
    fn count_garbage_test() {
        let test_input = vec![
            ("<>", 0),
            ("<random characters>", 17),
            ("<<<<>", 3),
            ("<{!>}>", 2),
            ("<!!>", 0),
            ("<!!!>>", 0),
            ("<{o\"i!a,<{i<a>", 10),
        ];

        for &(test_str, expected_garbage_count) in &test_input {
            assert_eq!(expected_garbage_count, score_groups(test_str).1);
        }

    }
}
