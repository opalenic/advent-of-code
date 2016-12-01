
use std::error::Error;

use std::str::FromStr;

use std::fmt;

use std::io;
use std::io::Read;

use std::mem;

use std::env;


#[derive(Debug, PartialEq)]
struct LightParseError;

impl fmt::Display for LightParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

impl Error for LightParseError {
    fn description(&self) -> &str {
        "error during input parsing"
    }
}


//                 y
//   +-------------->
//   |
//   |
//   |
// x |
//   v

#[derive(Debug)]
struct Lights {
    current: Vec<Vec<bool>>,
    next: Vec<Vec<bool>>,
    dimensions: (usize, usize),
}

impl FromStr for Lights {
    type Err = LightParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let initial_state_res = s.lines()
                                 .map(|line| {
                                     line.chars()
                                         .map(|ch| {
                                             match ch {
                                                 '.' => Ok(false),
                                                 '#' => Ok(true),
                                                 _ => Err(LightParseError),
                                             }
                                         })
                                         .collect::<Result<Vec<bool>, LightParseError>>()
                                 })
                                 .collect::<Result<Vec<Vec<bool>>, LightParseError>>();

        match initial_state_res {
            Ok(initial_state) => {
                let dims = (initial_state.len(), initial_state[0].len());

                if initial_state.iter().all(|line| line.len() == dims.1) {
                    Ok(Lights {
                        current: initial_state,
                        next: vec![vec![false; dims.1]; dims.0],
                        dimensions: dims,
                    })
                } else {
                    Err(LightParseError)
                }
            }
            Err(err) => Err(err),
        }
    }
}

impl fmt::Display for Lights {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::with_capacity((self.dimensions.0 + 1) * self.dimensions.1);

        for light_row in self.current.iter() {
            for light_state in light_row {
                out.push(if *light_state {
                    '#'
                } else {
                    '.'
                });
            }

            out.push('\n');
        }

        fmt.write_str(&out)
    }
}

impl Lights {
    fn step(&mut self) {

        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                let mut neighbours_on = 0;

                if x > 0 {
                    neighbours_on += self.current[x - 1][y] as u8;

                    if y > 0 {
                        neighbours_on += self.current[x - 1][y - 1] as u8;
                    }

                    if y < self.dimensions.1 - 1 {
                        neighbours_on += self.current[x - 1][y + 1] as u8;
                    }
                }

                if y > 0 {
                    neighbours_on += self.current[x][y - 1] as u8;
                }

                if y < self.dimensions.1 - 1 {
                    neighbours_on += self.current[x][y + 1] as u8;
                }

                if x < self.dimensions.0 - 1 {
                    neighbours_on += self.current[x + 1][y] as u8;

                    if y > 0 {
                        neighbours_on += self.current[x + 1][y - 1] as u8;
                    }

                    if y < self.dimensions.1 - 1 {
                        neighbours_on += self.current[x + 1][y + 1] as u8;
                    }
                }


                self.next[x][y] = if self.current[x][y] {
                    if neighbours_on == 2 || neighbours_on == 3 {
                        true
                    } else {
                        false
                    }
                } else {
                    if neighbours_on == 3 {
                        true
                    } else {
                        false
                    }
                };
            }
        }

        mem::swap(&mut self.current, &mut self.next);
    }

    fn step_b(&mut self) {
        self.step();
        self.current[0][0] = true;
        self.current[0][self.dimensions.1 - 1] = true;
        self.current[self.dimensions.0 - 1][0] = true;
        self.current[self.dimensions.0 - 1][self.dimensions.1 - 1] = true;
    }

    fn get_num_lights_on(&self) -> u32 {
        self.current.iter().fold(0, |acc, light_row| {
            acc +
            light_row.iter().fold(0, |acc, light_state| {
                if *light_state {
                    acc + 1
                } else {
                    acc
                }
            })
        })
    }
}

