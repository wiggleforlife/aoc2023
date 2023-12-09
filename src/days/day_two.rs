use crate::prelude::*;

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
