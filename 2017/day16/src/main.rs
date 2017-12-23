
extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

use std::str::FromStr;
use std::string::ToString;

use std::io;
use std::io::prelude::*;

use std::collections::BTreeMap;

lazy_static! {
    static ref SPIN_RE: Regex = Regex::new(r"^s(?P<spin_by>[0-9]+)$").unwrap();
    static ref EXCHANGE_RE: Regex = Regex::new(r"^x(?P<exchange_a>[0-9]+)/(?P<exchange_b>[0-9]+)$").unwrap();
    static ref PARTNER_RE: Regex = Regex::new(r"^p(?P<partner_a>[a-z]+)/(?P<partner_b>[a-z]+)$").unwrap();
}


#[derive(Debug, PartialEq, Eq)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for DanceMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = SPIN_RE.captures(s) {
            let spin_by = caps.name("spin_by").ok_or(())?.as_str().parse().map_err(
                |_| (),
            )?;

            Ok(DanceMove::Spin(spin_by))
        } else if let Some(caps) = EXCHANGE_RE.captures(s) {
            let a = caps.name("exchange_a")
                .ok_or(())?
                .as_str()
                .parse()
                .map_err(|_| ())?;
            let b = caps.name("exchange_b")
                .ok_or(())?
                .as_str()
                .parse()
                .map_err(|_| ())?;

            Ok(DanceMove::Exchange(a, b))
        } else if let Some(caps) = PARTNER_RE.captures(s) {
            let a = caps.name("partner_a")
                .ok_or(())?
                .as_str()
                .chars()
                .next()
                .ok_or(())?;

            let b = caps.name("partner_b")
                .ok_or(())?
                .as_str()
                .chars()
                .next()
                .ok_or(())?;

            Ok(DanceMove::Partner(a, b))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct Programs {
    programs: Vec<char>,
}

impl Programs {
    fn new(len: u8) -> Programs {
        let mut programs = Vec::with_capacity(len as usize);

        for ch_offset in 0..len {
            programs.push((('a' as u8) + ch_offset) as char);
        }

        Programs { programs }
    }

    fn dance(&mut self, dance: &Dance) -> Result<(), ()> {
        for mv in &dance.0 {
            match *mv {
                DanceMove::Spin(spin_by) => {
                    if spin_by > self.programs.len() {
                        return Err(());
                    }

                    let mut tmp = Vec::new();

                    {
                        let pivot = self.programs.len() - spin_by;

                        let end = &self.programs[pivot..];
                        let front = &self.programs[0..pivot];

                        tmp.extend(end.iter().cloned());
                        tmp.extend(front.iter().cloned());
                    }

                    self.programs = tmp;
                }
                DanceMove::Exchange(a, b) => {
                    if a >= self.programs.len() || b >= self.programs.len() {
                        return Err(());
                    }

                    let tmp = self.programs[a];
                    self.programs[a] = self.programs[b];
                    self.programs[b] = tmp;
                }

                DanceMove::Partner(a, b) => {
                    let (a_pos, _) = self.programs
                        .iter()
                        .enumerate()
                        .find(|&(_, ch)| *ch == a)
                        .ok_or(())?;
                    let (b_pos, _) = self.programs
                        .iter()
                        .enumerate()
                        .find(|&(_, ch)| *ch == b)
                        .ok_or(())?;

                    let tmp = self.programs[a_pos];
                    self.programs[a_pos] = self.programs[b_pos];
                    self.programs[b_pos] = tmp;
                }
            }
        }

        Ok(())
    }

    fn perform_dances(&mut self, dance: &Dance, num_dances: usize) -> Result<(), ()> {
        let mut states_seen = BTreeMap::new();

        let mut i = 0;

        while i < num_dances {
            self.dance(&dance)?;


            if !states_seen.contains_key(&self.programs) {
                states_seen.insert(self.programs.clone(), i);
            } else {
                let last_seen_at_idx = states_seen[&self.programs];
                let skip_size = i - last_seen_at_idx;

                i += ((num_dances - i) / skip_size) * skip_size;
            }

            i += 1;
        }

        Ok(())
    }
}

impl ToString for Programs {
    fn to_string(&self) -> String {
        self.programs.iter().collect()
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Dance(Vec<DanceMove>);

impl FromStr for Dance {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Dance(s.split(",")
            .map(|line| line.trim().parse())
            .collect::<Result<Vec<DanceMove>, ()>>()?))
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let dance = input_str.parse().expect("parse error");

    let mut progs = Programs::new(16);
    progs.dance(&dance).expect("dance error");


    println!("Program order after the first dance: {}", progs.to_string());

    progs.perform_dances(&dance, 999_999_999).unwrap();

    println!(
        "Program order after 1 000 000 000 dances: {}",
        progs.to_string()
    );
}


#[cfg(test)]
mod tests {

    use super::Dance;
    use super::DanceMove;
    use super::Programs;

    const TEST_DANCE: &str = "s1,x3/4,pe/b";

    #[test]
    fn parse_test() {
        assert_eq!(
            Dance(vec![
                DanceMove::Spin(1),
                DanceMove::Exchange(3, 4),
                DanceMove::Partner('e', 'b'),
            ]),
            TEST_DANCE.parse().unwrap()
        );
    }

    #[test]
    fn dance_test() {
        let mut progs = Programs::new(5);

        let dance = TEST_DANCE.parse().unwrap();

        progs.dance(&dance).unwrap();
        assert_eq!("baedc", progs.to_string());

        progs.dance(&dance).unwrap();
        assert_eq!("ceadb", progs.to_string());

        progs.perform_dances(&dance, 30).unwrap();
        assert_eq!("abcde", progs.to_string());
    }
}
