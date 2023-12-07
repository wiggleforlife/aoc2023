#![allow(dead_code)]

use regex::Regex;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("{}", day_three(false))
}

fn read_input(path: &str) -> String {
    let mut input = vec![];
    let file = File::open(path);
    file.unwrap().read_to_end(&mut input).unwrap();

    String::from_utf8(input).unwrap()
}

fn day_three(is_part_two: bool) -> String {
    let input = read_input("in/3-1.txt");
    let lines = input.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
    let re_digit = Regex::new(r"\d+").unwrap();
    let re_symbol = Regex::new(r"[^a-zA-Z0-9_.]").unwrap();
    let mut sum = 0u32;

    for i in 0..lines.len() {
        for num in re_digit.find_iter(&*lines[i]) {
            let bounds =
                if num.start() > 0 { num.start() - 1 } else { num.start() }
                    ..if num.end() < lines[i].len() - 1 {
                        num.end() + 1
                    } else {
                        num.end()
                    };
            let rows: Vec<String> = vec![
                if i > 0 {
                    lines[i - 1][bounds.clone()].to_string()
                } else {
                    String::new()
                },
                lines[i][bounds.clone()].to_string(),
                if i < lines.len() - 1 {
                    lines[i + 1][bounds.clone()].to_string()
                } else {
                    String::new()
                },
            ];
            println!("{}", lines[i]);
            for row in &rows {
                println!("{row}");
            }
            if rows
                .iter()
                .map(|r| re_symbol.is_match(r))
                .find(|b| *b == true)
                .is_some()
            {
                sum += num.as_str().parse::<u32>().unwrap();
            }
        }
    }

    sum.to_string()
}

fn day_two(is_part_two: bool) -> String {
    let input = read_input("in/2-1.txt");
    let lines = input.split("\n").collect::<Vec<&str>>();
    let games = lines
        .iter()
        .map(|l| {
            let l = l.split(": ").collect::<Vec<&str>>();
            (l[0].replace("Game ", "").parse().unwrap(), l[1])
        })
        .collect::<Vec<(i32, &str)>>();
    let re = Regex::new(r"[0-9]+ [a-z]+").unwrap();

    let mut powers_or_ids: Vec<i32> = vec![];
    for i in 0..games.len() {
        let draws: Vec<Vec<String>> = re
            .captures_iter(games[i].1)
            .map(|c| {
                let c = c[0].to_string();
                let arr = c.split(" ").map(|s| s.to_string()).collect();
                arr
            })
            .collect();

        if is_part_two {
            let mut min = (0, 0, 0);
            for draw in draws {
                let count = draw[0].parse::<i32>().unwrap();
                match &*draw[1] {
                    "red" => {
                        if count > min.0 {
                            min.0 = count;
                        }
                    },
                    "green" => {
                        if count > min.1 {
                            min.1 = count;
                        }
                    },
                    "blue" => {
                        if count > min.2 {
                            min.2 = count;
                        }
                    },
                    _ => (),
                }
            }

            powers_or_ids.push(min.0 * min.1 * min.2);
        } else {
            let mut possible = true;
            for draw in draws {
                match &*draw[1] {
                    "red" => {
                        if draw[0].parse::<i32>().unwrap() > 12 {
                            possible = false;
                            break;
                        }
                    },
                    "green" => {
                        if draw[0].parse::<i32>().unwrap() > 13 {
                            possible = false;
                            break;
                        }
                    },
                    "blue" => {
                        if draw[0].parse::<i32>().unwrap() > 14 {
                            possible = false;
                            break;
                        }
                    },
                    _ => (),
                }
            }

            if possible {
                powers_or_ids.push((i + 1) as i32)
            }
        }
    }

    powers_or_ids.iter().sum::<i32>().to_string()
}

fn day_one(is_part_two: bool) -> String {
    let input = read_input("in/1-1.txt");
    let re = Regex::new(r"[1-9]").unwrap();
    let number_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut numbers: Vec<i32> = vec![];
    for mut line in input.split("\n").map(|s| s.to_string()).collect::<Vec<_>>()
    {
        if is_part_two {
            let mut new = String::new();
            for i in 0..line.len() {
                new.push(line.as_bytes()[i] as char);
                for ii in 0..number_words.len() {
                    let end = i + number_words[ii].len();
                    if line.len() <= end || &line[i..end] != number_words[ii] {
                        continue;
                    }
                    new.push((ii + 1).to_string().parse().unwrap());
                    break;
                }
            }
            line = new;
        }

        let nums: Vec<String> =
            re.captures_iter(&line).map(|c| c[0].to_string()).collect();
        let num = format!("{}{}", nums.first().unwrap(), nums.last().unwrap());
        numbers.push(num.parse().unwrap());
    }

    numbers.iter().sum::<i32>().to_string()
}
