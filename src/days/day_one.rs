use crate::prelude::*;

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
