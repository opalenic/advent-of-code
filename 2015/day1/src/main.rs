#![feature(io)]

use std::io;
use std::io::Read;

fn main() {
    let mut cnt = 0;

    let mut pos_in_input = 1;
    let mut entered_basement_at_pos = -1;

    for wrapped_ch in io::stdin().chars() {

        let ch = wrapped_ch.unwrap();

        if ch == '(' {
            cnt += 1;
        } else if ch == ')' {
            cnt -= 1;
        }

        if cnt == -1 && entered_basement_at_pos == -1 {
            entered_basement_at_pos = pos_in_input;
        }

        pos_in_input += 1;
    }

    println!("The final floor number is: {}", cnt);
    println!("First entered basement at pos: {}", entered_basement_at_pos);
}
