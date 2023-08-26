#![allow(unused)]

use std::fmt::Display;
use std::str::FromStr;

use crate::ch13::RGBError::InvalidColor;

#[derive(Debug, PartialEq)]
struct Rgb {
    color: String,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        0
    }

    fn g(&self) -> u8 {
        0
    }

    fn b(&self) -> u8 {
        0
    }
}

#[derive(Debug)]
enum RGBError {
    InvalidColor,
}

impl FromStr for Rgb {
    type Err = RGBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = String::new();

        match s.get(0..=0) {
            None => return Err(InvalidColor),
            Some(x) => {
                if x != "#" {
                    return Err(InvalidColor);
                } else {
                    c.push(x.chars().next().unwrap())
                }
            }
        }

        for ss in s[1..].chars() {
            match ss {
                '0'..='9' => c.push(ss),
                'A'..='F' => c.push(ss),
                'a'..='f' => c.push(ss),
                _ => return Err(InvalidColor),
            }
        }
        Ok(Self {
            color: c.to_string(),
        })
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    //
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color.color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
