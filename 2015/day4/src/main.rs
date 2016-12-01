
extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {

    let puzzle_input = "iwrupvqb";

    let mut curr_num = 1;


    loop {
        let mut sum = Md5::new();

        sum.puzzle_input(&(puzzle_input.to_string() + &curr_num.to_string()));

        if &sum.result_str()[0..6] == "000000" {
            println!("Found the long hash: {} {}", curr_num, sum.result_str());
            break;
        } else if &sum.result_str()[0..5] == "00000" {
            println!("Found the short hash: {} {}", curr_num, sum.result_str());
        }

        curr_num += 1;
    }
}
