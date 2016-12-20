extern crate regex;

use regex::Regex;

use std::io;
use std::io::prelude::*;

fn get_decompressed_length(untrimmed_input: &str) -> usize {
    let input = untrimmed_input.trim();

    let marker_re = Regex::new(r"\(([0-9]+)x([0-9]+)\)").unwrap();

    let mut expanded_len = 0;
    let mut pos = 0;

    while pos < input.len() {
        if let Some(marker_cap) = marker_re.captures(&input[pos..]) {
            let num_chars_to_expand = marker_cap.at(1).unwrap().parse::<usize>().unwrap();
            let times_to_expand = marker_cap.at(2).unwrap().parse::<usize>().unwrap();

            let (marker_start_pos, marker_end_pos) = marker_cap.pos(0).unwrap();

            // Add the chars before the marker + the expanded chars.
            expanded_len += marker_start_pos + num_chars_to_expand * times_to_expand;
            pos += marker_end_pos + num_chars_to_expand;

        } else {
            // No remaining markers. Just add the rest of the string to the expanded length.
            expanded_len += input.len() - pos;
            break;
        }
    }

    expanded_len
}

fn get_decompressed_length_recursive(untrimmed_input: &str) -> usize {
    let input = untrimmed_input.trim();

    let marker_re = Regex::new(r"\(([0-9]+)x([0-9]+)\)").unwrap();

    let mut expanded_len = 0;
    let mut pos = 0;

    while pos < input.len() {
        if let Some(marker_cap) = marker_re.captures(&input[pos..]) {
            let num_chars_to_expand = marker_cap.at(1).unwrap().parse::<usize>().unwrap();
            let times_to_expand = marker_cap.at(2).unwrap().parse::<usize>().unwrap();

            let (marker_start_pos, marker_end_pos) = marker_cap.pos(0).unwrap();

            // Add the chars before the marker + the expanded chars.
            // Recursively parse the char sequence to be expanded.
            // !! The start and end positions of the marker are relative to &input[pos..] !!
            expanded_len +=
                marker_start_pos +
                get_decompressed_length_recursive(&input[(pos + marker_end_pos)..(pos + marker_end_pos +
                                                                          num_chars_to_expand)]) *
                times_to_expand;
            pos += marker_end_pos + num_chars_to_expand;

        } else {
            // No remaining markers. Just add the rest of the string to the expanded length.
            expanded_len += input.len() - pos;
            break;
        }
    }

    expanded_len
}



fn main() {
    let mut compressed_str = String::new();
    io::stdin().read_to_string(&mut compressed_str).expect("Invalid input string!");

    println!("The decompressed length of the input string is: {}",
             get_decompressed_length(&compressed_str));
    println!("The recursively decompressed length of the input string is: {}",
             get_decompressed_length_recursive(&compressed_str));
}


#[cfg(test)]
mod tests {
    use super::get_decompressed_length;
    use super::get_decompressed_length_recursive;

    #[test]
    fn decompressed_length_test() {
        let test_input = [("ADVENT", 6),
                          ("A(1x5)BC", 7),
                          ("(3x3)XYZ", 9),
                          ("A(2x2)BCD(2x2)EFG", 11),
                          ("(6x1)(1x3)A", 6),
                          ("X(8x2)(3x3)ABCY", 18)];


        for &(test_str, expected) in test_input.iter() {
            assert_eq!((test_str, get_decompressed_length(test_str)),
                       (test_str, expected));
        }
    }

    #[test]
    fn decompressed_length_recursive_test() {
        let test_input = [("(3x3)XYZ", 9),
                          ("X(8x2)(3x3)ABCY", 20),
                          ("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920),
                          ("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", 445)];

        for &(test_str, expected) in test_input.iter() {
            assert_eq!((test_str, get_decompressed_length_recursive(test_str)),
                       (test_str, expected));
        }
    }
}
