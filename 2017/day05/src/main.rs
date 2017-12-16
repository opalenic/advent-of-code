use std::io;
use std::io::prelude::*;

use std::str::FromStr;

impl FromStr for Computer {
    type Err = ();

    fn from_str(s: &str) -> Result<Computer, Self::Err> {
        Ok(Computer {
            instructions: s.lines()
                .map(|line| line.parse().map_err(|_| ()))
                .collect::<Result<Vec<isize>, Self::Err>>()?,
            pc: 0,
        })
    }
}

#[derive(Debug, Clone)]
struct Computer {
    instructions: Vec<isize>,
    pc: isize,
}

impl Computer {
    fn run_program(&mut self) -> usize {
        let mut step_count = 0;

        while let Some(instr) = self.instructions.get_mut(self.pc as usize) {
            self.pc += *instr;
            *instr += 1;
            step_count += 1;
        }

        step_count
    }

    fn run_program_b(&mut self) -> usize {
        let mut step_count = 0;

        while let Some(instr) = self.instructions.get_mut(self.pc as usize) {
            self.pc += *instr;
            *instr = if *instr >= 3 { *instr - 1 } else { *instr + 1 };
            step_count += 1;
        }

        step_count
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let mut computer: Computer = input_str.parse().unwrap();
    let mut computer_b: Computer = computer.clone();

    println!(
        "The total step count for program A is {}.",
        computer.run_program()
    );
    println!(
        "The total step count for program B is {}.",
        computer_b.run_program_b()
    );
}

#[cfg(test)]
mod tests {
    use super::Computer;

    const TEST_INPUT: &str = "0\n\
                              3\n\
                              0\n\
                              1\n\
                              -3";

    #[test]
    fn program_a_test() {
        let mut computer: Computer = TEST_INPUT.parse().unwrap();

        assert_eq!(5, computer.run_program());
    }

    #[test]
    fn program_b_test() {
        let mut computer: Computer = TEST_INPUT.parse().unwrap();

        assert_eq!(10, computer.run_program_b());
        assert_eq!(vec![2, 3, 2, 3, -1], computer.instructions);
    }
}
