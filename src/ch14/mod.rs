#![allow(unused)]

mod run_length_encoding {
    use std::collections::HashMap;

    pub fn encode(text: &str) -> String {
        let mut count = 0;
        let mut encoded = String::new();
        let mut prev: Option<char> = None;

        for c in text.chars() {
            if prev.is_none() {
                prev = Some(c);
            }

            if prev.unwrap() != c || count == 9 {
                encoded.push_str(&format!("{}{}", count, prev.unwrap()));
                count = 0
            }
            prev = Some(c);
            count += 1
        }

        if prev.is_some() {
            encoded.push_str(&format!("{}{}", count, prev.unwrap()));
        }
        encoded
    }

    pub fn decode(text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars();

        while let (Some(n), Some(c)) = (chars.next(), chars.next()) {
            let n = n.to_digit(10).unwrap();
            for _ in 0..n {
                result.push(c)
            }
        }
        result
    }
}

fn main() {
    //
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn abca() {
    use run_length_encoding::*;

    assert_eq!(encode("abca"), "1a1b1c1a");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
