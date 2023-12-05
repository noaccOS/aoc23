pub fn read_input() -> &'static str {
    include_str!("input")
}

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PARSE_CARD: Regex =
        Regex::new(r"^Card\s*(?<id>\d+):\s*(?<winning>\d[\d\s]*?)\s*\|\s*(?<hand>\d.*)$").unwrap();
}

#[allow(dead_code)]
pub struct Card {
    id: usize,
    winning: Vec<u128>,
    hand: Vec<u128>,
    winning_cards: u32,
}

impl Card {
    pub fn from_input(line: &str) -> Self {
        let captures = PARSE_CARD.captures(line).unwrap();

        let id = captures
            .name("id")
            .unwrap()
            .as_str()
            .parse::<usize>()
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

use std::iter;
use std::collections::VecDeque;
pub fn total_scratchcards(scratchcards: &[Card]) -> u128 {
    // cards are 1-based indexed. scratchcards.len() + 0 at the front
    let mut repetitions: VecDeque<u128> = iter::repeat(1).take(scratchcards.len()).collect();
    repetitions.push_front(0);
    for card in scratchcards {
        let reps = repetitions[card.id];
        let winning_cards: usize = card.winning_cards.try_into().unwrap();
        let range = card.id + 1 .. card.id + 1 + winning_cards;
        for id in range {
            repetitions[id] += reps;
        }
    }
    repetitions.iter().sum()
}
