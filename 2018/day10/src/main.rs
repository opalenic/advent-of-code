use std::str::FromStr;

use std::fmt::Display;
use std::fmt;

use std::io;
use std::io::prelude::*;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct AocError(String);

lazy_static! {
    static ref LIGHT_RE: Regex = Regex::new(r"^position=<\s*(?P<pos_x>-?[0-9]+),\s*(?P<pos_y>-?[0-9]+)> velocity=<\s*(?P<vel_x>-?[0-9]+),\s*(?P<vel_y>-?[0-9]+)>$").unwrap();
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Light {
    pos: (isize, isize),
    velocity: (isize, isize),
}

impl FromStr for Light {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = LIGHT_RE
            .captures(s.trim())
            .ok_or_else(|| AocError(format!("Invalid input: {:?}", s)))?;

        let pos_x = caps.name("pos_x").unwrap().as_str().parse().unwrap();
        let pos_y = caps.name("pos_y").unwrap().as_str().parse().unwrap();

        let vel_x = caps.name("vel_x").unwrap().as_str().parse().unwrap();
        let vel_y = caps.name("vel_y").unwrap().as_str().parse().unwrap();

        Ok(Light {
            pos: (pos_x, pos_y),
            velocity: (vel_x, vel_y),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Lights(Vec<Light>);

impl FromStr for Lights {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Lights(
            s.lines()
                .map(|line| line.parse())
                .collect::<Result<Vec<Light>, AocError>>()?,
        ))
    }
}

impl Display for Lights {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();

        if let Some((min, max)) = self.find_pos_min_max() {
            let size_x = max.0 - min.0;
            let size_y = max.1 - min.1;

            for y in 0..=size_y {
                for x in 0..=size_x {
                    let ch = if self.0.iter().filter(|light| light.pos == (min.0 + x, min.1 + y)).next().is_some() { '#' } else { '.' };
                    out.push(ch);
                }

                out.push('\n');
            }
        }

        formatter.write_str(&out)
    }
}

impl Lights {
    fn do_step(&mut self) {
        self.0.iter_mut().for_each(|light| {
            light.pos.0 += light.velocity.0;
            light.pos.1 += light.velocity.1;
        });
    }

    fn do_step_reverse(&mut self) {
        self.0.iter_mut().for_each(|light| {
            light.pos.0 -= light.velocity.0;
            light.pos.1 -= light.velocity.1;
        });
    }

    fn find_pos_min_max(&self) -> Option<((isize, isize), (isize, isize))> {
        let mut min_max = None;

        for light in &self.0 {
            min_max = if let Some(mm) = min_max {
                let mut new_mm: ((isize, isize), (isize, isize)) = mm;
                if light.pos.0 < (mm.0).0 {
                    (new_mm.0).0 = light.pos.0;
                }

                if light.pos.1 < (mm.0).1 {
                    (new_mm.0).1 = light.pos.1;
                }

                if light.pos.0 > (mm.1).0 {
                    (new_mm.1).0 = light.pos.0;
                }

                if light.pos.1 > (mm.1).1 {
                    (new_mm.1).1 = light.pos.1;
                }

                Some(new_mm)
            } else {
                Some((light.pos, light.pos))
            }
        }

        min_max
    }

    fn run_until_local_minimum(&mut self) -> Result<usize, AocError> {
        let (prev_min, prev_max) = self.find_pos_min_max().ok_or_else(|| AocError("no lights defined".into()))?;

        let mut prev_size_x = prev_max.0 - prev_min.0;
        let mut prev_size_y = prev_max.1 - prev_min.1;

        for i in 1.. {
            self.do_step();

            let (curr_min, curr_max) = self.find_pos_min_max().expect("can't get min and max light positions");

            let curr_size_x = curr_max.0 - curr_min.0;
            let curr_size_y = curr_max.1 - curr_min.1;

            if curr_size_x > prev_size_x || curr_size_y > prev_size_y {
                self.do_step_reverse();

                return Ok(i - 1);
            }

            prev_size_x = curr_size_x;
            prev_size_y = curr_size_y;
        }

        unreachable!();
    }
}




fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let mut lights: Lights = input_str.parse().expect("invalid input");

    let num_steps = lights.run_until_local_minimum().unwrap();

    println!(
        "Local minimum found after {} steps",
        num_steps
    );

    println!("{}", lights);
}


