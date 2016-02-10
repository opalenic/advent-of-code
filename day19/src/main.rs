
extern crate regex;

use regex::Regex;

use std::io;
use std::io::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;


fn parse_input<T: BufRead>(input: T) -> (HashMap<String, HashSet<Vec<String>>>, Vec<String>) {

    let transform_re = Regex::new("^(([A-Z][a-z]?)|e) => ([A-Z][a-z]?)+$").unwrap();
    let molecule_re = Regex::new("^([A-Z][a-z]?)+$").unwrap();

    let atoms_re = Regex::new("([A-Z][a-z]?)").unwrap();

    let mut transforms = HashMap::new();
    let mut molecule_vec = Vec::new();

    for wline in input.lines() {
        let line = wline.unwrap();

        if let Some(transform) = transform_re.captures(&line) {
            let source_atom = transform.at(1).unwrap();

            let mut output_molecule = Vec::new();

            let (_, endpos) = transform.pos(1).unwrap();
            let (_, rest) = line.split_at(endpos);

            for atom in atoms_re.captures_iter(&rest) {
                output_molecule.push(atom.at(0).unwrap().to_string());
            }

            let mut output_molecules = transforms.entry(source_atom.to_string())
                                                 .or_insert(HashSet::new());
            output_molecules.insert(output_molecule);
        }

        if molecule_re.is_match(&line) {
            if molecule_vec.len() != 0 {
                panic!("Multiple occureces of calibration molecule.");
            }

            for atom in atoms_re.captures_iter(&line) {
                molecule_vec.push(atom.at(0).unwrap().to_string());
            }
        }

    }

    (transforms, molecule_vec)
}

fn apply_transforms(transform_map: &HashMap<String, HashSet<Vec<String>>>,
                    molecule: &Vec<String>)
                    -> HashSet<Vec<String>> {

    let mut possible_molecules = HashSet::new();

    for (pos, atom) in molecule.iter().enumerate() {

        if let Some(transforms) = transform_map.get(atom) {
            for transform in transforms {
                possible_molecules.insert(molecule.iter()
                                                  .take(pos)
                                                  .cloned()
                                                  .chain(transform.iter().cloned())
                                                  .chain(molecule.iter().skip(pos + 1).cloned())
                                                  .collect::<Vec<String>>());
            }
        }
    }

    possible_molecules
}

// Unfortunately this is really slow. But it works for short lenghts of "cure".
fn steps_to_cure(transform_map: &HashMap<String, HashSet<Vec<String>>>,
                 cure: &Vec<String>,
                 current: &HashSet<Vec<String>>,
                 acc: u32)
                 -> u32 {
    println!("At level {}.", acc);
    let mut next = HashSet::with_capacity(current.len());

    for molecule in current {
        let possible = apply_transforms(transform_map, molecule);
        next = next.union(&possible).cloned().collect();
    }

    println!("{:?}", current.len());
    if next.contains(cure) {
        return acc;
    } else {
        return steps_to_cure(transform_map, cure, &next, acc + 1);
    }
}


fn main() {
    let stdin = io::stdin();

    let (transforms, calib_molecule) = parse_input(stdin.lock());

    let possible = apply_transforms(&transforms, &calib_molecule);

    println!("There are {} possible distinct molecules after applying the transforms.", possible.len());


    // Simply copied askalski's solution from Reddit. :-(
    let num_elements = calib_molecule.len();
    let num_parenths = calib_molecule.iter().filter(|atom| *atom == "Rn" || *atom == "Ar").count();
    let num_commas = calib_molecule.iter().filter(|atom| *atom == "Y").count();

    println!("The cure can be synthesized in {} steps.", num_elements - num_parenths - 2 * num_commas - 1);

}


#[cfg(test)]
mod tests {

    use super::parse_input;
    use super::apply_transforms;
    use super::steps_to_cure;

    use std::collections::{HashMap, HashSet};

    const TEST_INPUT_1: &'static str = "H => HO\n\
                                        H => OH\n\
                                        O => HH\n\
                                        Ca => PbHC\n\
                                        HHCaOPb";

    const TEST_INPUT_2: &'static str = "H => HO\n\
                                        H => OH\n\
                                        O => HH\n\
                                        HOH";

    const TEST_INPUT_3: &'static str = "e => H\n\
                                        e => O\n\
                                        H => HO\n\
                                        H => OH\n\
                                        O => HH\n\
                                        HOH";

    const TEST_INPUT_4: &'static str = "e => H\n\
                                        e => O\n\
                                        H => HO\n\
                                        H => OH\n\
                                        O => HH\n\
                                        HOHOHO";

    #[test]
    fn parse_test() {
        let mut ex_transforms = HashMap::new();

        let mut h_transforms = HashSet::new();
        h_transforms.insert(vec!["H".to_string(), "O".to_string()]);
        h_transforms.insert(vec!["O".to_string(), "H".to_string()]);

        ex_transforms.insert("H".to_string(), h_transforms);

        let mut o_transforms = HashSet::new();
        o_transforms.insert(vec!["H".to_string(), "H".to_string()]);

        ex_transforms.insert("O".to_string(), o_transforms);

        let mut ca_transforms = HashSet::new();
        ca_transforms.insert(vec!["Pb".to_string(), "H".to_string(), "C".to_string()]);

        ex_transforms.insert("Ca".to_string(), ca_transforms);


        let ex_molecule = vec!["H".to_string(),
                               "H".to_string(),
                               "Ca".to_string(),
                               "O".to_string(),
                               "Pb".to_string()];


        let (transforms, molecule) = parse_input(TEST_INPUT_1.as_ref());

        assert_eq!(ex_transforms, transforms);
        assert_eq!(ex_molecule, molecule);
    }

    #[test]
    fn possible_molecules_test() {
        let mut ex_possible = HashSet::new();

        ex_possible.insert(vec!["H".to_string(), "O".to_string(), "O".to_string(), "H".to_string()]);
        ex_possible.insert(vec!["H".to_string(), "O".to_string(), "H".to_string(), "O".to_string()]);
        ex_possible.insert(vec!["O".to_string(), "H".to_string(), "O".to_string(), "H".to_string()]);
        ex_possible.insert(vec!["H".to_string(), "H".to_string(), "H".to_string(), "H".to_string()]);

        let (transforms, molecule) = parse_input(TEST_INPUT_2.as_ref());

        let possible = apply_transforms(&transforms, &molecule);

        assert_eq!(ex_possible, possible);
    }

    #[test]
    fn find_cure_test() {
        let (transforms, molecule) = parse_input(TEST_INPUT_3.as_ref());

        let mut current = HashSet::new();
        current.insert(vec!["e".to_string()]);

        assert_eq!(3, steps_to_cure(&transforms, &molecule, &current, 1));
    }

    #[test]
    fn find_cure_test_2() {
        let (transforms, molecule) = parse_input(TEST_INPUT_4.as_ref());

        let mut current = HashSet::new();
        current.insert(vec!["e".to_string()]);

        assert_eq!(6, steps_to_cure(&transforms, &molecule, &current, 1));
    }
}
