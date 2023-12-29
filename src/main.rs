#![allow(dead_code)]

use crate::days::*;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub mod prelude {
    pub use regex::{Match, Regex};
    pub use std::ops::Range;

    pub use super::*;
}

mod days {
    pub mod day_five;
    pub mod day_four;
    pub mod day_one;
    pub mod day_three;
    pub mod day_two;
}

fn main() {
    println!("{}", day_five::day_five(true))
}

pub fn read_input(path: &str) -> String {
    let mut input = vec![];
    let file = File::open(path);
    file.unwrap().read_to_end(&mut input).unwrap();

    String::from_utf8(input).unwrap()
}

/// If running into issue, verify line endings are \n (Unix/LF)
pub fn split_to_string(input: String, pattern: &str) -> Vec<String> {
    input
        .split(pattern)
        .filter(|&s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

pub fn split_vec_to_string(
    input: Vec<String>, pattern: &str,
) -> Vec<Vec<String>> {
    let mut result = vec![];
    for item in input {
        let mut sub_vec = vec![];
        for sub_item in split_to_string(item, pattern) {
            sub_vec.push(sub_item);
        }

        result.push(sub_vec);
    }

    result
}

pub fn split_parse<T: FromStr>(input: String, pattern: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    input.split(pattern).map(|s| s.parse::<T>().unwrap()).collect::<Vec<T>>()
}
