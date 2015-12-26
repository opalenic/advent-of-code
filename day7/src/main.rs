extern crate regex;


use std::io;
use std::io::prelude::*;

use regex::Regex;

use std::collections::HashMap;

type WireName = String;

trait ResolvedInputs {
    fn has_resolved_inputs(&self, resolved_inputs: &HashMap<WireName, u16>) -> bool;
}

trait ResolveWire {
    fn resolve_wire(&self, other_resolved: &HashMap<WireName, u16>) -> Option<u16>;
}

#[derive(Debug)]
enum OperArg {
    Wire(WireName),
    Val(u16)
}

#[derive(Debug)]
enum Operation {
    And(OperArg, OperArg),
    LShift(OperArg, OperArg),
    Or(OperArg, OperArg),
    RShift(OperArg, OperArg),
    Not(OperArg),
    Alias(OperArg),
}

impl ResolvedInputs for Operation {
    fn has_resolved_inputs(&self, resolved_inputs: &HashMap<WireName, u16>) -> bool {
        match self {
            &Operation::And(ref input1, ref input2) |
            &Operation::LShift(ref input1, ref input2) |
            &Operation::Or(ref input1, ref input2) |
            &Operation::RShift(ref input1, ref input2) => {
                let found_in1 = match input1 {
                    &OperArg::Wire(ref wire_name) => resolved_inputs.contains_key(wire_name),
                    &OperArg::Val(_) => true,
                }; 

                let found_in2 = match input2 {
                    &OperArg::Wire(ref wire_name) => resolved_inputs.contains_key(wire_name),
                    &OperArg::Val(_) => true,
                };

                found_in1 && found_in2
            },
            &Operation::Not(ref input) |
            &Operation::Alias(ref input) => {
                match input {
                    &OperArg::Wire(ref wire_name) => resolved_inputs.contains_key(wire_name),
                    &OperArg::Val(_) => true,
                }
            }
        }

    }
}

impl ResolveWire for Operation {
    fn resolve_wire(&self, other_resolved: &HashMap<WireName, u16>) -> Option<u16> {

        fn get_input(arg: &OperArg, resolved: &HashMap<WireName, u16>) -> Option<u16> {
            match arg {
                &OperArg::Val(v) => Some(v),
                &OperArg::Wire(ref w_name) => match resolved.get(w_name) {
                    Some(w_val) => Some(w_val.clone()),
                    None => None
                }
            }
        }

        match self {
            &Operation::And(ref input1, ref input2) => {
                let i1 = get_input(input1, other_resolved);
                let i2 = get_input(input2, other_resolved);

                if i1.is_some() && i2.is_some() {
                    Some(i1.unwrap() & i2.unwrap())
                } else {
                    None
                }
            },
            &Operation::LShift(ref input1, ref input2) => {
                let i1 = get_input(input1, other_resolved);
                let i2 = get_input(input2, other_resolved);

                if i1.is_some() && i2.is_some() {
                    Some(i1.unwrap() << i2.unwrap())
                } else {
                    None
                }
            },
            &Operation::Or(ref input1, ref input2) => {
                let i1 = get_input(input1, other_resolved);
                let i2 = get_input(input2, other_resolved);

                if i1.is_some() && i2.is_some() {
                    Some(i1.unwrap() | i2.unwrap())
                } else {
                    None
                }
            },
            &Operation::RShift(ref input1, ref input2) => {
                let i1 = get_input(input1, other_resolved);
                let i2 = get_input(input2, other_resolved);

                if i1.is_some() && i2.is_some() {
                    Some(i1.unwrap() >> i2.unwrap())
                } else {
                    None
                }
            },
            &Operation::Not(ref input) => {
                let i = get_input(input, other_resolved);

                if i.is_some() {
                    Some(!i.unwrap())
                } else {
                    None
                }
            },
            &Operation::Alias(ref input) => {
                get_input(input, other_resolved)
            },
        }
    }
}



// 1) Build a has map of wires and operations - (wire name, operation type and inputs as strings)
// 2) Loop through all operations and try to resolve them into number values.
// 3) Move the resolved wires into a resolved wire list.
// 4) Repeat until all wires/operations are resolved.
//

fn main() {

    let mut wire_ops: HashMap<WireName, Operation> = HashMap::new();

    // Build a model of the circuit
    let re = Regex::new("^(?:(?P<in1>[a-z]+|[0-9]+) )?(?:(?P<oper>AND|LSHIFT|OR|RSHIFT|NOT) \
                         )?(?P<in2>[a-z]+|[0-9]+)? -> (?P<out>[a-z]+)$")
                 .unwrap();


    let stdin = io::stdin();

    for wline in stdin.lock().lines() {
        let line = wline.unwrap();

        let caps = re.captures(&line).unwrap();


        let wire_name = caps.name("out").unwrap().to_string();

        let in2_str = caps.name("in2").unwrap();

        let in2 = match in2_str.parse::<u16>() {
            Ok(num) => OperArg::Val(num),
            Err(_) => OperArg::Wire(in2_str.to_string())
        };

        match caps.name("oper") {
            Some(oper) => {
                // Some operation performed.
                if oper == "NOT" {
                    wire_ops.insert(wire_name, Operation::Not(in2));
                } else {
                    let in1_str = caps.name("in1").unwrap();

                    let in1 = match in1_str.parse::<u16>() {
                        Ok(num) => OperArg::Val(num),
                        Err(_) => OperArg::Wire(in1_str.to_string())
                    };

                    match oper {
                        "AND" => {
                            wire_ops.insert(wire_name, Operation::And(in1, in2));
                        }
                        "LSHIFT" => {
                            wire_ops.insert(wire_name, Operation::LShift(in1, in2));
                        }
                        "OR" => {
                            wire_ops.insert(wire_name, Operation::Or(in1, in2));
                        }
                        "RSHIFT" => {
                            wire_ops.insert(wire_name, Operation::RShift(in1, in2));
                        }
                        _ => panic!("Unknown operation: {:?}", oper),
                    }
                }
            }
            None => {
                // Simple assignment of a number or of one wire to another.
                wire_ops.insert(wire_name, Operation::Alias(in2));
            }
        }

    }




    let mut resolved_wires: HashMap<WireName, u16> = HashMap::new();

    while wire_ops.len() != resolved_wires.len() {

        let (r_name, r_oper) = wire_ops.iter().find(|&(key, val)| !resolved_wires.contains_key(key) && val.has_resolved_inputs(&resolved_wires)).unwrap();

        let wire_name = r_name.clone();
        let wire_val = r_oper.resolve_wire(&resolved_wires).unwrap();

        resolved_wires.insert(wire_name, wire_val);
    }

    {
        let a_val = resolved_wires.get("a").unwrap();
        println!("Wire A = {}", a_val);

        wire_ops.remove("b").is_none();
        wire_ops.insert("b".to_string(), Operation::Alias(OperArg::Val(*a_val)));
    }

    println!("Resetting.");
    resolved_wires.clear();

    println!("Setting A -> B");

    while wire_ops.len() != resolved_wires.len() {

        let (r_name, r_oper) = wire_ops.iter().find(|&(key, val)| !resolved_wires.contains_key(key) && val.has_resolved_inputs(&resolved_wires)).unwrap();

        let wire_name = r_name.clone();
        let wire_val = r_oper.resolve_wire(&resolved_wires).unwrap();

        resolved_wires.insert(wire_name, wire_val);
    }

    let a_val2 = resolved_wires.get("a").unwrap();
    println!("Wire A = {}", a_val2);
}
