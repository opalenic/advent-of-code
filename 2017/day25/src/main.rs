use std::collections::HashSet;

trait State {
    fn get_default_state() -> Self;
    fn run_state_change(&self, machine: &mut MachineRegs) -> Self;
}

#[derive(Debug)]
struct MachineRegs {
    tape: HashSet<isize>,
    cursor: isize,
}

impl MachineRegs {
    fn new() -> MachineRegs {
        MachineRegs {
            tape: HashSet::new(),
            cursor: 0,
        }
    }
}

#[derive(Debug)]
struct Machine<T: State> {
    regs: MachineRegs,
    state: T,
}

impl<T> Machine<T>
where
    T: State,
{
    fn new() -> Machine<T> {
        Machine {
            regs: MachineRegs::new(),
            state: T::get_default_state(),
        }
    }

    fn do_step(&mut self) {
        self.state = self.state.run_state_change(&mut self.regs);
    }

    fn get_checksum(&self) -> usize {
        self.regs.tape.len()
    }
}

// #[derive(Debug, PartialEq, Eq)]
enum MachineStates {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl State for MachineStates {
    fn get_default_state() -> MachineStates {
        // Begin in state A.
        MachineStates::A
    }

    fn run_state_change(&self, regs: &mut MachineRegs) -> MachineStates {
        match *self {
            // In state A:
            //   If the current value is 0:
            //     - Write the value 1.
            //     - Move one slot to the right.
            //     - Continue with state B.
            //   If the current value is 1:
            //     - Write the value 0.
            //     - Move one slot to the left.
            //     - Continue with state C.
            MachineStates::A => {
                if regs.tape.remove(&regs.cursor) {
                    regs.cursor -= 1;
                    MachineStates::C
                } else {
                    regs.tape.insert(regs.cursor);
                    regs.cursor += 1;
                    MachineStates::B
                }
            }
            // In state B:
            //   If the current value is 0:
            //     - Write the value 1.
            //     - Move one slot to the left.
            //     - Continue with state A.
            //   If the current value is 1:
            //     - Write the value 1.
            //     - Move one slot to the left.
            //     - Continue with state D.
            MachineStates::B => {
                if regs.tape.contains(&regs.cursor) {
                    regs.cursor -= 1;
                    MachineStates::D
                } else {
                    regs.tape.insert(regs.cursor);
                    regs.cursor -= 1;
                    MachineStates::A
                }
            }
            // In state C:
            //   If the current value is 0:
            //     - Write the value 1.
            //     - Move one slot to the right.
            //     - Continue with state D.
            //   If the current value is 1:
            //     - Write the value 0.
            //     - Move one slot to the right.
            //     - Continue with state C.
            MachineStates::C => {
                if regs.tape.remove(&regs.cursor) {
                    regs.cursor += 1;
                    MachineStates::C
                } else {
                    regs.tape.insert(regs.cursor);
                    regs.cursor += 1;
                    MachineStates::D
                }
            }
            // In state D:
            //   If the current value is 0:
            //     - Write the value 0.
            //     - Move one slot to the left.
            //     - Continue with state B.
            //   If the current value is 1:
            //     - Write the value 0.
            //     - Move one slot to the right.
            //     - Continue with state E.
            MachineStates::D => {
                if regs.tape.remove(&regs.cursor) {
                    regs.cursor += 1;
                    MachineStates::E
                } else {
                    regs.cursor -= 1;
                    MachineStates::B
                }
            }
            // In state E:
            //   If the current value is 0:
            //     - Write the value 1.
            //     - Move one slot to the right.
            //     - Continue with state C.
            //   If the current value is 1:
            //     - Write the value 1.
            //     - Move one slot to the left.
            //     - Continue with state F.
            MachineStates::E => {
                if regs.tape.contains(&regs.cursor) {
                    regs.cursor -= 1;
                    MachineStates::F
                } else {
                    regs.tape.insert(regs.cursor);
                    regs.cursor += 1;
                    MachineStates::C
                }
            }
            // In state F:
            //   If the current value is 0:
            //     - Write the value 1.
            //     - Move one slot to the left.
            //     - Continue with state E.
            //   If the current value is 1:
            //     - Write the value 1.
            //     - Move one slot to the right.
            //     - Continue with state A.
            MachineStates::F => {
                if regs.tape.contains(&regs.cursor) {
                    regs.cursor += 1;
                    MachineStates::A
                } else {
                    regs.tape.insert(regs.cursor);
                    regs.cursor -= 1;
                    MachineStates::E
                }
            }
        }
    }
}

fn main() {
    let mut machine: Machine<MachineStates> = Machine::new();

    for _ in 0..12656374 {
        machine.do_step();
    }

    println!(
        "The checksum after 12656374 iterations is: {}",
        machine.get_checksum()
    );
}

#[cfg(test)]
mod tests {
    use super::Machine;
    use super::MachineRegs;
    use super::State;
    use super::HashSet;

