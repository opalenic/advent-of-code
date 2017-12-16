#![feature(entry_and_modify)]

extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

use std::io;
use std::io::prelude::*;
use std::str::FromStr;


#[derive(Debug)]
enum WeightSearch<'a> {
    Subtree(usize, &'a Program),
    OffendingProgram(usize, usize, &'a Program),
}

#[derive(Debug)]
enum Weight<'a> {
    Unique(&'a Program),
    AlreadySeen(usize),
}

#[derive(Debug, PartialEq, Eq)]
struct Program {
    name: String,
    weight: usize,
    child_names: Vec<String>,
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"(?P<prg_name>[a-z]+) \((?P<weight>[0-9]+)\)(?: -> (?P<children>.+))?",
        ).unwrap();
        let re_children = Regex::new(r"([a-z]+)").unwrap();

        let caps = re.captures(&s).ok_or(())?;

        let name = caps.name("prg_name").ok_or(())?.as_str().to_string();
        let weight = caps.name("weight").ok_or(())?.as_str().parse().map_err(
            |_| (),
        )?;

        let child_names = if let Some(children_match) = caps.name("children") {
            children_match
                .as_str()
                .split(",")
                .map(|name_str| {
                    Ok(re_children.find(name_str).ok_or(())?.as_str().to_string())
                })
                .collect::<Result<Vec<String>, ()>>()?
        } else {
            Vec::new()
        };

        Ok(Program {
            name,
            weight,
            child_names,
        })
    }
}

fn get_root_name(programs: &Vec<Program>) -> Result<String, ()> {
    let (all_names, child_names) = programs.iter().fold(
        (HashSet::new(), HashSet::new()),
        |(mut all, mut children), curr_prog| {
            all.insert(&curr_prog.name);
            for child in &curr_prog.child_names {
                children.insert(child);
            }

            (all, children)
        },
    );


    let mut diff = all_names.difference(&child_names);

    let ret = diff.next().ok_or(())?.to_string();

    if diff.next().is_none() {
        Ok(ret)
    } else {
        Err(())
    }
}

fn find_imbalanced_node<'a>(
    programs: &'a Vec<Program>,
    root: &str,
) -> Result<(usize, usize, &'a Program), ()> {

    let prog_map = programs.iter().fold(HashMap::new(), |mut map, curr_prog| {
        map.insert(curr_prog.name.as_str(), curr_prog);
        map
    });

    if let WeightSearch::OffendingProgram(desired_weight, subtree_weight, prog) =
        get_weight_r(prog_map[root], &prog_map)?
    {
        Ok((desired_weight, subtree_weight, prog))
    } else {
        Err(())
    }
}

fn get_weight_r<'a, 'b: 'a>(
    root: &'b Program,
    programs: &'a HashMap<&'b str, &'b Program>,
) -> Result<WeightSearch<'b>, ()> {

    let mut subtree_weights: HashMap<usize, Weight> = HashMap::new();

    for child in &root.child_names {
        match get_weight_r(programs[child.as_str()], programs)? {
            WeightSearch::OffendingProgram(desired_weight, subtree_weight, prog) => {
                return Ok(WeightSearch::OffendingProgram(
                    desired_weight,
                    subtree_weight,
                    prog,
                ));
            }
            WeightSearch::Subtree(subtree_weight, subtree_root) => {

                subtree_weights
                    .entry(subtree_weight)
                    .and_modify(|e| {
                        *e = match *e {
                            Weight::Unique(_) => Weight::AlreadySeen(2),
                            Weight::AlreadySeen(num) => Weight::AlreadySeen(num + 1),
                        }
                    })
                    .or_insert(Weight::Unique(subtree_root));
            }
        }
    }

    if subtree_weights.len() <= 1 {
        let sub_weight = match subtree_weights.iter().next() {
            Some((el_weight, &Weight::Unique(_))) => *el_weight,
            Some((el_weight, &Weight::AlreadySeen(seen_count))) => el_weight * seen_count,
            None => 0,
        };

        Ok(WeightSearch::Subtree(root.weight + sub_weight, root))
    } else if subtree_weights.len() == 2 {
        let mut unique_prog = None;
        let mut unique_subtree_weight = None;
        let mut other_weight = None;

        for w in &subtree_weights {
            match w {
                (el_weight, &Weight::AlreadySeen(_)) => {
                    if other_weight.is_some() {
                        return Err(());
                    }

                    other_weight = Some(*el_weight);
                }
                (el_weight, &Weight::Unique(prog)) => {
                    if unique_prog.is_some() || unique_subtree_weight.is_some() {
                        return Err(());
                    }

                    unique_prog = Some(prog);
                    unique_subtree_weight = Some(*el_weight);
                }
            }
        }

        if let (Some(desired_weight), Some(subtree_weight), Some(prog)) =
            (other_weight, unique_subtree_weight, unique_prog)
        {
            Ok(WeightSearch::OffendingProgram(
                desired_weight,
                subtree_weight,
                prog,
            ))
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let programs = input_str
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Program>, ()>>()
        .expect("parsing error");

    let root_name = get_root_name(&programs).expect("no root");

    println!("The root node is '{}'.", root_name);

    let (desired_weight, subtree_weight, offending_prog) =
        find_imbalanced_node(&programs, &root_name).expect("no imbalanced node");

    println!(
        "Imbalanced node name '{}'. Weighs {}, and should weigh {} to bring the subtree balance to {}.",
        offending_prog.name,
        offending_prog.weight,
        offending_prog.weight + desired_weight - subtree_weight,
        desired_weight
    );
}


#[cfg(test)]
mod tests {

    use super::Program;
    use super::get_root_name;
    use super::find_imbalanced_node;

    const TEST_INPUT: &str = "pbga (66)\n\
                              xhth (57)\n\
                              ebii (61)\n\
                              havc (66)\n\
                              ktlj (57)\n\
                              fwft (72) -> ktlj, cntj, xhth\n\
                              qoyq (66)\n\
                              padx (45) -> pbga, havc, qoyq\n\
                              tknk (41) -> ugml, padx, fwft\n\
                              jptl (61)\n\
                              ugml (68) -> gyxo, ebii, jptl\n\
                              gyxo (61)\n\
                              cntj (57)";

    fn parse_input(input_str: &str) -> Vec<Program> {
        input_str
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Program>, ()>>()
            .expect("parsing error")
    }

    #[test]
    fn find_root_node_test() {
        let programs = parse_input(TEST_INPUT);

        assert_eq!("tknk", get_root_name(&programs).expect("no root node"));
    }

    #[test]
    fn find_imbalanced_node_test() {
        let programs = parse_input(TEST_INPUT);

        let imbalanced_prog = programs.iter().find(|prog| prog.name == "ugml").unwrap();

        assert_eq!(
            (243, 251, imbalanced_prog),
            find_imbalanced_node(&programs, "tknk").expect("no imbalanced node")
        );
    }
}
