#![feature(iterator_step_by)]

extern crate regex;
extern crate primal;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use std::str::FromStr;

use std::env;

lazy_static! {
    static ref INSTR_RE: Regex = Regex::new(r"^(?P<instr>set|sub|mul|jnz|subr1|subr2|subr3)(:? (?P<arg_1>[abcdefgh]|-?[0-9]+)(:? (?P<arg_2>[abcdefgh]|-?[0-9]+))?)?$").unwrap();
    static ref ARG_RE: Regex = Regex::new(r"^(?P<reg>[abcdefgh])|(?P<val>-?[0-9]+)$").unwrap();
}


#[derive(Debug, PartialEq, Eq)]
enum Value {
    Imm(isize),
    Reg(char),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = ARG_RE.captures(s).ok_or(())?;

        if let Some(reg_m) = caps.name("reg") {
            let reg = reg_m.as_str().chars().next().ok_or(())?;

            Ok(Value::Reg(reg))
        } else if let Some(val_m) = caps.name("val") {
            let val = val_m.as_str().parse().map_err(|_| ())?;

            Ok(Value::Imm(val))
        } else {
            Err(())
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Set(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Jnz(Value, Value),
    Subroutine1,
    Subroutine2,
    Subroutine3,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = INSTR_RE.captures(s).ok_or(())?;

        match caps.name("instr").ok_or(())?.as_str() {
            "set" => {
                let arg_1 = caps.name("arg_1").ok_or(())?.as_str().parse()?;
                let arg_2 = caps.name("arg_2").ok_or(())?.as_str().parse()?;

                Ok(Instruction::Set(arg_1, arg_2))
            }
            "sub" => {
                let arg_1 = caps.name("arg_1").ok_or(())?.as_str().parse()?;
                let arg_2 = caps.name("arg_2").ok_or(())?.as_str().parse()?;

                Ok(Instruction::Sub(arg_1, arg_2))
            }
            "mul" => {
                let arg_1 = caps.name("arg_1").ok_or(())?.as_str().parse()?;
                let arg_2 = caps.name("arg_2").ok_or(())?.as_str().parse()?;

                Ok(Instruction::Mul(arg_1, arg_2))
            }
            "jnz" => {
                let arg_1 = caps.name("arg_1").ok_or(())?.as_str().parse()?;
                let arg_2 = caps.name("arg_2").ok_or(())?.as_str().parse()?;

                Ok(Instruction::Jnz(arg_1, arg_2))
            }
            "subr1" => {
                Ok(Instruction::Subroutine1)
            }
            "subr2" => {
                Ok(Instruction::Subroutine2)
            }
            "subr3" => {
                Ok(Instruction::Subroutine3)
            }
            _ => Err(()),
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
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



struct Computer {
    regs: HashMap<char, isize>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            regs: HashMap::new(),
        }
    }

    fn get_reg_int(&mut self, reg_name: char) -> &mut isize {
        self.regs.entry(reg_name).or_insert(0)
    }

    fn get_reg_mut(&mut self, val: &Value) -> Result<&mut isize, ()> {
        match *val {
            Value::Reg(reg_name) => Ok(self.get_reg_int(reg_name)),
            Value::Imm(_) => Err(()),
        }
    }

    fn get_val(&mut self, val: &Value) -> isize {
        match *val {
            Value::Reg(reg_name) => *self.get_reg_int(reg_name),
            Value::Imm(val) => val,
        }
    }

    fn run_program(&mut self, prog: &Program) -> Result<(isize, isize), ()> {
        let mut pc = 0;
        let mut mul_ctr = 0;

        while let Some(instr) = prog.0.get(pc as usize) {
            match *instr {
                Instruction::Set(ref arg_1, ref arg_2) => {
                    let param = self.get_val(arg_2);
                    let reg = self.get_reg_mut(arg_1)?;

                    *reg = param;

                    pc += 1;
                }
                Instruction::Sub(ref arg_1, ref arg_2) => {
                    let param = self.get_val(arg_2);
                    let reg = self.get_reg_mut(arg_1)?;

                    *reg -= param;

                    pc += 1;
                }
                Instruction::Mul(ref arg_1, ref arg_2) => {
                    let param = self.get_val(arg_2);
                    let reg = self.get_reg_mut(arg_1)?;

                    *reg *= param;

                    pc += 1;

                    mul_ctr += 1;
                }
                Instruction::Jnz(ref arg_1, ref arg_2) => {
                    let val = self.get_val(arg_1);

                    if val != 0 {
                        pc += self.get_val(arg_2);
                    } else {
                        pc += 1;
                    }
                }
                Instruction::Subroutine1 => {
                    let val_d = self.get_val(&Value::Reg('d'));
                    let val_b = self.get_val(&Value::Reg('b'));

                    let val_e0 = self.get_val(&Value::Reg('e'));


                    if (val_e0..(val_b + 1)).any(|v| val_d * v == val_b) {
                        let reg_f = self.get_reg_mut(&Value::Reg('f'))?;
                        *reg_f = 0;
                    }

                    {
                        *self.get_reg_mut(&Value::Reg('g'))? = 0;
                    }
                    {
                        *self.get_reg_mut(&Value::Reg('e'))? = val_b;
                    }

                    mul_ctr += val_b - val_e0;

                    pc += 1;
                }
                Instruction::Subroutine2 => {
                    let val_b = self.get_val(&Value::Reg('b'));

                    let val_d0 = self.get_val(&Value::Reg('d'));
                    let val_e0 = 2;

                    if (val_d0..(val_b + 1)).any(|v_d| (val_e0..(val_b + 1)).any(|v_e| v_d * v_e == val_b)) {
                        let reg_f = self.get_reg_mut(&Value::Reg('f'))?;
                        *reg_f = 0;
                    }


                    {
                        *self.get_reg_mut(&Value::Reg('g'))? = 0;
                    }
                    {
                        *self.get_reg_mut(&Value::Reg('e'))? = val_b;
                    }
                    {
                        *self.get_reg_mut(&Value::Reg('d'))? = val_b;
                    }


                    mul_ctr += (val_b - val_e0) * (val_b - val_d0);

                    pc += 1;
                }
                Instruction::Subroutine3 => {
                    // This only works for positive values of b and c

                    let val_c = self.get_val(&Value::Reg('c'));

                    let val_b0 = self.get_val(&Value::Reg('b'));
                    let val_h0 = self.get_val(&Value::Reg('d'));


                    let mut f;
                    let mut b_vals = ((val_b0 + 17)..).step_by(17);
                    let mut b = val_b0;
                    let mut h = val_h0;
                    loop {
                        if !primal::is_prime(b as u64) {
                            h += 1;
                            f = 0;
                        } else {
                            f = 1;
                        }

                        mul_ctr += (b - 2) * (b - 2);

                        if b == val_c {
                            break;
                        }

                        b = if let Some(val) = b_vals.next() {
                            val
                        } else {
                            panic!("this should be an infinite iterator")
                        };
                    }


                    {
                        *self.get_reg_mut(&Value::Reg('g'))? = 0;
                    }
                    {
                        *self.get_reg_mut(&Value::Reg('b'))? = val_c;
                    }
                    {
                        *self.get_reg_mut(&Value::Reg('e'))? = val_c;
                    }
                    {
                        *self.get_reg_mut(&Value::Reg('d'))? = val_c;
                    }
                    {
                        *self.get_reg_mut(&Value::Reg('f'))? = f;
                    }
                    {
                        *self.get_reg_mut(&Value::Reg('h'))? = h;
                    }

                    pc += 1;
                }
            }
        }

        Ok((mul_ctr, self.get_val(&Value::Reg('h'))))
    }
}


fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let program = input_str.parse().expect("parse error");


    let mut args = env::args();
    args.next();

    if let Some(arg) = args.next() {
        match arg.as_str() {
            "-a" => {
                let mut computer = Computer::new();

                let (num_mul, reg_h) = computer
                    .run_program(&program)
                    .expect("invalid program");

                println!(
                    "Debug mode: The 'mul' instruction has been called {} times. The 'h' register contains {}.",
                    num_mul,
                    reg_h
                );
            },
            "-b" => {
                let mut computer = Computer::new();
                *computer.get_reg_mut(&Value::Reg('a')).unwrap() = 1;

                let (num_mul, reg_h) = computer
                    .run_program(&program)
                    .expect("invalid program");

                println!(
                    "Release mode: The 'mul' instruction has been called {} times. The 'h' register contains {}.",
                    num_mul,
                    reg_h
                );
            },
            _ => {panic!("Use -a or -b argument to specify the part of the puzzle.");}
        }
    } else {
        panic!("Use -a or -b argument to specify the part of the puzzle.");
    }
}


#[cfg(test)]
mod tests {
    use super::Computer;
    use std::collections::HashMap;

