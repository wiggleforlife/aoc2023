use crate::prelude::*;

struct Card {
    winning: Vec<u8>,
    scratched: Vec<u8>,
    points: u32,
}

pub fn day_four(is_part_two: bool) -> String {
    let input = read_input("in/4-1.txt");
    let lines = input.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
    let cards = split_cards(lines);
    let mut sum = 0u32;

    for mut card in cards {
        for num in card.scratched {
            if !card.winning.contains(&num) {
                continue;
            }
            if card.points == 0 {
                card.points += 1;
            } else {
                card.points *= 2;
            }
        }

        sum += card.points;
    }

    sum.to_string()
}

fn split_cards(lines: Vec<String>) -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];
    for mut line in lines {
        line = line.split(": ").last().unwrap().to_string();
        let sections: Vec<&str> = line.split(" | ").collect();

        let winning: Vec<u8> = sections[0]
            .split(" ")
            .filter(|&n| !n.is_empty())
            .map(|n| n.parse::<u8>().unwrap())
            .collect();

        let scratched: Vec<u8> = sections[1]
            .split(" ")
            .filter(|&n| !n.is_empty())
            .map(|n| {
                let n = n.replace("\r", "");
                n.parse::<u8>().unwrap()
            })
            .collect();

        cards.push(Card { winning, scratched, points: 0 });
    }

    cards
}
