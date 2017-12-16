extern crate regex;

use regex::Regex;

use std::io;
use std::io::prelude::*;

use std::str::FromStr;

use std::collections::HashMap;

use std::cmp;

#[derive(Debug)]
enum Condition {
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
    Equal,
    NotEqual,
}

impl Condition {
    fn apply(&self, left: isize, right: isize) -> bool {
        match *self {
            Condition::Greater => left > right,
            Condition::Less => left < right,
            Condition::GreaterOrEqual => left >= right,
            Condition::LessOrEqual => left <= right,
            Condition::Equal => left == right,
            Condition::NotEqual => left != right,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Inc,
    Dec,
}

impl Operation {
    fn exec(&self, left: isize, right: isize) -> isize {
        match *self {
            Operation::Inc => left + right,
            Operation::Dec => left - right,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op_register: String,
    operation: Operation,
    op_operand: isize,
    cond_register: String,
    condition: Condition,
    cond_operand: isize,
}


impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?P<op_reg_name>[a-z]+) (?P<op>inc|dec) (?P<op_operand>-?[0-9]+) if (?P<cond_reg_name>[a-z]+) (?P<cond_op>>|<|>=|<=|==|!=) (?P<cond_operand>-?[0-9]+)").unwrap();

        let caps = re.captures(s).ok_or(())?;

        let op_register = caps.name("op_reg_name").ok_or(())?.as_str().to_string();

        let operation = match caps.name("op").ok_or(())?.as_str() {
            "inc" => Ok(Operation::Inc),
            "dec" => Ok(Operation::Dec),
            _ => Err(()),
        }?;

        let op_operand = caps.name("op_operand")
            .ok_or(())?
            .as_str()
            .parse()
            .map_err(|_| ())?;

        let cond_register = caps.name("cond_reg_name").ok_or(())?.as_str().to_string();

        let condition = match caps.name("cond_op").ok_or(())?.as_str() {
            ">" => Ok(Condition::Greater),
            "<" => Ok(Condition::Less),
            ">=" => Ok(Condition::GreaterOrEqual),
            "<=" => Ok(Condition::LessOrEqual),
            "==" => Ok(Condition::Equal),
            "!=" => Ok(Condition::NotEqual),
            _ => Err(()),
        }?;

        let cond_operand = caps.name("cond_operand")
            .ok_or(())?
            .as_str()
            .parse()
            .map_err(|_| ())?;

        Ok(Instruction {
            op_register,
            operation,
            op_operand,
            cond_register,
            condition,
            cond_operand,
        })
    }
}

#[derive(Debug)]
struct Program(Vec<Instruction>);

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Program(
            s.lines().map(|line| line.parse()).collect::<Result<
                Vec<Instruction>,
                (),
            >>()?,
        ))
    }
}

#[derive(Debug)]
struct Computer {
    pc: usize,
    registers: HashMap<String, isize>,
    overall_max: Option<isize>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            pc: 0,
            registers: HashMap::new(),
            overall_max: None,
        }
    }

    fn get_register_mut(&mut self, name: &str) -> &mut isize {
        self.registers.entry(name.to_string()).or_insert(0)
    }

    fn run_program(&mut self, prog: &Program) {

        while let Some(instr) = prog.0.get(self.pc) {

            let cond_eval = instr.condition.apply(
                *self.get_register_mut(&instr.cond_register),
                instr.cond_operand,
            );

            if cond_eval {
                *self.get_register_mut(&instr.op_register) = instr.operation.exec(
                    *self.get_register_mut(&instr.op_register),
                    instr.op_operand,
                );
            }


            if let Some(o_max) = self.overall_max {
                if let Some(curr_max) = self.registers.values().max() {
                    self.overall_max = Some(cmp::max(o_max, *curr_max));
                }
            } else {
                self.overall_max = self.registers.values().max().map(|m| *m);
            }

            self.pc += 1;
        }
    }
}


fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let program: Program = input_str.parse().expect("parsing error");

    let mut computer = Computer::new();
    computer.run_program(&program);


    println!(
        "The maximum value of any register after the program run is {}.",
        computer.registers.values().max().expect("no max")
    );
    println!(
        "The maximum value of any register at any time during the program run was {}.",
        computer.overall_max.expect("no max")
    );
}


#[cfg(test)]
mod tests {
    use super::Computer;

    const TEST_INPUT: &str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    #[test]
    fn end_register_max_value_test() {
        let prog = TEST_INPUT.parse().unwrap();
        let mut comp = Computer::new();

        comp.run_program(&prog);
        assert_eq!(1, *comp.registers.values().max().expect("no max"));
    }

    #[test]
    fn all_time_register_max_value_test() {
        let prog = TEST_INPUT.parse().unwrap();
        let mut comp = Computer::new();

        comp.run_program(&prog);
        assert_eq!(10, comp.overall_max.expect("no max"));
    }
}
