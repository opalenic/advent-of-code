
extern crate regex;


use std::collections::VecDeque;

use std::io;
use std::io::prelude::*;

use regex::Regex;

struct Range((u32, u32), (u32, u32));

enum Operation {
    On(Range),
    Off(Range),
    Toggle(Range)
}

fn main() {

    let mut oper_list = VecDeque::new();

    {
        let re = Regex::new("^(turn on|turn off|toggle) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)$").unwrap();
        let stdin = io::stdin();

        for uline in stdin.lock().lines() {
            let line = &uline.unwrap();

            let capture = re.captures(line).unwrap();

            let start_point = (capture.at(2).unwrap().parse::<u32>().unwrap(), 
                               capture.at(3).unwrap().parse::<u32>().unwrap());

            let end_point = (capture.at(4).unwrap().parse::<u32>().unwrap(), 
                             capture.at(5).unwrap().parse::<u32>().unwrap());

            let op;
            let op_str = capture.at(1).unwrap();

            match op_str {
                "turn on" => op = Operation::On(Range(start_point, end_point)),
                "turn off" => op = Operation::Off(Range(start_point, end_point)),
                "toggle" => op = Operation::Toggle(Range(start_point, end_point)),
                _ => panic!("Unknown operation: {:?}", op_str)
            }

            oper_list.push_back(op);
        }
    }

}
