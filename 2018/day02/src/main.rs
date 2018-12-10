use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use std::cmp;

fn get_histograms(input: &str) -> Vec<HashMap<char, usize>> {
    input
        .lines()
        .map(|line| {
            let mut histo = HashMap::new();

            line.chars().for_each(|ch| {
                *histo.entry(ch).or_insert(0) += 1;
            });

            histo
        })
        .collect()
}

fn count_ids(histos: &[HashMap<char, usize>]) -> (usize, usize) {
    let mut count = (0, 0);

    for histo in histos {
        let mut two_seen = false;
        let mut three_seen = false;

        for cnt in histo.values() {
            if !two_seen && *cnt == 2 {
                count.0 += 1;
                two_seen = true;
            }

            if !three_seen && *cnt == 3 {
                count.1 += 1;
                three_seen = true;
            }
        }
    }

    count
}

fn find_first_similar(input_str: &str) -> Option<(String, String)> {
    for (idx_a, line_a) in input_str.lines().enumerate() {
        for (_, line_b) in input_str
            .lines()
            .enumerate()
            .filter(|(idx_b, _)| idx_a != *idx_b)
        {
            if line_a.len() != line_b.len() {
                continue;
            }

            let diffs_seen = (0..line_a.len())
                .filter(|i| line_a.as_bytes()[*i] != line_b.as_bytes()[*i])
                .count();

            if diffs_seen == 1 {
                return Some((line_a.to_string(), line_b.to_string()));
            }
        }
    }

    None
}

fn reduce_similar(string_a: &str, string_b: &str) -> String {
    let mut out = String::new();

    for i in 0..cmp::min(string_a.len(), string_b.len()) {
        if string_a.as_bytes()[i] == string_b.as_bytes()[i] {
            out.push(string_a.as_bytes()[i].into());
        }
    }

    out
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let histos = get_histograms(&input_str);

    let (twice, thrice) = count_ids(&histos);

    println!(
        "The ID counts are {} and {}. Checksum {}.",
        twice,
        thrice,
        twice * thrice
    );

    let (string_a, string_b) = find_first_similar(&input_str).expect("no similar ID strings");

    let reduced = reduce_similar(&string_a, &string_b);

    println!(
        "The similar ID strings are '{}' and '{}'. The common letters are '{}'.",
        string_a, string_b, reduced
    );
}

#[cfg(test)]
mod tests {
    use super::count_ids;
    use super::find_first_similar;
    use super::get_histograms;
    use super::reduce_similar;

    const TEST_INPUT_1: &str = "abcdef\n\
                                bababc\n\
                                abbcde\n\
                                abcccd\n\
                                aabcdd\n\
                                abcdee\n\
                                ababab";

    const TEST_INPUT_2: &str = "abcde\n\
                                fghij\n\
                                klmno\n\
                                pqrst\n\
                                fguij\n\
                                axcye\n\
                                wvxyz";

    #[test]
    fn id_count_test() {
        let histos = get_histograms(TEST_INPUT_1);

        assert_eq!((4, 3), count_ids(&histos));
    }

    #[test]
    fn find_similar_test() {
        let (line_a, line_b) = find_first_similar(TEST_INPUT_2).unwrap();

        assert_eq!("fghij", line_a);
        assert_eq!("fguij", line_b);

        assert_eq!("fgij", reduce_similar(&line_a, &line_b));
    }

}
