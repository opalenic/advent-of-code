
extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

use std::io;
use std::io::prelude::*;

use std::collections::HashMap;
use std::collections::VecDeque;

use std::str::FromStr;

lazy_static! {
    static ref INSTR_RE: Regex = Regex::new(r"^(?P<instr>snd|set|add|mul|mod|rcv|jgz) (?P<arg_1>[a-z]|-?[0-9]+)(:? (?P<arg_2>[a-z]|-?[0-9]+))?$").unwrap();
    static ref ARG_RE: Regex = Regex::new(r"^(?P<reg>[a-z])|(?P<val>-?[0-9]+)$").unwrap();
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
    Snd(Value),
    Set(Value, Value),
    Add(Value, Value),
    Mul(Value, Value),
    Mod(Value, Value),
    Rcv(Value),
    Jgz(Value, Value),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = INSTR_RE.captures(s).ok_or(())?;

        match caps.name("instr").ok_or(())?.as_str() {
            "snd" => {
                let arg = caps.name("arg_1").ok_or(())?.as_str().parse()?;

                if caps.name("arg_2").is_some() {
                    return Err(());
                }

                Ok(Instruction::Snd(arg))
            }
            "set" => {
                let arg_1 = caps.name("arg_1").ok_or(())?.as_str().parse()?;
                let arg_2 = caps.name("arg_2").ok_or(())?.as_str().parse()?;

                Ok(Instruction::Set(arg_1, arg_2))
            }
            "add" => {
                let arg_1 = caps.name("arg_1").ok_or(())?.as_str().parse()?;
                let arg_2 = caps.name("arg_2").ok_or(())?.as_str().parse()?;

                Ok(Instruction::Add(arg_1, arg_2))
            }
            "mul" => {
                let arg_1 = caps.name("arg_1").ok_or(())?.as_str().parse()?;
                let arg_2 = caps.name("arg_2").ok_or(())?.as_str().parse()?;

                Ok(Instruction::Mul(arg_1, arg_2))
            }
            "mod" => {
                let arg_1 = caps.name("arg_1").ok_or(())?.as_str().parse()?;
                let arg_2 = caps.name("arg_2").ok_or(())?.as_str().parse()?;

                Ok(Instruction::Mod(arg_1, arg_2))
            }
            "rcv" => {
                let arg = caps.name("arg_1").ok_or(())?.as_str().parse()?;

                if caps.name("arg_2").is_some() {
                    return Err(());
                }

                Ok(Instruction::Rcv(arg))
            }
            "jgz" => {
                let arg_1 = caps.name("arg_1").ok_or(())?.as_str().parse()?;
                let arg_2 = caps.name("arg_2").ok_or(())?.as_str().parse()?;

                Ok(Instruction::Jgz(arg_1, arg_2))
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
    last_sound: Option<isize>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            regs: HashMap::new(),
            last_sound: None,
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

    fn run_program(&mut self, prog: &Program) -> Result<Option<isize>, ()> {
        let mut pc = 0;

        while let Some(instr) = prog.0.get(pc as usize) {
            match *instr {
                Instruction::Snd(ref arg) => {
                    self.last_sound = Some(self.get_val(arg));

                    pc += 1;
                }
                Instruction::Set(ref arg_1, ref arg_2) => {
                    let param = self.get_val(arg_2);
                    let reg = self.get_reg_mut(arg_1)?;

                    *reg = param;

                    pc += 1;
                }
                Instruction::Add(ref arg_1, ref arg_2) => {
                    let param = self.get_val(arg_2);
                    let reg = self.get_reg_mut(arg_1)?;

                    *reg += param;

                    pc += 1;
                }
                Instruction::Mul(ref arg_1, ref arg_2) => {
                    let param = self.get_val(arg_2);
                    let reg = self.get_reg_mut(arg_1)?;

                    *reg *= param;

                    pc += 1;
                }
                Instruction::Mod(ref arg_1, ref arg_2) => {
                    let param = self.get_val(arg_2);
                    let reg = self.get_reg_mut(arg_1)?;

                    *reg %= param;

                    pc += 1;
                }
                Instruction::Rcv(ref arg) => {
                    let param = self.get_val(arg);

                    if param != 0 {
                        return Ok(self.last_sound);
                    }

                    pc += 1;
                }
                Instruction::Jgz(ref arg_1, ref arg_2) => {
                    let val = self.get_val(arg_1);

                    if val > 0 {
                        pc += self.get_val(arg_2);
                    } else {
                        pc += 1;
                    }
                }
            }
        }

        Err(())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CompState {
    Ready,
    Sent(isize),
    Blocked,
    Done,
}

struct ComputerV2 {
    regs: HashMap<char, isize>,
    pc: isize,
}

impl ComputerV2 {
    fn new() -> ComputerV2 {
        ComputerV2 {
            regs: HashMap::new(),
            pc: 0,
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

    fn run_program(
        &mut self,
        prog: &Program,
        rx_queue: &mut VecDeque<isize>,
    ) -> Result<CompState, ()> {
        while let Some(instr) = prog.0.get(self.pc as usize) {
            match *instr {
                Instruction::Snd(ref arg) => {
                    self.pc += 1;
                    return Ok(CompState::Sent(self.get_val(arg)));
                }
                Instruction::Set(ref arg_1, ref arg_2) => {
                    {
                        let param = self.get_val(arg_2);
                        let reg = self.get_reg_mut(arg_1)?;

                        *reg = param;
                    }

                    self.pc += 1;
                }
                Instruction::Add(ref arg_1, ref arg_2) => {
                    {
                        let param = self.get_val(arg_2);
                        let reg = self.get_reg_mut(arg_1)?;

                        *reg += param;
                    }

                    self.pc += 1;
                }
                Instruction::Mul(ref arg_1, ref arg_2) => {
                    {
                        let param = self.get_val(arg_2);
                        let reg = self.get_reg_mut(arg_1)?;

                        *reg *= param;
                    }

                    self.pc += 1;
                }
                Instruction::Mod(ref arg_1, ref arg_2) => {
                    {
                        let param = self.get_val(arg_2);
                        let reg = self.get_reg_mut(arg_1)?;

                        *reg %= param;
                    }

                    self.pc += 1;
                }
                Instruction::Rcv(ref arg) => {
                    {
                        let reg = self.get_reg_mut(arg)?;

                        if let Some(val) = rx_queue.pop_front() {
                            *reg = val;
                        } else {
                            return Ok(CompState::Blocked);
                        }
                    }

                    self.pc += 1;
                }
                Instruction::Jgz(ref arg_1, ref arg_2) => {
                    let val = self.get_val(arg_1);

                    if val > 0 {
                        self.pc += self.get_val(arg_2);
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }

        Ok(CompState::Done)
    }
}


struct Runner {
    comp_a: ComputerV2,
    comp_b: ComputerV2,
    comp_a_state: CompState,
    comp_b_state: CompState,
}

impl Runner {
    fn new() -> Runner {
        let comp_a = ComputerV2::new();
        let mut comp_b = ComputerV2::new();

        {
            let reg = comp_b.get_reg_int('p');
            *reg = 1;
        }

        Runner {
            comp_a: comp_a,
            comp_b: comp_b,
            comp_a_state: CompState::Ready,
            comp_b_state: CompState::Ready,
        }
    }

    fn computers_are_done(&self) -> bool {
        (self.comp_a_state == CompState::Done && self.comp_b_state == CompState::Done) ||
            (self.comp_a_state == CompState::Done && self.comp_b_state == CompState::Blocked) ||
            (self.comp_a_state == CompState::Blocked && self.comp_b_state == CompState::Done) ||
            (self.comp_a_state == CompState::Blocked && self.comp_b_state == CompState::Blocked)
    }

    fn run_program(&mut self, prog: &Program) -> Result<usize, ()> {
        let mut comp_b_send_count = 0;

        let mut a_queue = VecDeque::new();
        let mut b_queue = VecDeque::new();

        while !self.computers_are_done() {
            if self.comp_a_state != CompState::Done {
                self.comp_a_state = self.comp_a.run_program(&prog, &mut a_queue)?;

                if let CompState::Sent(val) = self.comp_a_state {
                    b_queue.push_back(val);
                }
            }


            if self.comp_b_state != CompState::Done {
                self.comp_b_state = self.comp_b.run_program(&prog, &mut b_queue)?;

                if let CompState::Sent(val) = self.comp_b_state {
                    a_queue.push_back(val);
                    comp_b_send_count += 1;
                }
            }
        }

        Ok(comp_b_send_count)
    }
}


fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let program = input_str.parse().expect("parse error");
    let mut computer = Computer::new();

    let freq = computer
        .run_program(&program)
        .expect("invalid program")
        .expect("no sound played");

    println!(
        "The last sound played before running 'rcv' with a non-zero argument was: {}",
        freq
    );


    let mut runner = Runner::new();

    let num_sent = runner.run_program(&program).expect("invalid program");
    println!(
        "Program 1 has sent a value to program 0 a total of {} times.",
        num_sent
    );
}


#[cfg(test)]
mod tests {
    use super::Program;
    use super::Value;
    use super::Instruction;
    use super::Computer;
    use super::Runner;

    #[test]
    fn parse_test() {
        let input_str = "set a 1\n\
                         add a 2\n\
                         mul a a\n\
                         mod a 5\n\
                         snd a\n\
                         set a 0\n\
                         rcv a\n\
                         jgz a -1\n\
                         set a 1\n\
                         jgz a -2";

        let expected_prog = Program(vec![
            Instruction::Set(Value::Reg('a'), Value::Imm(1)),
            Instruction::Add(Value::Reg('a'), Value::Imm(2)),
            Instruction::Mul(Value::Reg('a'), Value::Reg('a')),
            Instruction::Mod(Value::Reg('a'), Value::Imm(5)),
            Instruction::Snd(Value::Reg('a')),
            Instruction::Set(Value::Reg('a'), Value::Imm(0)),
            Instruction::Rcv(Value::Reg('a')),
            Instruction::Jgz(Value::Reg('a'), Value::Imm(-1)),
            Instruction::Set(Value::Reg('a'), Value::Imm(1)),
            Instruction::Jgz(Value::Reg('a'), Value::Imm(-2)),
        ]);

        assert_eq!(expected_prog, input_str.parse().expect("parse error"));
    }

    #[test]
    fn eval_test() {
        let prog = Program(vec![
            Instruction::Set(Value::Reg('a'), Value::Imm(1)),
            Instruction::Add(Value::Reg('a'), Value::Imm(2)),
            Instruction::Mul(Value::Reg('a'), Value::Reg('a')),
            Instruction::Mod(Value::Reg('a'), Value::Imm(5)),
            Instruction::Snd(Value::Reg('a')),
            Instruction::Set(Value::Reg('a'), Value::Imm(0)),
            Instruction::Rcv(Value::Reg('a')),
            Instruction::Jgz(Value::Reg('a'), Value::Imm(-1)),
            Instruction::Set(Value::Reg('a'), Value::Imm(1)),
            Instruction::Jgz(Value::Reg('a'), Value::Imm(-2)),
        ]);

        let mut computer = Computer::new();

        assert_eq!(Ok(Some(4)), computer.run_program(&prog));
    }

    #[test]
    fn runner_test() {
        let mut runner = Runner::new();

        let prog = Program(vec![
            Instruction::Snd(Value::Imm(1)),
            Instruction::Snd(Value::Imm(2)),
            Instruction::Snd(Value::Reg('p')),
            Instruction::Rcv(Value::Reg('a')),
            Instruction::Rcv(Value::Reg('b')),
            Instruction::Rcv(Value::Reg('c')),
            Instruction::Rcv(Value::Reg('d')),
        ]);

        assert_eq!(Ok(3), runner.run_program(&prog));
    }
}
