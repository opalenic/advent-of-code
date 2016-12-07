extern crate regex;
extern crate itertools;

use regex::Regex;

use std::collections::HashMap;

use itertools::Itertools;

use std::cmp::Ordering;

use std::io;
use std::io::prelude::*;


fn get_valid_rooms(input: &str) -> Vec<(String, u64)> {
    let re = Regex::new(r"^(?P<name>[a-z-]+)-(?P<id>[0-9]+)\[(?P<csum>[a-z]+)\]").unwrap();

    input.lines()
        .filter_map(|line| {

            let cap = re.captures(line).unwrap();

            let name = cap.name("name").unwrap();

            let mut histo = HashMap::new();
            for ch in name.chars().filter(|ch| *ch != '-') {
                let cnt = histo.entry(ch).or_insert(0);
                *cnt += 1;
            }

            let calc_csum = histo.iter()
                .sorted_by(|a, b| match Ord::cmp(b.1, a.1) {
                    Ordering::Equal => Ord::cmp(a.0, b.0),
                    ord => ord,
                })
                .into_iter()
                .take(5)
                .map(|(ch, _cnt)| *ch)
                .collect::<String>();

            let csum = cap.name("csum").unwrap();

            if calc_csum == csum {
                Some((name.to_string(), cap.name("id").unwrap().parse::<u64>().unwrap()))
            } else {
                None
            }
        })
        .collect()
}

fn rotate_str(input: &str, num_steps: u64) -> String {
    input.chars()
        .map(|ch| if ch != '-' {
            (((ch as u64 - 'a' as u64) + num_steps) % 26 + ('a' as u64)) as u8 as char
        } else {
            ' '
        })
        .collect()
}

fn main() {
    let mut room_str = String::new();
    io::stdin().read_to_string(&mut room_str).expect("Invalid input string!");

    let valid_rooms = get_valid_rooms(&room_str);

    println!("The sum of real room sector IDs is: {}",
             valid_rooms.iter().map(|&(ref _name, id)| id).sum::<u64>());

    println!("Decrypted room names:");
    for &(ref room_name, id) in &valid_rooms {
        println!("{} - {}", id, rotate_str(room_name, id));
    }
}


#[cfg(test)]
mod tests {
    use super::get_valid_rooms;
    use super::rotate_str;

    const TEST_STRING: &'static str = "aaaaa-bbb-z-y-x-123[abxyz]\n\
                                       a-b-c-d-e-f-g-h-987[abcde]\n\
                                       not-a-real-room-404[oarel]\n\
                                       totally-real-room-200[decoy]\n";

    #[test]
    fn test_valid_room_sum() {
        let valid_rooms = get_valid_rooms(TEST_STRING);

        assert_eq!(valid_rooms,
                   vec![("aaaaa-bbb-z-y-x".to_string(), 123),
                        ("a-b-c-d-e-f-g-h".to_string(), 987),
                        ("not-a-real-room".to_string(), 404)]);

        let sum = valid_rooms.iter().map(|&(ref _name, id)| id).sum::<u64>();
        assert_eq!(sum, 1514);
    }

    #[test]
    fn test_rotate_str() {
        assert_eq!(rotate_str("qzmt-zixmtkozy-ivhz", 343),
                   "very encrypted name");
    }
}
