extern crate regex;


use std::env::args;

use regex::Regex;

use std::io;
use std::io::prelude::*;

#[derive(PartialEq, Eq, Debug)]
struct Reindeer {
    name: String,
    speed_in_km_s: u32,
    move_time_in_s: u32,
    rest_time_in_s: u32,
    curr_pos: u32,
    points: u32
}

impl Reindeer {
    fn from_str(input_str: &str) -> Reindeer {
        let re = Regex::new("^(?P<name>[:alpha:]+) can fly (?P<speed>[:digit:]+) km/s for (?P<move_dur>[:digit:]+) seconds, but then must rest for (?P<rest_dur>[:digit:]+) seconds.$").unwrap();

        let caps = re.captures(input_str).unwrap();

        let name = caps.name("name").unwrap().to_string();
        let speed = caps.name("speed").unwrap().parse::<u32>().unwrap();
        let move_time = caps.name("move_dur").unwrap().parse::<u32>().unwrap();
        let rest_time = caps.name("rest_dur").unwrap().parse::<u32>().unwrap();

        Reindeer { name: name, speed_in_km_s: speed, move_time_in_s: move_time, rest_time_in_s: rest_time, curr_pos: 0, points: 0 }
    }

    fn get_pos_after(&self, num_s: u32) -> u32 {

        let cycle_time = self.move_time_in_s + self.rest_time_in_s;

        let num_whole_cycles = num_s / cycle_time;

        let time_in_last_cycle = num_s % cycle_time;
        let move_time_in_last_cycle = std::cmp::min(self.move_time_in_s, time_in_last_cycle);

        (num_whole_cycles * self.move_time_in_s + move_time_in_last_cycle) * self.speed_in_km_s
    }

    fn increment_pos(&mut self, curr_time: u32) {
        let cycle_time = self.move_time_in_s + self.rest_time_in_s;

        if (curr_time % cycle_time) < self.move_time_in_s {
            self.curr_pos += self.speed_in_km_s
        }

        self.curr_pos;
    }

    fn add_point_if_leading(&mut self, lead_pos: u32) {
        if self.curr_pos >= lead_pos {
            self.points += 1;
        }
    }
}


fn main() {

    let mut a = args();

    a.next(); // The first argument is the binary name/path
    let race_duration = a.next().unwrap().parse::<u32>().unwrap();


    let stdin = io::stdin();

    let mut reindeer = Vec::new();

    for wline in stdin.lock().lines() {
        let line = wline.unwrap();

        reindeer.push(Reindeer::from_str(&line));
    }


    {
        let mut max_dist = 0;
        let mut max_reindeer: Option<&Reindeer> = None;

        for r in &reindeer {
            let dist = r.get_pos_after(race_duration);

            if dist > max_dist {
                max_dist = dist;
                max_reindeer = Some(r);
            }
        }

        if let Some(rd) = max_reindeer {
            println!("{} has traveled the furthest distance - {} km.", rd.name, max_dist);
        } else {
            panic!();
        }
    }

    println!("---------------------------------");

    for curr_time in 0..race_duration {

        let mut curr_max_pos = 0;

        for r in reindeer.iter_mut() {
            r.increment_pos(curr_time);

            if r.curr_pos > curr_max_pos {
                curr_max_pos = r.curr_pos;
            }
        }


        for r in reindeer.iter_mut() {
            r.add_point_if_leading(curr_max_pos);
        }
    }

    println!("Die Punktestand:");
    for r in &reindeer {
        println!("{} - {}", r.name, r.points);
    }
}

#[cfg(test)]
mod tests {
    use super::Reindeer;
    use std::cmp;

    const TEST_INPUT: &'static str =
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n\
             Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.\n";


    #[test]
    fn input_parse_test() {
        let comet = Reindeer { name: "Comet".to_string(), speed_in_km_s: 14, move_time_in_s: 10, rest_time_in_s: 127, curr_pos: 0, points: 0 };
        let dancer = Reindeer { name: "Dancer".to_string(), speed_in_km_s: 16, move_time_in_s: 11, rest_time_in_s: 162, curr_pos: 0, points: 0 };

        let mut input_iter = TEST_INPUT.lines();

        let parsed_comet = Reindeer::from_str(input_iter.next().unwrap());
        let parsed_dancer = Reindeer::from_str(input_iter.next().unwrap());

        assert_eq!(comet, parsed_comet);
        assert_eq!(dancer, parsed_dancer);
    }

    #[test]
    fn position_calc_test() {
        let comet = Reindeer { name: "Comet".to_string(), speed_in_km_s: 14, move_time_in_s: 10, rest_time_in_s: 127, curr_pos: 0, points: 0 };
        let dancer = Reindeer { name: "Dancer".to_string(), speed_in_km_s: 16, move_time_in_s: 11, rest_time_in_s: 162, curr_pos: 0, points: 0 };

        assert_eq!(1120, comet.get_pos_after(1000));
        assert_eq!(1056, dancer.get_pos_after(1000));
    }

    #[test]
    fn points_calc_test() {
        let mut comet = Reindeer { name: "Comet".to_string(), speed_in_km_s: 14, move_time_in_s: 10, rest_time_in_s: 127, curr_pos: 0, points: 0 };
        let mut dancer = Reindeer { name: "Dancer".to_string(), speed_in_km_s: 16, move_time_in_s: 11, rest_time_in_s: 162, curr_pos: 0, points: 0 };


        for curr_time in 0..1000 {
            comet.increment_pos(curr_time);
            dancer.increment_pos(curr_time);

            let curr_max_pos = cmp::max(comet.curr_pos, dancer.curr_pos);

            comet.add_point_if_leading(curr_max_pos);
            dancer.add_point_if_leading(curr_max_pos);
        }

        assert_eq!(312, comet.points);
        assert_eq!(689, dancer.points);
    }
}