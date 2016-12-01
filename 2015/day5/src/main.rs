
extern crate pcre;

use std::io;
use std::io::prelude::*;

use pcre::Pcre;

fn main() {

    let mut re1 = Pcre::compile(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let mut re2 = Pcre::compile(r"([a-z])\1").unwrap();
    let mut re3 = Pcre::compile(r"(ab)|(cd)|(pq)|(xy)").unwrap();


    let mut re4 = Pcre::compile(r"([a-z][a-z]).*\1").unwrap();
    let mut re5 = Pcre::compile(r"([a-z]).\1").unwrap();


    let mut count_a = 0;
    let mut count_b = 0;


    let stdin = io::stdin();

    for wline in stdin.lock().lines() {
        let line = wline.unwrap();

        if re1.exec(&line).is_some() && re2.exec(&line).is_some() && re3.exec(&line).is_none() {
            count_a += 1;
        }

        if re4.exec(&line).is_some() && re5.exec(&line).is_some() {
            count_b += 1;
        }
    }

    println!("Answer A: {}", count_a);
    println!("Answer B: {}", count_b);
}
