pub fn read_input() -> &'static str {
    include_str!("input")
}

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PARSE_CARD: Regex =
        Regex::new(r"^Card\s*(?<id>\d+):\s*(?<winning>\d[\d\s]*?)\s*\|\s*(?<hand>\d.*)$").unwrap();
}

pub struct Card {
    id: u128,
    winning: Vec<u128>,
    hand: Vec<u128>,
    winning_cards: u32,
}

use std::collections::HashMap;

impl Card {
    pub fn from_input(line: &str) -> Self {
        let captures = PARSE_CARD.captures(line).unwrap();

        let id = captures
            .name("id")
            .unwrap()
            .as_str()
            .parse::<u128>()
            .unwrap();
        let winning: Vec<_> = captures
            .name("winning")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|num| num.parse::<u128>().unwrap())
            .collect();
        let hand: Vec<_> = captures
            .name("hand")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|num| num.parse::<u128>().unwrap())
            .collect();
        let winning_cards: u32 = hand
            .iter()
            .filter(|x| winning.contains(x))
            .count()
            .try_into()
            .unwrap();

        Self { id, winning, hand, winning_cards }
    }

    pub fn points(&self) -> u128 {
        if self.winning_cards == 0 {
            0
        } else {
            u128::pow(2, self.winning_cards - 1)
        }
    }
}
