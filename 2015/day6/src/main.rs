
extern crate regex;


use std::collections::VecDeque;

use std::io;
use std::io::prelude::*;

use regex::Regex;

#[derive(Debug)]
struct Range((usize, usize), (usize, usize));

#[derive(Debug)]
enum OperationType {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Operation {
    op_type: OperationType,
    range: Range,
}

fn main() {

    let mut oper_list = VecDeque::new();

    {
        let re = Regex::new("^(turn on|turn off|toggle) ([0-9]+),([0-9]+) through \
                             ([0-9]+),([0-9]+)$")
                     .unwrap();
        let stdin = io::stdin();

        for uline in stdin.lock().lines() {
            let line = &uline.unwrap();

            let capture = re.captures(line).unwrap();

            let start_point = (capture.at(2).unwrap().parse::<usize>().unwrap(),
                               capture.at(3).unwrap().parse::<usize>().unwrap());

            let end_point = (capture.at(4).unwrap().parse::<usize>().unwrap(),
                             capture.at(5).unwrap().parse::<usize>().unwrap());

            let op;
            let op_str = capture.at(1).unwrap();
            let op_range = Range(start_point, end_point);

            match op_str {
                "turn on" => {
                    op = Operation {
                        op_type: OperationType::On,
                        range: op_range,
                    }
                }
                "turn off" => {
                    op = Operation {
                        op_type: OperationType::Off,
                        range: op_range,
                    }
                }
                "toggle" => {
                    op = Operation {
                        op_type: OperationType::Toggle,
                        range: op_range,
                    }
                }
                _ => panic!("Unknown operation: {:?}", op_str),
            }

            oper_list.push_back(op);
        }
    }

    let mut light_matrix = [[false; 1000]; 1000];

    for oper in oper_list.iter() {
        // + 1 because the ranges are inclusive
        for x in ((oper.range.0).0)..((oper.range.1).0 + 1) {

            for y in ((oper.range.0).1)..((oper.range.1).1 + 1) {

                match oper.op_type {
                    OperationType::On => light_matrix[x][y] = true,
                    OperationType::Off => light_matrix[x][y] = false,
                    OperationType::Toggle => light_matrix[x][y] = !light_matrix[x][y],
                }
            }
        }
    }

    let sum = light_matrix.iter().fold(0, |total_acc, &row| {
        total_acc +
        row.iter().fold(0, |acc, &light_state| {
            if light_state {
                acc + 1
            } else {
                acc
            }
        })
    });

    println!("Number of light lit after all operations: {}", sum);



    let mut brt_matrix = [[0 as u32; 1000]; 1000];

    for oper in oper_list.iter() {
        // + 1 because the ranges are inclusive
        for x in ((oper.range.0).0)..((oper.range.1).0 + 1) {

            for y in ((oper.range.0).1)..((oper.range.1).1 + 1) {

                match oper.op_type {
                    OperationType::On => brt_matrix[x][y] += 1,
                    OperationType::Off => {
                        if brt_matrix[x][y] != 0 {
                            brt_matrix[x][y] -= 1
                        }
                    }
                    OperationType::Toggle => brt_matrix[x][y] += 2,
                }
            }
        }
    }

    let brt_sum = brt_matrix.iter().fold(0, |total_acc, &row| {
        total_acc + row.iter().fold(0, |acc, &light_state| acc + light_state)
    });

    println!("Total brightness after all operations: {}", brt_sum);
}
