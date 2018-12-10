use std::num::ParseIntError;
use std::str::FromStr;

use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct AocError(String);

#[derive(Debug, PartialEq, Eq)]
struct LicenseTree {
    raw_nodes_input: Vec<usize>,
}

impl FromStr for LicenseTree {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_nodes_input = s
            .split_whitespace()
            .map(|val| {
                val.parse::<usize>()
                    .map_err(|e: ParseIntError| AocError(e.to_string()))
            })
            .collect::<Result<Vec<usize>, AocError>>()?;

        Ok(LicenseTree { raw_nodes_input })
    }
}

impl LicenseTree {
    fn get_metadata_sum(&self) -> Result<usize, AocError> {
        fn parse_node(input: &[usize]) -> Result<(usize, usize), AocError> {
            let mut idx = 0;
            let mut metadata_sum = 0;

            let num_nodes = input[idx];
            let num_metadata = input[idx + 1];

            idx += 2;
            for _ in 0..num_nodes {
                let (idx_offset, metadata) = parse_node(&input[idx..])?;

                idx += idx_offset;
                metadata_sum += metadata;
            }

            for i in 0..num_metadata {
                metadata_sum += input[idx + i];
            }

            idx += num_metadata;

            Ok((idx, metadata_sum))
        }

        Ok(parse_node(&self.raw_nodes_input)?.1)
    }

    fn get_root_node_value(&self) -> Result<usize, AocError> {
        fn parse_node(input: &[usize]) -> Result<(usize, usize), AocError> {
            let mut idx = 0;
            let mut metadata_sum = 0;

            let num_nodes = input[idx];
            let num_metadata = input[idx + 1];

            let mut child_sums = Vec::new();

            idx += 2;
            for _ in 0..num_nodes {
                let (idx_offset, metadata) = parse_node(&input[idx..])?;

                idx += idx_offset;
                child_sums.push(metadata);
            }

            if child_sums.is_empty() {
                for i in 0..num_metadata {
                    metadata_sum += input[idx + i];
                }
            } else {
                for i in 0..num_metadata {
                    let metadata_idx = input[idx + i];
                    metadata_sum += child_sums.get(metadata_idx - 1).unwrap_or(&0);
                }
            }

            idx += num_metadata;

            Ok((idx, metadata_sum))
        }

        Ok(parse_node(&self.raw_nodes_input)?.1)
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let license_tree: LicenseTree = input_str.parse().expect("invalid input");

    println!(
        "The sum of all metadata in the input is: {}",
        license_tree.get_metadata_sum().expect("invalid input")
    );

    println!(
        "The value of the root node is: {}",
        license_tree.get_root_node_value().expect("invalid input")
    );
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use super::LicenseTree;

    const TEST_INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    lazy_static! {
        static ref EXPECTED: LicenseTree = {
            LicenseTree {
                raw_nodes_input: vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2],
            }
        };
    }

    #[test]
    fn parse_test() {
        assert_eq!(*EXPECTED, TEST_INPUT.parse().unwrap());
    }

    #[test]
    fn metadata_sum_test() {
        assert_eq!(138, EXPECTED.get_metadata_sum().unwrap());
    }

    #[test]
    fn root_node_value_test() {
        assert_eq!(66, EXPECTED.get_root_node_value().unwrap());
    }
}
