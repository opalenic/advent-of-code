use std::collections::HashMap;
use std::collections::HashSet;

use std::str::FromStr;

use std::ops::{Deref, DerefMut};

use std::io;
use std::io::prelude::*;

use regex::Regex;

use std::num::ParseIntError;

use lazy_static::lazy_static;

#[derive(Debug)]
struct AocError(String);

#[derive(Debug, PartialEq, Eq)]
struct Patch {
    loc: (usize, usize),
    size: (usize, usize),
}

#[derive(Debug, PartialEq, Eq)]
struct Claims(HashMap<usize, Patch>);

lazy_static! {
    static ref CLAIMS_RE: Regex = Regex::new("^#(?P<id>[0-9]+) @ (?P<loc_x>[0-9]+),(?P<loc_y>[0-9]+): (?P<size_x>[0-9]+)x(?P<size_y>[0-9]+)$").unwrap();
}

impl FromStr for Claims {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut claims = Claims(HashMap::new());

        for line in input.lines() {
            let caps = CLAIMS_RE
                .captures(line)
                .ok_or_else(|| AocError(format!("Invalid input: {:?}", line)))?;

            let id = caps
                .name("id")
                .ok_or_else(|| AocError(format!("Missing claim ID in input: {:?}", line)))?
                .as_str()
                .parse()
                .map_err(|e: ParseIntError| AocError(e.to_string()))?;

            let loc = (
                caps.name("loc_x")
                    .ok_or_else(|| AocError(format!("Missing location X in input: {:?}", line)))?
                    .as_str()
                    .parse()
                    .map_err(|e: ParseIntError| AocError(e.to_string()))?,
                caps.name("loc_y")
                    .ok_or_else(|| AocError(format!("Missing location Y in input: {:?}", line)))?
                    .as_str()
                    .parse()
                    .map_err(|e: ParseIntError| AocError(e.to_string()))?,
            );

            let size = (
                caps.name("size_x")
                    .ok_or_else(|| AocError(format!("Missing size X in input: {:?}", line)))?
                    .as_str()
                    .parse()
                    .map_err(|e: ParseIntError| AocError(e.to_string()))?,
                caps.name("size_y")
                    .ok_or_else(|| AocError(format!("Missing size Y in input: {:?}", line)))?
                    .as_str()
                    .parse()
                    .map_err(|e: ParseIntError| AocError(e.to_string()))?,
            );

            claims.insert(id, Patch { loc, size });
        }

        Ok(claims)
    }
}

impl Deref for Claims {
    type Target = HashMap<usize, Patch>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Claims {
    fn deref_mut(&mut self) -> &mut HashMap<usize, Patch> {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Fabric(HashMap<(usize, usize), HashSet<usize>>);

impl Fabric {
    fn new() -> Fabric {
        Fabric(HashMap::new())
    }

    fn add_patches(&mut self, claims: &Claims) {
        for (id, patch) in claims.iter() {
            for x in (patch.loc.0)..(patch.loc.0 + patch.size.0) {
                for y in (patch.loc.1)..(patch.loc.1 + patch.size.1) {
                    self.entry((x, y)).or_insert_with(HashSet::new).insert(*id);
                }
            }
        }
    }

    fn multiple_claim_count(&self) -> usize {
        self.values()
            .map(|set| set.len())
            .filter(|count| *count >= 2)
            .count()
    }

    fn first_intact_claim_id(&self, claims: &Claims) -> Option<usize> {
        for (id, patch) in claims.iter() {
            let mut patch_intact = true;

            'patch_loop: for x in (patch.loc.0)..(patch.loc.0 + patch.size.0) {
                for y in (patch.loc.1)..(patch.loc.1 + patch.size.1) {
                    if let Some(set) = self.get(&(x, y)) {
                        if set.len() != 1 {
                            patch_intact = false;
                            break 'patch_loop;
                        }
                    }
                }
            }

            if patch_intact {
                return Some(*id);
            }
        }

        None
    }
}

impl Deref for Fabric {
    type Target = HashMap<(usize, usize), HashSet<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fabric {
    fn deref_mut(&mut self) -> &mut HashMap<(usize, usize), HashSet<usize>> {
        &mut self.0
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let claims = input_str.parse().expect("could not parse input");

    let mut fabric = Fabric::new();
    fabric.add_patches(&claims);

    println!(
        "The number of locations claimed by multiple patches is: {}",
        fabric.multiple_claim_count()
    );

    println!(
        "The first claim that is intact is: {}",
        fabric
            .first_intact_claim_id(&claims)
            .expect("no intact claims")
    );
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    use super::Claims;
    use super::Fabric;
    use super::Patch;

    const TEST_INPUT: &str = "#1 @ 1,3: 4x4\n\
                              #2 @ 3,1: 4x4\n\
                              #3 @ 5,5: 2x2";

    lazy_static! {
        static ref EXPECTED_CLAIMS: Claims = {
            let mut expected = Claims(HashMap::new());

            expected.insert(
                1,
                Patch {
                    loc: (1, 3),
                    size: (4, 4),
                },
            );
            expected.insert(
                2,
                Patch {
                    loc: (3, 1),
                    size: (4, 4),
                },
            );
            expected.insert(
                3,
                Patch {
                    loc: (5, 5),
                    size: (2, 2),
                },
            );

            expected
        };
    }

    #[test]
    fn claim_test() {
        let claims: Claims = TEST_INPUT.parse().unwrap();

        assert_eq!(*EXPECTED_CLAIMS, claims);

        let mut fabric = Fabric::new();
        fabric.add_patches(&claims);

        assert_eq!(4, fabric.multiple_claim_count());

        assert_eq!(Some(3), fabric.first_intact_claim_id(&claims));
    }
}
