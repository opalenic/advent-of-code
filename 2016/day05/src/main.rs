extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::env;

fn generate_password(base: &str) -> String {
    let mut md5 = Md5::new();

    (0..)
        .filter_map(|i| {
            md5.reset();
            md5.input_str(format!("{}{}", base, i).as_str());
            let out = md5.result_str();

            if &out[0..5] == "00000" {
                Some(out.as_bytes()[5] as char)
            } else {
                None
            }
        })
        .take(8)
        .collect()
}

fn generate_password_b(base: &str) -> String {
    let mut md5 = Md5::new();

    let mut out_chars = vec![None; 8];

    let mut i = 0;

    while !out_chars.iter().all(|ch| ch.is_some()) {
        md5.reset();
        md5.input_str(format!("{}{}", base, i).as_str());
        let out_str = md5.result_str();

        if &out_str[0..5] == "00000" {
            let pos = (out_str.as_bytes()[5] - ('0' as u8)) as usize;
            if pos < 8 {
                let ch = out_str.as_bytes()[6] as char;

                if out_chars[pos].is_none() {
                    out_chars[pos] = Some(ch);
                }
            }
        }

        i += 1;
    }

    out_chars.into_iter().collect::<Option<String>>().unwrap()
}

fn main() {
    let mut a = env::args();

    a.next(); // The first argument is the binary name/path

    let input_str = a.next().unwrap();

    println!("The first password is: {}", generate_password(&input_str));

    println!("The second password is: {}",
             generate_password_b(&input_str));

}

#[cfg(test)]
mod tests {
    use super::generate_password;
    use super::generate_password_b;

    #[test]
    fn test_generator() {
        assert_eq!(generate_password("abc"), "18f47a30");
    }

    #[test]
    fn test_generator_b() {
        assert_eq!(generate_password_b("abc"), "05ace8e3");
    }
}
