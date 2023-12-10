pub fn read_input() -> &'static str {
    include_str!("input")
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn from_ascii(value: &u8) -> Card {
        match value {
            b'A' => Card::Ace,
            b'K' => Card::King,
            b'Q' => Card::Queen,
            b'J' => Card::Jack,
            b'T' => Card::Ten,
            b'9' => Card::Nine,
            b'8' => Card::Eight,
            b'7' => Card::Seven,
            b'6' => Card::Six,
            b'5' => Card::Five,
            b'4' => Card::Four,
            b'3' => Card::Three,
            b'2' => Card::Two,
            _ => panic!("Card constructor: found character '{}' as a card", value),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum HandType {
    HighCard(Card),
    Pair(Card),
    TwoPair(Card, Card),
    ThreeOAK(Card),
    FullHouse(Card, Card),
    FourOAK(Card),
    FiveOAK(Card),
}

use itertools::Itertools;

impl HandType {
    pub fn from_hand(hand: &[Card]) -> HandType {
        let groups: Vec<_> = hand
            .into_iter()
            .sorted()
            .group_by(|card| *card)
            .into_iter()
            .map(|(key, values)| (key, values.count()))
            .collect();

        let groups_of_two: Vec<_> = groups.iter().filter(|(k, v)| *v == 2).collect();

        match groups
            .iter()
            .max_by(|&(_, v1), &(_, v2)| v1.cmp(&v2))
            .unwrap()
        {
            (card, 5) => HandType::FiveOAK(**card),
            (card, 4) => HandType::FourOAK(**card),
            (card, 3) => match groups_of_two.len() {
                1 => HandType::FullHouse(**card, *groups_of_two[0].0),
                0 => HandType::ThreeOAK(**card),
                n => panic!(
                    "Found a group of 3 and multiple ({n}) groups of 2 with hand {:?}",
                    hand
                ),
            },
            (card, 2) => match groups_of_two.len() {
                1 => HandType::Pair(**card),
                2 => HandType::TwoPair(*groups_of_two[0].0, *groups_of_two[1].0),
                n => panic!("Found a hand with {n} groups of 2"),
            },
            (_, 1) => {
                let max_card = hand.into_iter().max().unwrap();
                HandType::HighCard(*max_card)
            }
            _ => panic!("Groups are weird {:?}", groups),
        }
    }

    pub fn simple_cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match (self, other) {
            (Self::HighCard(_), Self::HighCard(_)) => Equal,
            (Self::Pair(_), Self::Pair(_)) => Equal,
            (Self::TwoPair(_, _), Self::TwoPair(_, _)) => Equal,
            (Self::ThreeOAK(_), Self::ThreeOAK(_)) => Equal,
            (Self::FullHouse(_, _), Self::FullHouse(_, _)) => Equal,
            (Self::FourOAK(_), Self::FourOAK(_)) => Equal,
            (Self::FiveOAK(_), Self::FiveOAK(_)) => Equal,
            _ => self.cmp(&other),
        }
    }
}

pub struct Hand {
    pub cards: Vec<Card>,
    pub hand_type: HandType,
    pub bid: usize,
}

impl Hand {
    pub fn from_str(input: &str) -> Hand {
        let mut groups = input.split_whitespace();
        let hand: Vec<Card> = groups
            .next()
            .unwrap()
            .as_bytes()
            .into_iter()
            .map(Card::from_ascii)
            .collect();
        let hand_type = HandType::from_hand(hand.as_slice());
        let bid = groups.next().unwrap().parse::<usize>().unwrap();

        Hand {
            cards: hand,
            hand_type,
            bid,
        }
    }

    pub fn simple_cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match self.hand_type.simple_cmp(&other.hand_type) {
            Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .filter_map(|(s, o)| match s.cmp(&o) {
                    Equal => None,
                    res => Some(res),
                })
                .next()
                .unwrap_or(Equal),
            other => other,
        }
    }
}
