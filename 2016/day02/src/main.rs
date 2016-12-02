use std::io;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq)]
enum Step {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Step {
    fn from(ch: char) -> Step {
        match ch {
            'U' => Step::Up,
            'R' => Step::Right,
            'D' => Step::Down,
            'L' => Step::Left,
            _ => panic!("Invalid direction: {}", ch),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Step>> {
    input.lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.into())
                .collect()
        })
        .collect()
}

fn get_code(directions: &Vec<Vec<Step>>) -> Vec<char> {
    let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

    let mut code = Vec::new();
    let mut pos = (1, 1);
    for digit_directions in directions {

        for step in digit_directions {
            match *step {
                Step::Up => {
                    if pos.1 > 0 {
                        pos.1 -= 1;
                    }
                }
                Step::Right => {
                    if pos.0 < keypad[pos.1].len() - 1 {
                        pos.0 += 1;
                    }
                }
                Step::Down => {
                    if pos.1 < keypad.len() - 1 {
                        pos.1 += 1;
                    }
                }
                Step::Left => {
                    if pos.0 > 0 {
                        pos.0 -= 1;
                    }
                }
            }
        }

        code.push(keypad[pos.1][pos.0]);
    }

    code
}

fn get_code_b(directions: &Vec<Vec<Step>>) -> Vec<char> {
    let keypad = [[None, None, Some('1'), None, None],
                  [None, Some('2'), Some('3'), Some('4'), None],
                  [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
                  [None, Some('A'), Some('B'), Some('C'), None],
                  [None, None, Some('D'), None, None]];

    let mut code = Vec::new();
    let mut pos = (0, 2);
    for digit_directions in directions {

        for step in digit_directions {
            match *step {
                Step::Up => {
                    if pos.1 > 0 && keypad[pos.1 - 1][pos.0].is_some() {
                        pos.1 -= 1;
                    }
                }
                Step::Right => {
                    if pos.0 < keypad[pos.1].len() - 1 && keypad[pos.1][pos.0 + 1].is_some() {
                        pos.0 += 1;
                    }
                }
                Step::Down => {
                    if pos.1 < keypad.len() - 1 && keypad[pos.1 + 1][pos.0].is_some() {
                        pos.1 += 1;
                    }
                }
                Step::Left => {
                    if pos.0 > 0 && keypad[pos.1][pos.0 - 1].is_some() {
                        pos.0 -= 1;
                    }
                }
            }
        }

        code.push(keypad[pos.1][pos.0].unwrap());
    }

    code
}

fn main() {
    let mut dir_str = String::new();
    io::stdin().read_to_string(&mut dir_str).expect("Invalid input string!");

    let directions = parse_input(&dir_str);

    println!("The bathroom code is: {:?}", get_code(&directions));
    println!("The second bathroom code is: {:?}", get_code_b(&directions));
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::Step;
    use super::get_code;
    use super::get_code_b;

    const TEST_INPUT: &'static str = "ULL\n\
                                      RRDDD\n\
                                      LURDL\n\
                                      UUUUD\n";
    #[test]
    fn parse_test() {
        let directions = parse_input(TEST_INPUT);

        assert_eq!(directions,
                   vec![vec![Step::Up, Step::Left, Step::Left],
                        vec![Step::Right, Step::Right, Step::Down, Step::Down, Step::Down],
                        vec![Step::Left, Step::Up, Step::Right, Step::Down, Step::Left],
                        vec![Step::Up, Step::Up, Step::Up, Step::Up, Step::Down]]);
    }

    #[test]
    fn code_test() {
        let directions = parse_input(TEST_INPUT);

        assert_eq!(get_code(&directions), vec!['1', '9', '8', '5']);
    }

    #[test]
    fn code_test_b() {
        let directions = parse_input(TEST_INPUT);

        assert_eq!(get_code_b(&directions), vec!['5', 'D', 'B', '3']);
    }
}
