use std::collections::HashMap;


fn reallocate(mut banks: Vec<usize>) -> (usize, usize) {
    let mut states_seen: HashMap<Vec<usize>, usize> = HashMap::new();

    for num_reallocs in 1.. {

        let (max_idx, max) = banks.iter().enumerate().fold((0, 0), |(curr_max_idx, curr_max), (new_idx, new_val)| {
            if *new_val > curr_max {
                (new_idx, *new_val)
            } else {
                (curr_max_idx, curr_max)
            }
        });

        banks[max_idx] = 0;

        for idx in (max_idx + 1)..(max_idx + 1 + max) {
            let actual_idx = idx % banks.len();

            banks[actual_idx] += 1;
        }

        if states_seen.contains_key(&banks) {
            return (num_reallocs - states_seen[&banks], num_reallocs);
        }

        states_seen.insert(banks.clone(), num_reallocs);
    }

    unreachable!();
}

fn main() {
    println!("Loop length, num reallocs: {:?}", reallocate(vec![0, 5, 10, 0, 11, 14, 13, 4, 11, 8, 8, 7, 1, 4, 12, 11]));
}

#[cfg(test)]
mod tests {
    use super::reallocate;

    #[test]
    fn test_realloc() {
        assert_eq!((4, 5), reallocate(vec![0, 2, 7, 0]));
    }
}