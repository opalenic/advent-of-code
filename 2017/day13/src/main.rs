
extern crate regex;

use regex::Regex;

use std::collections::HashMap;

use std::str::FromStr;

use std::io;
use std::io::prelude::*;


#[derive(Debug)]
struct FwLayer {
    len: usize,
}



impl FwLayer {
    fn new(len: usize) -> FwLayer {
        FwLayer {
            len: len,
        }
    }

    fn get_pos_at_time(&self, time: usize) -> usize {
        let pos_rem = time % (self.len - 1);
        let dir_down = (time / (self.len - 1)) % 2 == 0;


        if dir_down {
            pos_rem
        } else {
            self.len - pos_rem - 1
        }
    }

    fn get_depth(&self) -> usize {
        self.len
    }
}


#[derive(Debug)]
struct Firewall {
    layers: HashMap<usize, FwLayer>
}


impl FromStr for Firewall {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("^(?P<layer_pos>[0-9]+): (?P<layer_depth>[0-9]+)$").unwrap();

        let mut layers = HashMap::new();

        for line in s.lines() {
            let caps = re.captures(line).ok_or(())?;

            let layer_pos = caps.name("layer_pos").ok_or(())?.as_str().parse().map_err(|_| ())?;
            let layer_depth = caps.name("layer_depth").ok_or(())?.as_str().parse().map_err(|_| ())?;

            layers.insert(layer_pos, FwLayer::new(layer_depth));
        }

        Ok(Firewall {
            layers
        })
    }
}


impl Firewall {
    fn run_packet(&self, start_time: usize) -> (bool, usize) {
        let num_layers = self.layers.keys().max().expect("no maximum") + 1;

        let mut severity = 0;
        let mut hit = false;

        for i in 0..(num_layers + 1) {
            if let Some(layer) = self.layers.get(&i) {
                if layer.get_pos_at_time(start_time + i) == 0 {
                    hit = true;
                    severity += i * layer.get_depth();
                }
            }
        }

        (hit, severity)
    }
}


fn find_fw_entry_delay(fw: &Firewall) -> usize {
    for start_time in 0.. {
        if !fw.run_packet(start_time).0 {
            return start_time;
        }
    }

    unreachable!();
}


fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();
    let fw: Firewall = input_str.parse().expect("parse error");

    println!("The severity of a packet run at time 0 is {}.", fw.run_packet(0).1);
    println!("The first start time at which a packet is not caught is {}.", find_fw_entry_delay(&fw));
}


#[cfg(test)]
mod tests {
    use super::Firewall;
    use super::find_fw_entry_delay;

    const TEST_INPUT: &str = "0: 3\n\
                              1: 2\n\
                              4: 4\n\
                              6: 4";

    #[test]
    fn parse_test() {
        let fw: Firewall = TEST_INPUT.parse().expect("parse error");

        assert_eq!(24, fw.run_packet(0).1);
    }

    #[test]
    fn catch_test() {
        let fw: Firewall = TEST_INPUT.parse().expect("parse error");

        assert_eq!((false, 0), fw.run_packet(10));

        assert_eq!(10, find_fw_entry_delay(&fw));
    }
}