use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    let mut input = vec![];
    let file = File::open("in/1-1.txt");
    file.unwrap().read_to_end(&mut input).unwrap();

    println!("{}", day_one_one(String::from_utf8(input).unwrap()))
}

fn day_one_one(input: String) -> String {
    let re = Regex::new(r"[0-9]").unwrap();

    let mut numbers: Vec<i32> = vec![];
    for line in input.split("\n").collect::<Vec<_>>() {
        let nums: Vec<String> = re.captures_iter(line).map(|c| c[0].to_string()).collect();
        let num = format!("{}{}", nums.first().unwrap(), nums.last().unwrap());
        numbers.push(num.parse().unwrap());
    }

    numbers.iter().sum::<i32>().to_string()
}