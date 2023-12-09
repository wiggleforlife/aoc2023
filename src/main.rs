#![allow(dead_code)]

use crate::days::*;
use std::fs::File;
use std::io::Read;

pub mod prelude {
    pub use regex::{Match, Regex};
    pub use std::ops::Range;

    pub use super::read_input;
}

mod days {
    pub mod day_four;
    pub mod day_one;
    pub mod day_three;
    pub mod day_two;
}

fn main() {
    println!("{}", day_four::day_four(false))
}

pub fn read_input(path: &str) -> String {
    let mut input = vec![];
    let file = File::open(path);
    file.unwrap().read_to_end(&mut input).unwrap();

    String::from_utf8(input).unwrap()
}