    #[test]
    fn part_a_test() {
        let input_str = include_str!("../input.txt");

        let program = input_str.parse().expect("parse error");

        let mut computer = Computer::new();

        let (num_mul, _) = computer
            .run_program(&program)
            .expect("invalid program");

        let mut final_reg_state = HashMap::new();
        final_reg_state.insert('a', 0);
        final_reg_state.insert('b', 57);
        final_reg_state.insert('c', 57);
        final_reg_state.insert('d', 57);
        final_reg_state.insert('e', 57);
        final_reg_state.insert('f', 0);
        final_reg_state.insert('g', 0);
        final_reg_state.insert('h', 1);

        assert_eq!(3025, num_mul);
        assert_eq!(final_reg_state, computer.regs);
    }

    #[test]
    fn part_a_optimized_test() {
        let input_str = include_str!("../input_optimized.txt");

        let program = input_str.parse().expect("parse error");

        let mut computer = Computer::new();

        let (num_mul, _) = computer
            .run_program(&program)
            .expect("invalid program");

        let mut final_reg_state = HashMap::new();
        final_reg_state.insert('a', 0);
        final_reg_state.insert('b', 57);
        final_reg_state.insert('c', 57);
        final_reg_state.insert('d', 57);
        final_reg_state.insert('e', 57);
        final_reg_state.insert('f', 0);
        final_reg_state.insert('g', 0);
        final_reg_state.insert('h', 1);

        assert_eq!(3025, num_mul);
        assert_eq!(final_reg_state, computer.regs);
    }

