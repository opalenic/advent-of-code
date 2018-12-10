use std::io;
use std::io::prelude::*;

fn find_shortest_polymer(input: &str) -> String {
    let mut min = std::usize::MAX;
    let mut min_str = input.to_string();

    for remove_ch_byte in b'a'..b'z' {
        let test_polymer = input
            .chars()
            .filter(|ch| {
                let remove_ch = char::from(remove_ch_byte);

                !ch.eq_ignore_ascii_case(&remove_ch)
            })
            .collect::<String>();

        let reduced = reduce_polymer(&test_polymer);

        if reduced.len() < min {
            min = reduced.len();
            min_str = reduced;
        }
    }

    min_str
}

fn reduce_polymer(input: &str) -> String {
    fn does_react(ch_a: char, ch_b: char) -> bool {
        ch_a.to_lowercase().eq(ch_b.to_lowercase())
            && ((ch_a.is_uppercase() && ch_b.is_lowercase())
                || (ch_a.is_lowercase() && ch_b.is_uppercase()))
    }

    let mut curr = input.trim().to_string();
    let mut out = String::new();

    loop {
        let mut reaction_occurred = false;

        let mut char_iter = curr.chars().peekable();

        while let Some(ch_a) = char_iter.next() {
            if let Some(ch_b) = char_iter.peek() {
                if does_react(ch_a, *ch_b) {
                    char_iter.next();
                    reaction_occurred = true;
                } else {
                    out.push(ch_a);
                }
            } else {
                out.push(ch_a);
            }
        }

        if !reaction_occurred {
            break;
        }

        curr = out;
        out = String::new();
    }

    out
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let reduced = reduce_polymer(&input_str);

    println!("The length of the reduced polymer is {}.", reduced.len());

    let shortest = find_shortest_polymer(&input_str);

    println!(
        "The length of the shortest possible polymer is {}.",
        shortest.len()
    );
}

#[cfg(test)]
mod tests {
    use super::find_shortest_polymer;
    use super::reduce_polymer;

    const TEST_INPUT: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn reduce_test() {
        let reduced = reduce_polymer(TEST_INPUT);
        assert_eq!("dabCBAcaDA", &reduced);
    }

    #[test]
    fn shortest_polymer_test() {
        let shortest = find_shortest_polymer(TEST_INPUT);
        assert_eq!("daDA", &shortest);
    }
}
