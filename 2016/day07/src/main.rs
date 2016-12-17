
extern crate pcre;

use pcre::Pcre;

use std::io;
use std::io::prelude::*;

fn validate_ip_tls(input: &str) -> Vec<String> {
    let mut hypernet_re = Pcre::compile(r"\[([^\]]*)\]").unwrap();
    let mut abba_re = Pcre::compile(r"([a-z])([a-z])\2\1").unwrap();

    input.lines()
        .filter(|line| {

            for m in hypernet_re.matches(line) {
                if abba_re.matches(m.group(0)).any(|a| a.group(1) != a.group(2)) {
                    return false;
                }
            }

            abba_re.matches(line).any(|m| m.group(1) != m.group(2))
        })
        .map(|line| line.to_string())
        .collect()
}

fn validate_ip_ssl(input: &str) -> Vec<String> {
    // Get hypernet sections
    let mut hypernet_re = Pcre::compile(r"\[([^\]]*)\]").unwrap();

    // Get addr sections
    let mut addr_re = Pcre::compile(r"([a-z]+)(?:\[[^\]]*\])*").unwrap();

    input.lines()
        .filter(|line| {
            let mut possible_babs = Vec::new();

            // Find all ABAs and create corresponding BAB regexes
            for addr_match in addr_re.matches(line) {

                let addr_chars: &[u8] = addr_match.group(1).as_ref();
                for window in addr_chars.windows(3) {
                    if window[0] == window[2] && window[0] != window[1] {
                        let bab_re = Pcre::compile(format!("{}{}{}",
                                                           window[1] as char,
                                                           window[0] as char,
                                                           window[1] as char)
                                .as_str())
                            .unwrap();

                        possible_babs.push(bab_re);
                    }
                }
            }


            // Look for any possible BAB in the hypernet strings
            for hypernet_match in hypernet_re.matches(line) {
                for bab_re in possible_babs.iter_mut() {
                    if bab_re.exec(hypernet_match.group(1)).is_some() {
                        return true;
                    }
                }
            }

            false
        })
        .map(|line| line.to_string())
        .collect()
}


fn main() {
    let mut address_str = String::new();
    io::stdin().read_to_string(&mut address_str).expect("Invalid input string!");

    println!("The number of valid addresses is: {}",
             validate_ip_tls(&address_str).len());
    println!("The number of valid SSL addresses is: {}",
             validate_ip_ssl(&address_str).len());
}


#[cfg(test)]
mod tests {
    use super::validate_ip_tls;
    use super::validate_ip_ssl;

    #[test]
    fn test_tls_validation() {
        let test_str = "abba[mnop]qrst\n\
                        abcd[bddb]xyyx\n\
                        aaaa[qwer]tyui\n\
                        ioxxoj[asdfgh]zxcvbn";

        assert_eq!(validate_ip_tls(test_str),
                   vec!["abba[mnop]qrst".to_string(), "ioxxoj[asdfgh]zxcvbn".to_string()]);
    }

    #[test]
    fn test_ssl_validation() {
        let test_str = "aba[bab]xyz\n\
                        xyx[xyx]xyx\n\
                        aaa[kek]eke\n\
                        zazbz[bzb]cdb";

        assert_eq!(validate_ip_ssl(test_str),
                   vec!["aba[bab]xyz".to_string(),
                        "aaa[kek]eke".to_string(),
                        "zazbz[bzb]cdb".to_string()]);
    }
}
