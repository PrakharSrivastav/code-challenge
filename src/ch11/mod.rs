#![allow(unused)]

use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::ch11::InvalidISBN::{InvalidChar, TooLong, TooShort};

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum InvalidISBN {
    TooLong,
    TooShort,
    FailedCheckSum,
    InvalidChar,
}

impl FromStr for Isbn {
    type Err = InvalidISBN; // TODO: replace with appropriate type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(13);
        let mut chars = String::from("");

        for (i, c) in s.char_indices() {
            match c {
                '-' => continue,
                '0'..='9' => {
                    digits.push(match c.to_string().parse::<u8>() {
                        Ok(i) => i,
                        Err(_) => return Err(InvalidChar),
                    });
                    chars.push(c);
                }
                _ => return Err(InvalidChar),
            }
        }

        // if digits.len() < 13 {
        //     return Err(TooShort);
        // } else if digits.len() > 13 { return Err(TooLong); }

        match digits.len().cmp(&13) {
            Ordering::Greater => return Err(TooLong),
            Ordering::Less => return Err(TooShort),
            _ => (),
        }

        if digits[12] != calculate_check_digit(&digits) {
            return Err(InvalidISBN::FailedCheckSum);
        }

        Ok(Isbn {
            raw: chars.to_string(),
            digits,
        })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let result: u8 = digits
        .iter()
        .enumerate()
        .map(|(i, d)| weighted_value(i, d))
        .sum();

    match result % 10 {
        0 => 0,
        z => 10 - z,
    }
}

fn weighted_value(i: usize, d: &u8) -> u8 {
    if i % 2 == 0 {
        (*d)
    } else {
        (*d * 3)
    }
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
