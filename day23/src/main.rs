#![feature(plugin)]

#![plugin(regex_macros)]
extern crate regex;

use std::str::FromStr;

use std::error::Error;

use std::fmt;

use std::io;
use std::io::prelude::*;


#[derive(Debug)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "Input parsing error"
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Register {
    A,
    B,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reg_name = match self {
            &Register::A => "a",
            &Register::B => "b",
        };

        write!(f, "{}", reg_name)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, isize),
    Jio(Register, isize),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        fn parse_regname(oper: &str) -> Result<Register, ParseError> {
            if oper.len() != 1 {
                return Err(ParseError);
            }

            let reg = try!(oper.chars().next().ok_or(ParseError));

            match reg {
                'a' => Ok(Register::A),
                'b' => Ok(Register::B),
                _ => Err(ParseError),
            }
        }

        fn parse_offset(oper: &str) -> Result<isize, ParseError> {
            oper.parse::<isize>().map_err(|_| ParseError)
        }


        let re = regex!(r"^(?P<instr>hlf|tpl|inc|jmp|jie|jio) (?P<oper1>[ab]|[+-][:digit:]+)(?:, (?P<oper2>[+-][:digit:]+))?");

        let caps = try!(re.captures(s).ok_or(ParseError));

        let oper1 = try!(caps.name("oper1").ok_or(ParseError));

        match try!(caps.name("instr").ok_or(ParseError)) {
            "hlf" => Ok(Instruction::Hlf(try!(parse_regname(oper1)))),
            "tpl" => Ok(Instruction::Tpl(try!(parse_regname(oper1)))),
            "inc" => Ok(Instruction::Inc(try!(parse_regname(oper1)))),
            "jmp" => Ok(Instruction::Jmp(try!(parse_offset(oper1)))),
            "jie" => {
                let oper2 = try!(caps.name("oper2").ok_or(ParseError));

                Ok(Instruction::Jie(try!(parse_regname(oper1)), try!(parse_offset(oper2))))
            }
            "jio" => {
                let oper2 = try!(caps.name("oper2").ok_or(ParseError));

                Ok(Instruction::Jio(try!(parse_regname(oper1)), try!(parse_offset(oper2))))
            }
            _ => Err(ParseError),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Instruction::Hlf(reg) => write!(f, "hlf {}", reg),
            &Instruction::Tpl(reg) => write!(f, "tpl {}", reg),
            &Instruction::Inc(reg) => write!(f, "inc {}", reg),
            &Instruction::Jmp(offset) => write!(f, "jmp {}", offset),
            &Instruction::Jie(reg, offset) => write!(f, "jie {}, {}", reg, offset),
            &Instruction::Jio(reg, offset) => write!(f, "jio {}, {}", reg, offset),
        }
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
    reg_a: usize,
    reg_b: usize,
    pc: usize,
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let instructions = try!(s.lines().map(|line| line.parse::<Instruction>()).collect());

        Ok(Program {
            instructions: instructions,
            reg_a: 0,
            reg_b: 0,
            pc: 0,
        })
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pc: {}, a: {}, b: {}", self.pc, self.reg_a, self.reg_b)
    }
}


#[derive(Debug, PartialEq, Eq)]
enum ProgramError {
    ProgramCounterError,
    RegisterOverflowError,
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ProgramError {
    fn description(&self) -> &str {
        match self {
            &ProgramError::ProgramCounterError => "Invalid program counter value",
            &ProgramError::RegisterOverflowError => "Arithmetic overflow in register",
        }
    }
}

impl Program {
    fn run(&mut self) -> Result<(), ProgramError> {
        while self.pc != self.instructions.len() {
            try!(self.single_step());
        }

        Ok(())
    }

