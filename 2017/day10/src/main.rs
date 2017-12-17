#![feature(iterator_step_by)]

use std::io;
use std::io::prelude::*;


fn create_hash(
    hash_lenght: usize,
    num_hash_rounds: usize,
    skip_lenghts: &Vec<usize>,
) -> Vec<usize> {

    let mut out: Vec<usize> = (0..hash_lenght).collect();
    let mut curr_pos = 0;
    let mut skip_len = 0;

    for _ in 0..num_hash_rounds {
        for in_skip_len in skip_lenghts {

            let end_pos = curr_pos + in_skip_len;

            let tmp_vec = if end_pos <= out.len() {
                out[curr_pos..end_pos].to_vec()
            } else {
                let mut tmp = out[curr_pos..].to_vec();

                tmp.extend(out[0..(end_pos % out.len())].to_vec());

                tmp
            };

            for (i, val) in tmp_vec.into_iter().rev().enumerate() {
                let l = out.len();
                out[(curr_pos + i) % l] = val;
            }

            curr_pos = (end_pos + skip_len) % out.len();
            skip_len += 1;
        }
    }

    out
}


fn create_full_hash(input_str: &str) -> String {
    let mut skip_lenghts: Vec<usize> = input_str.chars().map(|ch| ch as usize).collect();
    skip_lenghts.extend(vec![17, 31, 73, 47, 23]);



    let sparse_hash = create_hash(256, 64, &skip_lenghts);

    sparse_hash
        .windows(16)
        .step_by(16)
        .map(|window| {
            format!(
                "{:02x}",
                window.into_iter().fold(0, |acc, ch| acc ^ (*ch as u8))
            )
        })
        .collect()
}


fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let inputs = input_str
        .trim()
        .split(",")
        .map(|len_str| len_str.parse())
        .collect::<Result<Vec<usize>, _>>()
        .expect("parsing error");

    let hash = create_hash(256, 1, &inputs);
    println!("Part A: {}", hash[0] * hash[1]);

    println!("Part B: {}", create_full_hash(&input_str.trim()));
}

#[cfg(test)]
mod tests {
    use super::create_hash;
    use super::create_full_hash;

    #[test]
    fn create_hash_test() {
        assert_eq!(vec![3, 4, 2, 1, 0], create_hash(5, 1, &vec![3, 4, 1, 5]));
    }

    #[test]
    fn full_hash_test() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", create_full_hash(""));
        assert_eq!(
            "33efeb34ea91902bb2f59c9920caa6cd",
            create_full_hash("AoC 2017")
        );
        assert_eq!(
            "3efbe78a8d82f29979031a4aa0b16a9d",
            create_full_hash("1,2,3")
        );
        assert_eq!(
            "63960835bcdc130f0b66d7ff4f6a5a8e",
            create_full_hash("1,2,4")
        );
    }
}
