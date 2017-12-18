extern crate regex;

use std::io;
use std::io::prelude::*;

use regex::Regex;

use std::str::FromStr;

use std::collections::HashSet;

#[derive(Debug)]
struct ProgramGroups {
    groups: Vec<HashSet<usize>>,
}

impl FromStr for ProgramGroups {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        fn merge_groups(grp: &mut HashSet<usize>, groups: &mut Vec<HashSet<usize>>) -> bool {
            for i in 0..groups.len() {
                if grp.intersection(&groups[i]).count() != 0 {

                    grp.extend(groups.remove(i).into_iter());
                    return false;
                }
            }

            return true;
        }


        let mut groups = Vec::new();
        let re = Regex::new(r"(?P<prog>[0-9]+) <-> (?P<other_progs>[0-9]+(?:, [0-9]+)*)").unwrap();

        for line in s.lines() {
            let caps = re.captures(line).ok_or(())?;
            let prog: usize = caps.name("prog").ok_or(())?.as_str().parse().map_err(
                |_| (),
            )?;

            let mut progs = caps.name("other_progs")
                .ok_or(())?
                .as_str()
                .split(",")
                .map(|prog_str| prog_str.trim().parse().map_err(|_| ()))
                .collect::<Result<HashSet<usize>, ()>>()?;

            progs.insert(prog);

            groups.push(progs);
        }

        let mut out = Vec::new();


        // Take group
        // Check each remaining group for intersection
        // If there is an intersection, remove the group, do a union, and start iterating from the start
        // If there are no intersections, push the group into the output vec, and take the next one.

        while let Some(mut grp) = groups.pop() {
            while !merge_groups(&mut grp, &mut groups) {}

            out.push(grp)
        }


        Ok(ProgramGroups { groups: out })
    }
}

impl ProgramGroups {
    fn get_num_programs_in_group(&self, program: usize) -> Option<usize> {
        Some(self.groups.iter().find(|grp| grp.contains(&program))?.iter().count())
    }

    fn get_group_count(&self) -> usize {
        self.groups.len()
    }
}


fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();


    let prog_groups: ProgramGroups = input_str.parse().expect("parse error");

    println!("The group containing program 0 contains {} programs.", prog_groups.get_num_programs_in_group(0).expect("no such program"));
    println!("There are {} groups in totat.", prog_groups.get_group_count());
}


#[cfg(test)]
mod tests {
    use super::ProgramGroups;

    const TEST_INPUT: &str = "0 <-> 2\n\
                              1 <-> 1\n\
                              2 <-> 0, 3, 4\n\
                              3 <-> 2, 4\n\
                              4 <-> 2, 3, 6\n\
                              5 <-> 6\n\
                              6 <-> 4, 5";

    #[test]
    fn program_groups_test() {
        let prg_group: ProgramGroups = TEST_INPUT.parse().expect("parse error");
        println!("{:?}", prg_group);

        assert_eq!(Some(6), prg_group.get_num_programs_in_group(0));
        assert_eq!(Some(1), prg_group.get_num_programs_in_group(1));
        assert_eq!(None, prg_group.get_num_programs_in_group(10));

        assert_eq!(2, prg_group.get_group_count());
    }
}