#[cfg(test)]
mod tests {
    use super::Lights;
    use super::Light;

    use lazy_static::lazy_static;

    const INPUT_STR: &str = "position=< 9,  1> velocity=< 0,  2>\n\
                             position=< 7,  0> velocity=<-1,  0>\n\
                             position=< 3, -2> velocity=<-1,  1>\n\
                             position=< 6, 10> velocity=<-2, -1>\n\
                             position=< 2, -4> velocity=< 2,  2>\n\
                             position=<-6, 10> velocity=< 2, -2>\n\
                             position=< 1,  8> velocity=< 1, -1>\n\
                             position=< 1,  7> velocity=< 1,  0>\n\
                             position=<-3, 11> velocity=< 1, -2>\n\
                             position=< 7,  6> velocity=<-1, -1>\n\
                             position=<-2,  3> velocity=< 1,  0>\n\
                             position=<-4,  3> velocity=< 2,  0>\n\
                             position=<10, -3> velocity=<-1,  1>\n\
                             position=< 5, 11> velocity=< 1, -2>\n\
                             position=< 4,  7> velocity=< 0, -1>\n\
                             position=< 8, -2> velocity=< 0,  1>\n\
                             position=<15,  0> velocity=<-2,  0>\n\
                             position=< 1,  6> velocity=< 1,  0>\n\
                             position=< 8,  9> velocity=< 0, -1>\n\
                             position=< 3,  3> velocity=<-1,  1>\n\
                             position=< 0,  5> velocity=< 0, -1>\n\
                             position=<-2,  2> velocity=< 2,  0>\n\
                             position=< 5, -2> velocity=< 1,  2>\n\
                             position=< 1,  4> velocity=< 2,  1>\n\
                             position=<-2,  7> velocity=< 2, -2>\n\
                             position=< 3,  6> velocity=<-1, -1>\n\
                             position=< 5,  0> velocity=< 1,  0>\n\
                             position=<-6,  0> velocity=< 2,  0>\n\
                             position=< 5,  9> velocity=< 1, -2>\n\
                             position=<14,  7> velocity=<-2,  0>\n\
                             position=<-3,  6> velocity=< 2, -1>";

    const EXPECTED_STATES: [&str; 5] = [
        "........#.............\n\
         ................#.....\n\
         .........#.#..#.......\n\
         ......................\n\
         #..........#.#.......#\n\
         ...............#......\n\
         ....#.................\n\
         ..#.#....#............\n\
         .......#..............\n\
         ......#...............\n\
         ...#...#.#...#........\n\
         ....#..#..#.........#.\n\
         .......#..............\n\
         ...........#..#.......\n\
         #...........#.........\n\
         ...#.......#..........\n",
        "........#....#....\n\
         ......#.....#.....\n\
         #.........#......#\n\
         ..................\n\
         ....#.............\n\
         ..##.........#....\n\
         ....#.#...........\n\
         ...##.##..#.......\n\
         ......#.#.........\n\
         ......#...#.....#.\n\
         #...........#.....\n\
         ..#.....#.#.......\n",
        "..........#...\n\
         #..#...####..#\n\
         ..............\n\
         ....#....#....\n\
         ..#.#.........\n\
         ...#...#......\n\
         ...#..#..#.#..\n\
         #....#.#......\n\
         .#...#...##.#.\n\
         ....#.........\n",
        "#...#..###\n\
         #...#...#.\n\
         #...#...#.\n\
         #####...#.\n\
         #...#...#.\n\
         #...#...#.\n\
         #...#...#.\n\
         #...#..###\n",
        "........#....\n\
         ....##...#.#.\n\
         ..#.....#..#.\n\
         .#..##.##.#..\n\
         ...##.#....#.\n\
         .......#....#\n\
         ..........#..\n\
         #......#...#.\n\
         .#.....##....\n\
         ...........#.\n\
         ...........#.\n"];

