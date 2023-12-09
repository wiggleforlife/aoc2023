use crate::prelude::*;

pub fn day_three(is_part_two: bool) -> String {
    let input = read_input("in/3-1.txt");
    let lines = input.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
    let mut sum = 0u32;

    let re_digit = Regex::new(r"\d+").unwrap();
    let re_symbol = Regex::new(r"[^a-zA-Z0-9_.]").unwrap();
    let re_gear = Regex::new(r"\*").unwrap();

    let get_adjacent_chars_match = |i: usize, num: Match, re: Regex| {
        let mut bounds = num.start()..num.end();
        bounds =
            extend_bounds(bounds.clone(), re.clone(), lines[i].clone(), true);

        vec![
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
        ]
    };

    for i in 0..lines.len() {
        if !is_part_two {
            for num in re_digit.find_iter(&*lines[i]) {
                let rows = get_adjacent_chars_match(i, num, re_digit.clone());

                if rows
                    .iter()
                    .map(|r| re_symbol.is_match(r))
                    .find(|b| *b)
                    .is_some()
                {
                    sum += num.as_str().parse::<u32>().unwrap();
                }
                continue;
            }
            continue;
        }

        for gear in re_gear.find_iter(&*lines[i]) {
            let rows = get_adjacent_chars_match(i, gear, re_gear.clone());
            let mut nums: Vec<u32> = vec![];
            let mut num_bounds: Vec<(usize, Range<usize>)> = vec![];

            for row in 0..rows.len() {
                let row_bytes = rows[row].as_bytes();
                for byte in 0..row_bytes.len() {
                    if re_digit.is_match(
                        &*String::from_utf8(vec![row_bytes[byte]]).unwrap(),
                    ) {
                        let line = i + row - 1;
                        let bounds = extend_bounds(
                            if row_bytes.len() == 3 && byte == 0 {
                                (gear.start() - 1)..gear.start()
                            } else if row_bytes.len() == 3 && byte == 1 {
                                gear.start()..gear.end()
                            } else if row_bytes.len() == 3 && byte == 2 {
                                gear.end()..(gear.end() + 1)
                            } else {
                                gear.start()..gear.end()
                            },
                            re_digit.clone(),
                            lines[line].clone(),
                            false,
                        );

                        if num_bounds.contains(&(line, bounds.clone())) {
                            continue;
                        }
                        let num = lines[line][bounds.clone()]
                            .to_string()
                            .parse::<u32>();
                        num_bounds.push((line, bounds));

                        if num.is_ok() {
                            nums.push(num.unwrap());
                        }
                    }
                }
            }

            if nums.len() == 2 {
                sum += nums.iter().product::<u32>();
            }
        }
    }

    sum.to_string()
}

fn extend_bounds(
    bounds: Range<usize>, re: Regex, line: String, add_padding: bool,
) -> Range<usize> {
    let mut new_bounds = bounds;
    loop {
        if new_bounds.start != 0
            && re.is_match(
                &*String::from_utf8(vec![
                    line.as_bytes()[new_bounds.start - 1],
                ])
                .unwrap(),
            )
        {
            new_bounds.start -= 1;
            continue;
        }
        if new_bounds.end < line.len() - 1
            && re.is_match(
                &*String::from_utf8(vec![line.as_bytes()[new_bounds.end]])
                    .unwrap(),
            )
        {
            new_bounds.end += 1;
            continue;
        }
        break;
    }

    if add_padding {
        if new_bounds.start != 0 {
            new_bounds.start -= 1;
        }
        if new_bounds.end != line.len() - 1 {
            new_bounds.end += 1;
        }
    }

    new_bounds
}