fn main() {

    let mut a = env::args();

    a.next(); // The first argument is the binary name/path

    let num_iterations = a.next().unwrap().parse::<u32>().unwrap();
    let corners_on = match a.next().unwrap().as_str() {
        "on" => true,
        "off" => false,
        _ => panic!()
    };


    let mut input_str = String::new();
    let mut stdin = io::stdin();

    stdin.read_to_string(&mut input_str).unwrap();

    let mut lights = Lights::from_str(&input_str).unwrap();

    for i in 0..(num_iterations + 1) {
        println!("State after {} iterations.", i);
        println!("Number of lights on: {}", lights.get_num_lights_on());
        println!("{}", lights);

        if !corners_on {
            lights.step();
        } else {
            lights.step_b();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Lights;
    use super::LightParseError;
    use std::str::FromStr;

    const TEST_INPUT_NOT_RECTANGULAR: &'static str = ".#.#.#\n\
                                                      ...##.\n\
                                                      #....\n\
                                                      ..#...\n\
                                                      #.#..#\n\
                                                      ####..";

    const TEST_INPUT_INVALID_CHAR: &'static str = ".#.#.#\n\
                                                   .X.##.\n\
                                                   #....#\n\
                                                   ..#...\n\
                                                   #.#..#\n\
                                                   ####..";


    const TEST_INPUT: [&'static str; 5] = [".#.#.#\n\
                                            ...##.\n\
                                            #....#\n\
                                            ..#...\n\
                                            #.#..#\n\
                                            ####..",

                                           "..##..\n\
                                            ..##.#\n\
                                            ...##.\n\
                                            ......\n\
                                            #.....\n\
                                            #.##..",

                                           "..###.\n\
                                            ......\n\
                                            ..###.\n\
                                            ......\n\
                                            .#....\n\
                                            .#....",

                                           "...#..\n\
                                            ......\n\
                                            ...#..\n\
                                            ..##..\n\
                                            ......\n\
                                            ......",

                                           "......\n\
                                            ......\n\
                                            ..##..\n\
                                            ..##..\n\
                                            ......\n\
                                            ......"];

    const TEST_INPUT_LIGHT_COUNT: [u32; 5] = [15, 11, 8, 4, 4];

    const TEST_INPUT_B: [&'static str; 6] = ["##.#.#\n\
                                              ...##.\n\
                                              #....#\n\
                                              ..#...\n\
                                              #.#..#\n\
                                              ####.#",

                                             "#.##.#\n\
                                              ####.#\n\
                                              ...##.\n\
                                              ......\n\
                                              #...#.\n\
                                              #.####",

                                             "#..#.#\n\
                                              #....#\n\
                                              .#.##.\n\
                                              ...##.\n\
                                              .#..##\n\
                                              ##.###",

                                             "#...##\n\
                                              ####.#\n\
                                              ..##.#\n\
                                              ......\n\
                                              ##....\n\
                                              ####.#",

                                             "#.####\n\
                                              #....#\n\
                                              ...#..\n\
                                              .##...\n\
                                              #.....\n\
                                              #.#..#",

                                             "##.###\n\
                                              .##..#\n\
                                              .##...\n\
                                              .##...\n\
                                              #.#...\n\
                                              ##...#"];

    #[test]
    fn parse_test() {

        let lights = Lights::from_str(TEST_INPUT[0]).unwrap();

        let expected = vec![vec![false, true, false, true, false, true],
                            vec![false, false, false, true, true, false],
                            vec![true, false, false, false, false, true],
                            vec![false, false, true, false, false, false],
                            vec![true, false, true, false, false, true],
                            vec![true, true, true, true, false, false]];

        assert_eq!(lights.current, expected);

        let lights_not_rectangular = Lights::from_str(TEST_INPUT_NOT_RECTANGULAR);

        assert_eq!(lights_not_rectangular.unwrap_err(), LightParseError);


        let lights_invalid_char = Lights::from_str(TEST_INPUT_INVALID_CHAR);

        assert_eq!(lights_invalid_char.unwrap_err(), LightParseError);
    }

    #[test]
    fn display_test() {

        for state in TEST_INPUT.iter() {
            let lights = Lights::from_str(state).unwrap();

            assert_eq!(lights.to_string().trim(), *state);
        }
    }

    #[test]
    fn num_lights_on_test() {

        for (i, state) in TEST_INPUT.iter().enumerate() {
            let lights = Lights::from_str(state).unwrap();

            assert_eq!(lights.get_num_lights_on(), TEST_INPUT_LIGHT_COUNT[i]);
        }
    }

    #[test]
    fn state_step_test() {

        let mut lights = Lights::from_str(TEST_INPUT[0]).unwrap();

        for state in TEST_INPUT.iter() {
            assert_eq!(lights.to_string().trim(), *state);
            lights.step();
        }
    }

    #[test]
    fn state_step_b_test() {
        let mut lights = Lights::from_str(TEST_INPUT_B[0]).unwrap();

        for state in TEST_INPUT_B.iter() {
            assert_eq!(lights.to_string().trim(), *state);
            lights.step_b();
        }
    }
}
