#![feature(str_escape)]

extern crate regex;

use std::io;
use std::io::prelude::*;

use regex::Regex;

fn main() {

    let stdin = io::stdin();

    let mut character_count = 0;
    let mut byte_count = 0;
    let mut escaped_byte_count = 0;

    let line_quotes = Regex::new(r#"^".*""#).unwrap();
    let two_char_escape = Regex::new(r#"\\\\|\\""#).unwrap();
    let hex_escape = Regex::new(r#"\\x[:xdigit:]{2}"#).unwrap();

    for wline in stdin.lock().lines() {
        let line = wline.unwrap();

        byte_count += line.len();
        character_count += line.len();

        // +2 because the escaped string is supposed to be surrounded in quotes '"'
        escaped_byte_count += line.escape_default().len() + 2;

        if line_quotes.is_match(&line) {
            character_count -= 2;
        } else {
            panic!(r#"Expected line to start and end with quotes '"': {}"#, line);
        }

        character_count -= two_char_escape.find_iter(&line).count();

        character_count -= 3 * hex_escape.find_iter(&line).count();
    }

    println!("Total number of bytes: {}", byte_count);
    println!("Number of characters: {}", character_count);
    println!("Total number of bytes in escaped strings: {}", escaped_byte_count);
    println!("bytes - characters: {}", byte_count - character_count);
    println!("escaped bytes - bytes: {}", escaped_byte_count - byte_count);
}