    fn single_step(&mut self) -> Result<(), ProgramError> {

        let instr = try!(self.instructions.get(self.pc).ok_or(ProgramError::ProgramCounterError));

        let pc_offset;

        match instr {
            &Instruction::Hlf(reg) => {
                let mut r = match reg {
                    Register::A => &mut self.reg_a,
                    Register::B => &mut self.reg_b,
                };

                *r /= 2;
                pc_offset = 1;
            }
            &Instruction::Tpl(reg) => {
                let mut r = match reg {
                    Register::A => &mut self.reg_a,
                    Register::B => &mut self.reg_b,
                };

                *r = try!(r.checked_mul(3).ok_or(ProgramError::RegisterOverflowError));
                pc_offset = 1;
            }
            &Instruction::Inc(reg) => {
                let mut r = match reg {
                    Register::A => &mut self.reg_a,
                    Register::B => &mut self.reg_b,
                };

                *r = try!(r.checked_add(1).ok_or(ProgramError::RegisterOverflowError));
                pc_offset = 1;
            }
            &Instruction::Jmp(offset) => {
                pc_offset = offset;
            }
            &Instruction::Jie(reg, offset) => {
                let reg_val = match reg {
                    Register::A => self.reg_a,
                    Register::B => self.reg_b,
                };

                pc_offset = if reg_val % 2 == 0 {
                    offset
                } else {
                    1
                };
            }
            &Instruction::Jio(reg, offset) => {
                let reg_val = match reg {
                    Register::A => self.reg_a,
                    Register::B => self.reg_b,
                };

                pc_offset = if reg_val == 1 {
                    offset
                } else {
                    1
                };
            }
        }

        if pc_offset >= 0 {
            self.pc = try!(self.pc
                               .checked_add(pc_offset as usize)
                               .ok_or(ProgramError::ProgramCounterError));
        } else {
            self.pc = try!(self.pc
                               .checked_sub((-pc_offset) as usize)
                               .ok_or(ProgramError::ProgramCounterError));
        }


        Ok(())
    }

    fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_b = 0;
        self.pc = 0;
    }
}


fn main() {

    let mut input_str = String::new();
    let mut stdin = io::stdin();

    stdin.read_to_string(&mut input_str).unwrap();

    let mut prog = Program::from_str(&input_str).unwrap();

    prog.run().unwrap();
    println!("Status after part A: {}", prog);

    prog.reset();
    prog.reg_a = 1;

    prog.run().unwrap();
    println!("Status after part B: {}", prog);
}


#[cfg(test)]
mod tests {

    use super::Program;
    use super::Instruction;
    use super::Register;
    use super::ProgramError;
    use std::str::FromStr;
    use std::usize;

    const TEST_INPUT: &'static str = "inc a\n\
                                      jio a, +2\n\
                                      tpl b\n\
                                      inc b";


    const TEST_INPUT_2: &'static str = "tpl a\n\
                                        inc a\n\
                                        jio b, +8\n\
                                        inc b\n\
                                        jie a, +4\n\
                                        tpl a\n\
                                        inc a\n\
                                        jmp +2\n\
                                        hlf a\n\
                                        jmp -7";

    #[test]
    fn parse_test() {
        let prog1 = Program::from_str(TEST_INPUT).unwrap();

        let expected1 = vec![
            Instruction::Inc(Register::A),
            Instruction::Jio(Register::A, 2),
            Instruction::Tpl(Register::B),
            Instruction::Inc(Register::B),
        ];

        assert_eq!(prog1.instructions, expected1);

        let prog2 = TEST_INPUT_2.parse::<Program>().unwrap();

        let expected2 = vec![
            Instruction::Tpl(Register::A),
            Instruction::Inc(Register::A),
            Instruction::Jio(Register::B, 8),
            Instruction::Inc(Register::B),
            Instruction::Jie(Register::A, 4),
            Instruction::Tpl(Register::A),
            Instruction::Inc(Register::A),
            Instruction::Jmp(2),
            Instruction::Hlf(Register::A),
            Instruction::Jmp(-7),
        ];

        assert_eq!(prog2.instructions, expected2);
    }

