use lazy_static::lazy_static;
use regex::Regex;

use std::collections::{BTreeMap, BTreeSet, HashSet};

use std::str::FromStr;

use std::io;
use std::io::prelude::*;

lazy_static! {
    static ref PREREQ_RE: Regex = Regex::new(
        r"^Step (?P<prereq_ch>[A-Z]) must be finished before step (?P<curr_ch>[A-Z]) can begin."
    )
    .unwrap();
}

#[derive(Debug)]
struct AocError(String);

#[derive(Debug, PartialEq, Eq)]
struct Prerequisites(BTreeMap<char, HashSet<char>>);

impl FromStr for Prerequisites {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = BTreeMap::new();

        for line in s.lines() {
            let trimmed = line.trim();

            let caps = PREREQ_RE
                .captures(trimmed)
                .ok_or_else(|| AocError(format!("Invalid input: '{}'", trimmed)))?;

            let curr_ch = caps
                .name("curr_ch")
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap();

            let prereq_ch = caps
                .name("prereq_ch")
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap();

            out.entry(curr_ch)
                .or_insert_with(HashSet::new)
                .insert(prereq_ch);
            out.entry(prereq_ch).or_insert_with(HashSet::new);
        }

        Ok(Prerequisites(out))
    }
}

impl Prerequisites {
    fn get_steps_a(&self) -> Result<String, AocError> {
        let mut out = String::new();
        let mut prereqs_fulfilled = HashSet::new();
        let mut open_nodes = self.0.clone();

        while !open_nodes.is_empty() {
            let new_node_ch = *open_nodes
                .iter()
                .filter(|(_, prereqs)| prereqs.is_subset(&prereqs_fulfilled))
                .map(|(ch, _)| ch)
                .next()
                .ok_or_else(|| AocError("Ran out of nodes with fulfilled prerequisites!".into()))?;

            out.push(new_node_ch);
            prereqs_fulfilled.insert(new_node_ch);
            open_nodes.remove(&new_node_ch);
        }

        Ok(out)
    }

    fn get_steps_b(
        &self,
        num_workers: usize,
        base_step_time: usize,
    ) -> Result<(usize, String), AocError> {
        fn get_char_time(ch: char, base_step_time: usize) -> usize {
            if !ch.is_ascii_uppercase() {
                panic!("Invalid char: {:?}", ch);
            }

            ((ch as u8) - b'A') as usize + base_step_time
        }

        let mut free_workers = num_workers;
        let mut out = String::new();
        let mut prereqs_fulfilled = HashSet::new();
        let mut open_nodes = self.0.clone();
        let mut processing_nodes = BTreeMap::new();

        for time in 0.. {
            if let Some(f_n) = processing_nodes.remove(&time) {
                let finished_nodes: BTreeSet<char> = f_n;

                free_workers += finished_nodes.len();

                out.extend(finished_nodes.clone());
                prereqs_fulfilled.extend(finished_nodes);
            }

            if open_nodes.is_empty() && processing_nodes.is_empty() {
                return Ok((time, out));
            }

            let new_nodes = open_nodes
                .iter()
                .filter(|(_, prereqs)| prereqs.is_subset(&prereqs_fulfilled))
                .map(|(ch, _)| ch)
                .take(free_workers)
                .cloned()
                .collect::<Vec<char>>();

            if free_workers > 0 && new_nodes.is_empty() && processing_nodes.is_empty() {
                return Err(AocError(
                    "Ran out of nodes with fulfilled prerequisites!".into(),
                ));
            }

            free_workers -= new_nodes.len();

            for new_node_ch in new_nodes {
                let end_time = time + 1 + get_char_time(new_node_ch, base_step_time);

                processing_nodes
                    .entry(end_time)
                    .or_insert_with(BTreeSet::new)
                    .insert(new_node_ch);
                open_nodes.remove(&new_node_ch);
            }
        }

        unreachable!();
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let prerequisites: Prerequisites = input_str.parse().expect("parse error");

    let step_order = prerequisites.get_steps_a().expect("invalid prerequisites");

    println!("The step order is: {}", step_order);

    let sim = prerequisites
        .get_steps_b(5, 60)
        .expect("invalid prerequisites");

    println!(
        "With 5 workers, the nodes will be processed in {} seconds.",
        sim.0
    );
}

#[cfg(test)]
mod tests {
    use super::Prerequisites;

    use std::collections::{BTreeMap, HashSet};

    use lazy_static::lazy_static;

    const INPUT_STR: &str = "Step C must be finished before step A can begin.\n\
                             Step C must be finished before step F can begin.\n\
                             Step A must be finished before step B can begin.\n\
                             Step A must be finished before step D can begin.\n\
                             Step B must be finished before step E can begin.\n\
                             Step D must be finished before step E can begin.\n\
                             Step F must be finished before step E can begin.";

    lazy_static! {
        static ref EXPECTED: Prerequisites = {
            let mut out = BTreeMap::new();

            let e_prereqs = out.entry('E').or_insert_with(HashSet::new);
            e_prereqs.insert('B');
            e_prereqs.insert('D');
            e_prereqs.insert('F');

            out.entry('D').or_insert_with(HashSet::new).insert('A');

            out.entry('B').or_insert_with(HashSet::new).insert('A');

            out.entry('F').or_insert_with(HashSet::new).insert('C');

            out.entry('A').or_insert_with(HashSet::new).insert('C');

            out.insert('C', HashSet::new());

            Prerequisites(out)
        };
    }

    #[test]
    fn parse_test() {
        let prereqs: Prerequisites = INPUT_STR.parse().unwrap();

        assert_eq!(*EXPECTED, prereqs);
    }

    #[test]
    fn step_order_test() {
        let steps = EXPECTED.get_steps_a().unwrap();

        assert_eq!("CABDFE".to_string(), steps);
    }

    #[test]
    fn worker_sim_test() {
        let time = EXPECTED.get_steps_b(2, 0).unwrap().0;

        assert_eq!(15, time);
    }
}
