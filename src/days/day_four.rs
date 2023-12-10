use crate::prelude::*;

#[derive(Clone, Debug)]
struct Card {
    winning: Vec<u8>,
    scratched: Vec<u8>,
    points: u32,
    matches: usize,
}

impl Card {
    fn new(winning: Vec<u8>, scratched: Vec<u8>) -> Self {
        let (mut points, mut matches) = (0, 0);

        for num in scratched.clone() {
            if !winning.contains(&num) {
                continue;
            }

            matches += 1;
            if points == 0 {
                points += 1;
            } else {
                points *= 2;
            }
        }

        Card { winning, scratched, points, matches }
    }

    fn matched_cards(self, i: usize) -> Vec<usize> {
        (i + 1..i + 1 + self.matches).collect()
    }
}

pub fn day_four(is_part_two: bool) -> String {
    let input = read_input("in/4-1.txt");
    let lines = input.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
    let cards = get_cards(lines);
    let mut won_cards: Vec<usize> = (0..cards.len()).collect();
    let mut points = 0u32;

    let mut i = 0;
    while i < won_cards.len() - 1 {
        let card_index = won_cards[i];
        let card = cards[won_cards[card_index]].clone();

        if !is_part_two {
            points += card.points;
            i += 1;
            continue;
        }

        let mut won = card.clone().matched_cards(card_index);
        won_cards.append(&mut won);

        i += 1;
    }

    return if is_part_two {
        won_cards.len().to_string()
    } else {
        points.to_string()
    };
}

fn get_cards(lines: Vec<String>) -> Vec<Card> {
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

        cards.push(Card::new(winning, scratched));
    }

    cards
}