    #[test]
    fn execution_test() {
        let mut prog1 = Program::from_str(TEST_INPUT).unwrap();

        assert_eq!(prog1.pc, 0);
        assert_eq!(prog1.reg_a, 0);
        assert_eq!(prog1.reg_b, 0);

        // inc a
        assert!(prog1.single_step().is_ok());
        assert_eq!(prog1.pc, 1);
        assert_eq!(prog1.reg_a, 1);
        assert_eq!(prog1.reg_b, 0);

        // jio a, +2
        assert!(prog1.single_step().is_ok());
        assert_eq!(prog1.pc, 3);
        assert_eq!(prog1.reg_a, 1);
        assert_eq!(prog1.reg_b, 0);

        // inc b
        assert!(prog1.single_step().is_ok());
        assert_eq!(prog1.pc, 4);
        assert_eq!(prog1.reg_a, 1);
        assert_eq!(prog1.reg_b, 1);

        assert_eq!(prog1.single_step().err().unwrap(),
                   ProgramError::ProgramCounterError);

        prog1.reset();

        assert!(prog1.run().is_ok());
        assert_eq!(prog1.pc, 4);
        assert_eq!(prog1.reg_a, 1);
        assert_eq!(prog1.reg_b, 1);


        let mut prog2 = Program::from_str(TEST_INPUT_2).unwrap();

        assert_eq!(prog2.pc, 0);
        assert_eq!(prog2.reg_a, 0);
        assert_eq!(prog2.reg_b, 0);


        // tpl a
        assert!(prog2.single_step().is_ok());
        assert_eq!(prog2.pc, 1);
        assert_eq!(prog2.reg_a, 0);
        assert_eq!(prog2.reg_b, 0);

        // inc a
        assert!(prog2.single_step().is_ok());
        assert_eq!(prog2.pc, 2);
        assert_eq!(prog2.reg_a, 1);
        assert_eq!(prog2.reg_b, 0);

        // jio b, +8
        assert!(prog2.single_step().is_ok());
        assert_eq!(prog2.pc, 3);
        assert_eq!(prog2.reg_a, 1);
        assert_eq!(prog2.reg_b, 0);

        // inc b
        assert!(prog2.single_step().is_ok());
        assert_eq!(prog2.pc, 4);
        assert_eq!(prog2.reg_a, 1);
        assert_eq!(prog2.reg_b, 1);

        // jie a, +4
        assert!(prog2.single_step().is_ok());
        assert_eq!(prog2.pc, 5);
        assert_eq!(prog2.reg_a, 1);
        assert_eq!(prog2.reg_b, 1);

        // tpl a
        assert!(prog2.single_step().is_ok());
        assert_eq!(prog2.pc, 6);
        assert_eq!(prog2.reg_a, 3);
        assert_eq!(prog2.reg_b, 1);

        // inc a
        assert!(prog2.single_step().is_ok());
        assert_eq!(prog2.pc, 7);
        assert_eq!(prog2.reg_a, 4);
        assert_eq!(prog2.reg_b, 1);

        // jmp +2
        assert!(prog2.single_step().is_ok());
        assert_eq!(prog2.pc, 9);
        assert_eq!(prog2.reg_a, 4);
        assert_eq!(prog2.reg_b, 1);

        // jmp -7
        assert!(prog2.single_step().is_ok());
        assert_eq!(prog2.pc, 2);
        assert_eq!(prog2.reg_a, 4);
        assert_eq!(prog2.reg_b, 1);

        // jio b, +8
        assert!(prog2.single_step().is_ok());
        assert_eq!(prog2.pc, 10);
        assert_eq!(prog2.reg_a, 4);
        assert_eq!(prog2.reg_b, 1);

        assert_eq!(prog1.single_step().err().unwrap(),
                   ProgramError::ProgramCounterError);


        prog2.reset();

        assert!(prog2.run().is_ok());
        assert_eq!(prog2.pc, 10);
        assert_eq!(prog2.reg_a, 4);
        assert_eq!(prog2.reg_b, 1);
    }

    #[test]
    fn overflow_test() {
        let mut prog = Program::from_str("inc a\ntpl a\njmp -1").unwrap();

        assert_eq!(prog.run().err().unwrap(),
                   ProgramError::RegisterOverflowError);

        prog.pc = 0;
        prog.reg_a = usize::MAX;

        assert_eq!(prog.single_step().err().unwrap(),
                   ProgramError::RegisterOverflowError);
    }
}
