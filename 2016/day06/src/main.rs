#![feature(iter_max_by, iter_min_by)]

use std::collections::HashMap;

use std::io;
use std::io::prelude::*;

enum DecodingMethod {
    MostLikely,
    LeastLikely,
}

fn decode_message(input: &str, method: DecodingMethod) -> String {
    let len = input.lines().map(|line| line.chars().count()).max().unwrap();

    let mut histograms = vec![HashMap::new(); len];

    for line in input.lines() {
        for (pos, ch) in line.chars().enumerate() {
            let cnt = histograms[pos].entry(ch).or_insert(0);
            *cnt += 1;
        }
    }

    histograms.into_iter()
        .map(|h| {
            match method {
                    DecodingMethod::MostLikely => {
                        h.into_iter().max_by(|&(_, cnt_a), &(_, cnt_b)| cnt_a.cmp(&cnt_b))
                    }
                    DecodingMethod::LeastLikely => {
                        h.into_iter().min_by(|&(_, cnt_a), &(_, cnt_b)| cnt_a.cmp(&cnt_b))
                    }
                }
                .unwrap()
                .0
        })
        .collect()
}

fn main() {
    let mut message_str = String::new();
    io::stdin().read_to_string(&mut message_str).expect("Invalid input string!");

    println!("The initial decoded message is: {}",
             decode_message(&message_str, DecodingMethod::MostLikely));
    println!("The actual decoded message is: {}",
             decode_message(&message_str, DecodingMethod::LeastLikely));
}


#[cfg(test)]
mod tests {
    use super::decode_message;
    use super::DecodingMethod;

    const TEST_INPUT: &'static str = "eedadn\n\
                                      drvtee\n\
                                      eandsr\n\
                                      raavrd\n\
                                      atevrs\n\
                                      tsrnev\n\
                                      sdttsa\n\
                                      rasrtv\n\
                                      nssdts\n\
                                      ntnada\n\
                                      svetve\n\
                                      tesnvt\n\
                                      vntsnd\n\
                                      vrdear\n\
                                      dvrsen\n\
                                      enarar";

    #[test]
    fn decode_message_test() {
        assert_eq!(decode_message(TEST_INPUT, DecodingMethod::MostLikely),
                   "easter");
    }

    #[test]
    fn decode_message_b_test() {
        assert_eq!(decode_message(TEST_INPUT, DecodingMethod::LeastLikely),
                   "advent");
    }
}