    #[test]
    fn part_a_more_optimized_test() {
        let input_str = include_str!("../input_optimized_2.txt");

        let program = input_str.parse().expect("parse error");

        let mut computer = Computer::new();

        let (num_mul, _) = computer
            .run_program(&program)
            .expect("invalid program");

        let mut final_reg_state = HashMap::new();
        final_reg_state.insert('a', 0);
        final_reg_state.insert('b', 57);
        final_reg_state.insert('c', 57);
        final_reg_state.insert('d', 57);
        final_reg_state.insert('e', 57);
        final_reg_state.insert('f', 0);
        final_reg_state.insert('g', 0);
        final_reg_state.insert('h', 1);

        assert_eq!(3025, num_mul);
        assert_eq!(final_reg_state, computer.regs);
    }

    #[test]
    fn part_a_most_optimized_test() {
        let input_str = include_str!("../input_optimized_3.txt");

        let program = input_str.parse().expect("parse error");

        let mut computer = Computer::new();

        let (num_mul, _) = computer
            .run_program(&program)
            .expect("invalid program");

        let mut final_reg_state = HashMap::new();
        final_reg_state.insert('a', 0);
        final_reg_state.insert('b', 57);
        final_reg_state.insert('c', 57);
        final_reg_state.insert('d', 57);
        final_reg_state.insert('e', 57);
        final_reg_state.insert('f', 0);
        final_reg_state.insert('g', 0);
        final_reg_state.insert('h', 1);

        assert_eq!(3025, num_mul);
        assert_eq!(final_reg_state, computer.regs);
    }
}