
extern crate regex;
use regex::Regex;

use std::str::FromStr;

use std::io;
use std::io::prelude::*;

use std::ops::Deref;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Program(Vec<Instruction>);

impl Deref for Program {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err(()),
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    CpyReg(Register, Register),
    CpyImm(i64, Register),
    Inc(Register),
    Dec(Register),
    JnzReg(Register, isize),
    JnzImm(i64, isize),
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cpy_reg_re = Regex::new(r"^cpy (?P<from_reg>[abcd]) (?P<to_reg>[abcd])$")
            .map_err(|_| ())?;
        let cpy_imm_re = Regex::new(r"^cpy (?P<val>-?[0-9]+) (?P<to_reg>[abcd])$")
            .map_err(|_| ())?;
        let inc_reg_re = Regex::new(r"^inc (?P<reg>[abcd])$").map_err(|_| ())?;
        let dec_reg_re = Regex::new(r"^dec (?P<reg>[abcd])$").map_err(|_| ())?;
        let jnz_reg_re = Regex::new(r"^jnz (?P<reg>[abcd]) (?P<offset>-?[0-9]+)$")
            .map_err(|_| ())?;
        let jnz_imm_re = Regex::new(r"^jnz (?P<val>[-]?[0-9]+) (?P<offset>-?[0-9]+)$")
            .map_err(|_| ())?;

        Ok(Program(s.lines()
            .map(|line| if let Some(cap) = cpy_reg_re.captures(line) {
                let from_reg = cap.name("from_reg").ok_or(())?.as_str().parse()?;
                let to_reg = cap.name("to_reg").ok_or(())?.as_str().parse()?;

                Ok(Instruction::CpyReg(from_reg, to_reg))

            } else if let Some(cap) = cpy_imm_re.captures(line) {
                let val = cap.name("val").ok_or(())?.as_str().parse().map_err(|_| ())?;
                let to_reg = cap.name("to_reg").ok_or(())?.as_str().parse()?;

                Ok(Instruction::CpyImm(val, to_reg))

            } else if let Some(cap) = inc_reg_re.captures(line) {
                let reg = cap.name("reg").ok_or(())?.as_str().parse()?;

                Ok(Instruction::Inc(reg))

            } else if let Some(cap) = dec_reg_re.captures(line) {
                let reg = cap.name("reg").ok_or(())?.as_str().parse()?;

                Ok(Instruction::Dec(reg))

            } else if let Some(cap) = jnz_reg_re.captures(line) {
                let reg = cap.name("reg").ok_or(())?.as_str().parse()?;
                let offset = cap.name("offset").ok_or(())?.as_str().parse().map_err(
                    |_| (),
                )?;

                Ok(Instruction::JnzReg(reg, offset))

            } else if let Some(cap) = jnz_imm_re.captures(line) {
                let val = cap.name("val").ok_or(())?.as_str().parse().map_err(|_| ())?;
                let offset = cap.name("offset").ok_or(())?.as_str().parse().map_err(
                    |_| (),
                )?;

                Ok(Instruction::JnzImm(val, offset))

            } else {
                Err(())
            })
            .collect::<Result<Vec<Instruction>, ()>>()?))
    }
}


struct Computer {
    regs: HashMap<Register, i64>,
    pc: usize,
}

impl Computer {
    fn new() -> Computer {
        let mut regs = HashMap::new();
        regs.insert(Register::A, 0);
        regs.insert(Register::B, 0);
        regs.insert(Register::C, 0);
        regs.insert(Register::D, 0);

        Computer { regs: regs, pc: 0 }
    }

    fn reset(&mut self) {
        *self.regs.get_mut(&Register::A).unwrap() = 0;
        *self.regs.get_mut(&Register::B).unwrap() = 0;
        *self.regs.get_mut(&Register::C).unwrap() = 0;
        *self.regs.get_mut(&Register::D).unwrap() = 0;
        self.pc = 0;
    }

    fn run_program(&mut self, prog: &Program) -> Result<(), ()> {

        while let Some(instr) = prog.get(self.pc) {
            match *instr {
                Instruction::CpyReg(from_reg, to_reg) => {
                    *self.regs.get_mut(&to_reg).unwrap() = self.regs[&from_reg];
                    self.pc += 1;
                }
                Instruction::CpyImm(val, to_reg) => {
                    *self.regs.get_mut(&to_reg).unwrap() = val;
                    self.pc += 1;
                }
                Instruction::Inc(reg) => {
                    *self.regs.get_mut(&reg).unwrap() += 1;
                    self.pc += 1;
                }
                Instruction::Dec(reg) => {
                    *self.regs.get_mut(&reg).unwrap() -= 1;
                    self.pc += 1;
                }
                Instruction::JnzReg(reg, offset) => {
                    if self.regs[&reg] != 0 {
                        self.pc = ((self.pc as isize) + offset) as usize;
                    } else {
                        self.pc += 1;
                    }
                }
                Instruction::JnzImm(val, offset) => {
                    if val != 0 {
                        self.pc = ((self.pc as isize) + offset) as usize;
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }

        if self.pc == prog.len() {
            Ok(())
        } else {
            Err(())
        }
    }
}

fn main() {

    let mut program_str = String::new();
    io::stdin().read_to_string(&mut program_str).expect(
        "invalid input",
    );

    let prog = program_str.parse().expect("error parsing program");

    let mut comp = Computer::new();

    comp.run_program(&prog).expect("error running program");
    println!("Register A contains: {}", comp.regs[&Register::A]);


    println!("Resetting computer. Setting C to 1.");
    comp.reset();
    *comp.regs.get_mut(&Register::C).unwrap() = 1;


    comp.run_program(&prog).expect("error running program");
    println!("Register A contains: {}", comp.regs[&Register::A]);

}

#[cfg(test)]
mod tests {
    use super::Program;
    use super::Register;
    use super::Instruction;
    use super::Computer;

    const TEST_PROGRAM_STR: &str = "cpy 1 a\n\
                                    cpy -21 b\n\
                                    jnz c 2\n\
                                    jnz 1 5\n\
                                    jnz -3 23\n\
                                    inc d\n\
                                    dec c\n\
                                    jnz c -2\n\
                                    cpy a c\n";

    const TEST_PROGRAM_STR_2: &str = "cpy 41 a\n\
                                      inc a\n\
                                      inc a\n\
                                      dec a\n\
                                      jnz a 2\n\
                                      dec a\n";



    #[test]
    fn parse_test() {
        let expected_prog = Program(vec![
            Instruction::CpyImm(1, Register::A),
            Instruction::CpyImm(-21, Register::B),
            Instruction::JnzReg(Register::C, 2),
            Instruction::JnzImm(1, 5),
            Instruction::JnzImm(-3, 23),
            Instruction::Inc(Register::D),
            Instruction::Dec(Register::C),
            Instruction::JnzReg(Register::C, -2),
            Instruction::CpyReg(Register::A, Register::C),
        ]);

        assert_eq!(
            expected_prog,
            TEST_PROGRAM_STR.parse().expect(
                "could not parse test input",
            )
        );
    }

    #[test]
    fn prog_test() {
        let prog = TEST_PROGRAM_STR_2.parse().unwrap();

        let mut comp = Computer::new();

        assert!(comp.run_program(&prog).is_ok());

        assert_eq!(42, comp.regs[&Register::A]);
    }
}