    #[derive(Debug, PartialEq, Eq)]
    enum SimpleStates {
        A,
        B,
    }

    impl State for SimpleStates {
        fn get_default_state() -> SimpleStates {
            // Begin in state A.
            SimpleStates::A
        }

        fn run_state_change(&self, regs: &mut MachineRegs) -> SimpleStates {
            match *self {
                // In state A:
                //   If the current value is 0:
                //     - Write the value 1.
                //     - Move one slot to the right.
                //     - Continue with state B.
                //   If the current value is 1:
                //     - Write the value 0.
                //     - Move one slot to the left.
                //     - Continue with state B.
                SimpleStates::A => {
                    if regs.tape.remove(&regs.cursor) {
                        regs.cursor -= 1;
                    } else {
                        regs.tape.insert(regs.cursor);
                        regs.cursor += 1;
                    }
                    SimpleStates::B
                }
                // In state B:
                //   If the current value is 0:
                //     - Write the value 1.
                //     - Move one slot to the left.
                //     - Continue with state A.
                //   If the current value is 1:
                //     - Write the value 1.
                //     - Move one slot to the right.
                //     - Continue with state A.
                SimpleStates::B => {
                    if regs.tape.contains(&regs.cursor) {
                        regs.cursor += 1;
                    } else {
                        regs.tape.insert(regs.cursor);
                        regs.cursor -= 1;
                    }
                    SimpleStates::A
                }
            }
        }
    }

    #[test]
    fn simple_state_test() {
        let mut machine: Machine<SimpleStates> = Machine::new();
        let mut test_tape = HashSet::new();

        // ... 0  0  0 [0] 0  0 ... (before any steps; about to run state A)
        assert_eq!(machine.regs.tape, test_tape);
        assert_eq!(machine.regs.cursor, 0);
        assert_eq!(machine.state, SimpleStates::A);

        machine.do_step();

        // ... 0  0  0  1 [0] 0 ... (after 1 step;     about to run state B)
        test_tape.insert(0);
        assert_eq!(machine.regs.tape, test_tape);
        assert_eq!(machine.regs.cursor, 1);
        assert_eq!(machine.state, SimpleStates::B);

        machine.do_step();

        // ... 0  0  0 [1] 1  0 ... (after 2 steps;    about to run state A)
        test_tape.insert(1);
        assert_eq!(machine.regs.tape, test_tape);
        assert_eq!(machine.regs.cursor, 0);
        assert_eq!(machine.state, SimpleStates::A);

        machine.do_step();

        // ... 0  0 [0] 0  1  0 ... (after 3 steps;    about to run state B)
        test_tape.remove(&0);
        assert_eq!(machine.regs.tape, test_tape);
        assert_eq!(machine.regs.cursor, -1);
        assert_eq!(machine.state, SimpleStates::B);

        machine.do_step();

        // ... 0 [0] 1  0  1  0 ... (after 4 steps;    about to run state A)
        test_tape.insert(-1);
        assert_eq!(machine.regs.tape, test_tape);
        assert_eq!(machine.regs.cursor, -2);
        assert_eq!(machine.state, SimpleStates::A);

        machine.do_step();

        // ... 0  1 [1] 0  1  0 ... (after 5 steps;    about to run state B)
        test_tape.insert(-2);
        assert_eq!(machine.regs.tape, test_tape);
        assert_eq!(machine.regs.cursor, -1);
        assert_eq!(machine.state, SimpleStates::B);

        machine.do_step();

        // ... 0  1  1 [0] 1  0 ... (after 6 steps;    about to run state A)
        assert_eq!(machine.regs.tape, test_tape);
        assert_eq!(machine.regs.cursor, 0);
        assert_eq!(machine.state, SimpleStates::A);

        assert_eq!(machine.get_checksum(), 3);
    }

}
