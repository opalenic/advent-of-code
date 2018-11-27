extern crate crypto;
extern crate pcre;

use crypto::digest::Digest;
use crypto::md5::Md5;

use pcre::Pcre;

use std::collections::VecDeque;

use std::env;

fn hash_stretch(base: &str, num_iterations: usize) -> String {
    let mut md5 = Md5::new();
    let mut out = base.to_string();

    for _ in 0..num_iterations {
        md5.reset();
        md5.input_str(&out);
        out = md5.result_str();
    }

    out
}

fn generate_keys(salt: &str, num_keys: usize, secure_hashing: bool) -> Vec<(usize, String)> {
    let mut hash_check_re = Pcre::compile(r"([0-9a-z])\1\1").expect("bad regex");

    let mut candidate_list: VecDeque<(usize, String, Pcre)> = VecDeque::new();
    let mut key_list = Vec::new();

    let mut md5 = Md5::new();

    let mut curr_idx = 0;

    while key_list.len() < num_keys {
        // Create hash
        md5.reset();
        md5.input_str(&format!("{}{}", salt, curr_idx));

        let curr_hash = if secure_hashing {
            hash_stretch(&md5.result_str(), 2016)
        } else {
            md5.result_str()
        };

        // Purge candidates older than 1000 elements
        loop {
            let elem_idx = if let Some((elem_idx, _, _)) = candidate_list.front() {
                *elem_idx
            } else {
                break;
            };

            if elem_idx + 1000 <= curr_idx {
                candidate_list.pop_front();
            } else {
                break;
            }
        }

        // Loop through candidate list and see if one of them is a key
        // Remove a candidate if it is a key
        let mut new_list = VecDeque::new();

        for (candidate_idx, candidate_hash_str, mut candidate_hash_regex) in candidate_list {
            if candidate_hash_regex.exec(&curr_hash).is_some() {
                key_list.push((candidate_idx, candidate_hash_str));
            } else {
                new_list.push_back((candidate_idx, candidate_hash_str, candidate_hash_regex));
            }
        }

        candidate_list = new_list;

        // See if the current hash is a candidate
        if let Some(caps) = hash_check_re.exec(&curr_hash) {
            let matched_char = caps.group(1).chars().next().expect("missing first char");

            let check_re = Pcre::compile(&format!(r"({})\1\1\1\1", matched_char))
                .expect("could not create regex");

            candidate_list.push_back((curr_idx, curr_hash.clone(), check_re));
        }

        curr_idx += 1;
    }

    key_list.sort_by_key(|(idx, _)| *idx);
    key_list.truncate(num_keys);
    key_list
}

fn main() {
    let mut a = env::args();

    a.next(); // The first argument is the binary name/path

    let salt = a.next().expect("missing first CLI parameter (salt string)");

    let keys = generate_keys(&salt, 64, false);

    println!(
        "The index that generates the 64th key is: {}",
        keys.last().unwrap().0
    );

    let secure_keys = generate_keys(&salt, 64, true);
    println!(
        "Hash stretching enabled. The index that generates the 64th key is: {}",
        secure_keys.last().unwrap().0
    );
}

#[cfg(test)]
mod tests {

    use super::generate_keys;

    #[test]
    fn generate_keys_test() {
        let keys = generate_keys("abc", 64, false);

        assert_eq!(39, keys.first().unwrap().0);
        assert_eq!(22728, keys.last().unwrap().0);

        let secure_keys = generate_keys("abc", 64, true);

        assert_eq!(22551, secure_keys.last().unwrap().0);
    }
}
