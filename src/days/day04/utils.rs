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
        let winning = captures
            .name("winning")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|num| num.parse::<u128>().unwrap())
            .collect();
        let hand = captures
            .name("hand")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|num| num.parse::<u128>().unwrap())
            .collect();

        Self { id, winning, hand }
    }

    fn build_winning_hand(mut hashmap: HashMap<u128, u128>, elem: &u128) -> HashMap<u128, u128> {
        let cur_value = hashmap.entry(*elem).or_insert(0);
        *cur_value += 1;
        hashmap
    }

    pub fn points(&self) -> u128 {
        let winning_cards: u32 = self
            .hand
            .iter()
            .filter(|x| self.winning.contains(x))
            .count()
            .try_into()
            .unwrap();

        if winning_cards == 0 {
            0
        } else {
            u128::pow(2, winning_cards - 1)
        }
    }
}