    lazy_static! {
        static ref EXPECTED_LIGHTS: Lights = {
            Lights(vec![
                Light { pos: (9, 1), velocity: (0, 2) },
                Light { pos: (7, 0), velocity: (-1, 0) },
                Light { pos: (3, -2), velocity: (-1, 1) },
                Light { pos: (6, 10), velocity: (-2, -1) },
                Light { pos: (2, -4), velocity: (2, 2) },
                Light { pos: (-6, 10), velocity: (2, -2) },
                Light { pos: (1, 8), velocity: (1, -1) },
                Light { pos: (1, 7), velocity: (1, 0) },
                Light { pos: (-3, 11), velocity: (1, -2) },
                Light { pos: (7, 6), velocity: (-1, -1) },
                Light { pos: (-2, 3), velocity: (1, 0) },
                Light { pos: (-4, 3), velocity: (2, 0) },
                Light { pos: (10, -3), velocity: (-1, 1) },
                Light { pos: (5, 11), velocity: (1, -2) },
                Light { pos: (4, 7), velocity: (0, -1) },
                Light { pos: (8, -2), velocity: (0, 1) },
                Light { pos: (15, 0), velocity: (-2, 0) },
                Light { pos: (1, 6), velocity: (1, 0) },
                Light { pos: (8, 9), velocity: (0, -1) },
                Light { pos: (3, 3), velocity: (-1, 1) },
                Light { pos: (0, 5), velocity: (0, -1) },
                Light { pos: (-2, 2), velocity: (2, 0) },
                Light { pos: (5, -2), velocity: (1, 2) },
                Light { pos: (1, 4), velocity: (2, 1) },
                Light { pos: (-2, 7), velocity: (2, -2) },
                Light { pos: (3, 6), velocity: (-1, -1) },
                Light { pos: (5, 0), velocity: (1, 0) },
                Light { pos: (-6, 0), velocity: (2, 0) },
                Light { pos: (5, 9), velocity: (1, -2) },
                Light { pos: (14, 7), velocity: (-2, 0) },
                Light { pos: (-3, 6), velocity: (2, -1) }
            ])
        };
    }

    #[test]
    fn parse_test() {
        let lights: Lights = INPUT_STR.parse().unwrap();

        assert_eq!(*EXPECTED_LIGHTS, lights);
    }

    #[test]
    fn simualtion_step_test() {
        let mut lights = EXPECTED_LIGHTS.clone();

        assert_eq!(EXPECTED_STATES[0], &format!("{}", lights));

        lights.do_step();
        assert_eq!(EXPECTED_STATES[1], &format!("{}", lights));

        lights.do_step();
        assert_eq!(EXPECTED_STATES[2], &format!("{}", lights));

        lights.do_step();
        assert_eq!(EXPECTED_STATES[3], &format!("{}", lights));

        lights.do_step();
        assert_eq!(EXPECTED_STATES[4], &format!("{}", lights));

        lights.do_step_reverse();
        assert_eq!(EXPECTED_STATES[3], &format!("{}", lights));

        lights.do_step_reverse();
        assert_eq!(EXPECTED_STATES[2], &format!("{}", lights));

        lights.do_step();
        assert_eq!(EXPECTED_STATES[3], &format!("{}", lights));
    }

    #[test]
    fn local_min_search_test() {
        let mut lights = EXPECTED_LIGHTS.clone();

        let num_steps = lights.run_until_local_minimum().unwrap();
        assert_eq!(3, num_steps);

        assert_eq!(EXPECTED_STATES[3], &format!("{}", lights));
    }

}