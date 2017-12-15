use std::io;
use std::io::prelude::*;

use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::BTreeMap;

fn get_valid_passphrases(input_str: &str) -> Vec<String> {
    input_str
        .lines()
        .filter(|line| {
            let mut words_seen = HashSet::new();
            line.split_whitespace().all(|word| words_seen.insert(word))
        })
        .map(|line| line.to_string())
        .collect()
}

fn get_valid_passphrases_b(input_str: &str) -> Vec<String> {
    input_str
        .lines()
        .filter(|line| {
            let mut words_seen = BTreeSet::new();

            line.split_whitespace().all(|word| {
                let mut word_histo = BTreeMap::new();

                for ch in word.chars() {
                    *word_histo.entry(ch).or_insert(0) += 1;
                }

                words_seen.insert(word_histo)
            })
        })
        .map(|line| line.to_string())
        .collect()
}

fn main() {

    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let passphrases = get_valid_passphrases(&input_str);

    println!("Number of valid passphrases: {}", passphrases.len());

    let passphrases_b = get_valid_passphrases_b(&input_str);
    println!(
        "Number of valid anagram passphrases: {}",
        passphrases_b.len()
    );
}

#[cfg(test)]
mod tests {
    use super::get_valid_passphrases;
    use super::get_valid_passphrases_b;

    #[test]
    fn password_validity_test() {
        let test_input = "aa bb cc dd ee\n\
                          aa bb cc dd aa\n\
                          aa bb cc dd aaa\n";

        let valid_output = vec!["aa bb cc dd ee", "aa bb cc dd aaa"];

        assert_eq!(valid_output, get_valid_passphrases(test_input));
    }

    #[test]
    fn password_anagram_validity_test() {
        let test_input = "abcde fghij\n\
                          abcde xyz ecdab\n\
                          a ab abc abd abf abj\n\
                          iiii oiii ooii oooi oooo\n\
                          oiii ioii iioi iiio";

        let valid_output = vec![
            "abcde fghij",
            "a ab abc abd abf abj",
            "iiii oiii ooii oooi oooo",
        ];

        assert_eq!(valid_output, get_valid_passphrases_b(test_input));
    }
}
