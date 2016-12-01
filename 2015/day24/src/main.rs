
use std::io;
use std::io::prelude::*;

use std::u64;

#[derive(Debug)]
struct Arrangement {
    present_locations: Vec<u32>,
    present_sizes: Vec<u32>,
    single_group_sum: u32,
    num_groups: u8,
}



impl Arrangement {
    fn new(mut presents: Vec<u32>, num_groups: u8) -> Arrangement {
        presents.sort();
        presents.reverse();

        let locations = vec![0; presents.len()];
        let sum = presents.iter().fold(0, |acc, &x| acc + x) / (num_groups as u32);

        Arrangement {
            present_locations: locations,
            present_sizes: presents,
            single_group_sum: sum,
            num_groups: num_groups,
        }
    }

    fn get_next_valid(&mut self) -> Option<(usize, u64, Vec<u32>)> {

        while self.increment() {
            let present_weight_in_first = self.present_locations
                .iter()
                .enumerate()
                .fold(0, |acc, (pres_idx, contained_in_first)| {
                    if *contained_in_first == 1 {
                        acc + self.present_sizes[pres_idx]
                    } else {
                        acc
                    }
                });


            if present_weight_in_first == self.single_group_sum {
                let mut tail = Vec::with_capacity(self.present_sizes.len());
                let mut head_len = 0;
                let mut qe = 1u64;

                for pres_idx in 0..self.present_locations.len() {
                    if self.present_locations[pres_idx] == 1 {
                        head_len += 1;
                        qe = if let Some(val) =
                                    qe.checked_mul(self.present_sizes[pres_idx] as u64) {
                            val
                        } else {
                            u64::MAX
                        };

                    } else {
                        tail.push(self.present_sizes[pres_idx]);
                    }
                }

                return Some((head_len, qe, tail));
            }
        }

        None
    }

    fn get_best_arrangement(&mut self) -> u64 {

        let mut min_valid_group_len = self.present_locations.len();
        let mut min_valid_qe_seen = self.present_sizes
            .iter()
            .fold(1u64, |acc, &weight| {
                if let Some(val) = acc.checked_mul(weight as u64) {
                    val
                } else {
                    u64::MAX
                }
            });


        while let Some((head_len, this_qe, tail)) = self.get_next_valid() {

            if head_len > min_valid_group_len {
                continue;
            }


            let mut remaining_arr = Arrangement::new(tail, self.num_groups - 1);
            if remaining_arr.get_next_valid().is_some() {

                if this_qe < min_valid_qe_seen {
                    min_valid_qe_seen = this_qe;
                }

                min_valid_group_len = head_len;
            }
        }

        min_valid_qe_seen
    }

    fn increment(&mut self) -> bool {
        self.present_locations[0] += 1;

        let mut carry = 0;

        for pos in 0..self.present_locations.len() {
            if carry > 0 {
                self.present_locations[pos] += carry;
                carry = 0
            }

            if self.present_locations[pos] > 1 {
                carry = 1;
                self.present_locations[pos] = 0;
            }
        }

        carry == 0
    }
}


fn main() {

    let stdin = io::stdin();
    let presents = stdin.lock()
        .lines()
        .map(|weight_str| {
            weight_str.unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .collect::<Vec<u32>>();



    let mut arr = Arrangement::new(presents.clone(), 3);
    println!("Best QE if divided into three groups: {}",
             arr.get_best_arrangement());


    let mut arr2 = Arrangement::new(presents, 4);
    println!("Best QE if divided into four groups: {}",
             arr2.get_best_arrangement());
}


#[cfg(test)]
mod tests {

    use super::Arrangement;

    #[test]
    fn optimum_distribution_3_test() {

        let present_sizes = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];

        // 11 9       (QE= 99); 10 8 2;  7 5 4 3 1
        // 10 9 1     (QE= 90); 11 7 2;  8 5 4 3
        // 10 8 2     (QE=160); 11 9;    7 5 4 3 1
        // 10 7 3     (QE=210); 11 9;    8 5 4 2 1
        // 10 5 4 1   (QE=200); 11 9;    8 7 3 2
        // 10 5 3 2   (QE=300); 11 9;    8 7 4 1
        // 10 4 3 2 1 (QE=240); 11 9;    8 7 5
        // 9 8 3      (QE=216); 11 7 2;  10 5 4 1
        // 9 7 4      (QE=252); 11 8 1;  10 5 3 2
        // 9 5 4 2    (QE=360); 11 8 1;  10 7 3
        // 8 7 5      (QE=280); 11 9;    10 4 3 2 1
        // 8 5 4 3    (QE=480); 11 9;    10 7 2 1
        // 7 5 4 3 1  (QE=420); 11 9;    10 8 2
        // 8 7 5      (QE=280); 11 9;    10 4 3 2 1
        // 8 5 4 3    (QE=480); 11 9;    10 7 2 1
        // 7 5 4 3 1  (QE=420); 11 9;    10 8 2

        let mut arr = Arrangement::new(present_sizes.clone(), 3);

        assert_eq!(arr.get_best_arrangement(), 99);
    }

    #[test]
    fn optimum_distribution_4_test() {

        let present_sizes = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];

        // 11 4    (QE=44); 10 5;   9 3 2 1; 8 7
        // 10 5    (QE=50); 11 4;   9 3 2 1; 8 7
        // 9 5 1   (QE=45); 11 4;   10 3 2;  8 7
        // 9 4 2   (QE=72); 11 3 1; 10 5;    8 7
        // 9 3 2 1 (QE=54); 11 4;   10 5;    8 7
        // 8 7     (QE=56); 11 4;   10 5;    9 3 2 1

        let mut arr = Arrangement::new(present_sizes, 4);

        assert_eq!(arr.get_best_arrangement(), 44);
    }
}
