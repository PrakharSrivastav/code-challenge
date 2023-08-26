#![allow(unused)]
use std::ops::Sub;

use chrono::{DateTime, NaiveDate};

fn weeks_between(a: &str, b: &str) -> i32 {
    let format = "%Y-%m-%d";
    let date_from = NaiveDate::parse_from_str(a, format).unwrap();
    let date_to = NaiveDate::parse_from_str(b, format).unwrap();

    date_to.sub(date_from).num_weeks() as i32
}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21");

    println!("hello: {}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}
