
use std::env::args;

use std::collections::HashMap;

trait Validator {

    fn increment(&mut self) -> bool;

    fn has_sequence(&self) -> bool;

    fn no_forbidden_chars(&self) -> bool;

    fn has_two_doubles(&self) -> bool;
}

impl Validator for Vec<u8> {
    fn increment(&mut self) -> bool {

        *(self.last_mut().unwrap()) += 1;

        let mut carry: u8 = 0;

        for pos in (0..self.len()).rev() {
            if carry > 0 {
                self[pos] += 1;
                carry = 0;
            }

            if self[pos] >= 26 {
                carry = self[pos] / 26;
                self[pos] = 0;
            }
        }

        carry != 0
    }

    fn has_sequence(&self) -> bool {
        for win in self.windows(3) {

            if win[0] + 2 == win[1] + 1 && win[1] + 1 == win[2] {
                return true;
            }
        }

        false
    }

    fn no_forbidden_chars(&self) -> bool {
        let i = ('i' as u8) - ('a' as u8);
        let o = ('o' as u8) - ('a' as u8);
        let l = ('l' as u8) - ('a' as u8);
        !(self.contains(&i) || self.contains(&o) || self.contains(&l))
    }

    fn has_two_doubles(&self) -> bool {

        let mut double_count = 0;

        let mut pos = 0;
        while pos < (self.len() - 1) {

            if self[pos] == self[pos + 1] {
                double_count += 1;
                pos += 1;

                if double_count >= 2 {
                    return true;
                }
            }

            pos += 1;
        }

        false
    }
}


fn main() {

    let mut a = args();

    a.next(); // The first argument is the binary name/path

    let start = a.next().unwrap(); // The puzzle input

    let mut char_to_num = HashMap::new();
    let mut num_to_char = HashMap::new();


    for i in 0..26 {
        let ch = (('a' as u8) + i) as char;

        char_to_num.insert(ch, i);
        num_to_char.insert(i, ch);
    }

    let mut passwd_vec = start.chars().map(|ch| char_to_num[&ch]).collect::<Vec<u8>>();

    loop {
        if passwd_vec.increment() {
            panic!("All password combinations exhausted and no password found.");
        }

        if !passwd_vec.has_sequence() {
            continue;
        }

        if !passwd_vec.no_forbidden_chars() {
            continue;
        }

        if !passwd_vec.has_two_doubles() {
            continue;
        }

        break;
    }

    let readable_passwd = passwd_vec.iter().map(|ch_num| num_to_char[ch_num]).collect::<String>();

    println!("The next password is: {:?}", passwd_vec);
    println!("Readable password: {:?}", readable_passwd);

}
