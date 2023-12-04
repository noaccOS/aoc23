pub fn read_input() -> &'static str {
    include_str!("input")
}

use std::collections::HashMap;
use by_address::ByAddress;

#[derive(Clone, Debug)]
pub struct Number<'a> {
    pub value: u128,
    pub neighbors: Vec<&'a u8>,
}

impl<'a> Number<'a> {
    fn neighbors(
        begin_idx: usize,
        end_idx: usize,
        self_line: &'a str,
        previous_line: Option<&'a str>,
        next_line: Option<&'a str>,
    ) -> Vec<&'a u8> {
        let self_line = self_line.as_bytes();

        let lines_to_check = match (previous_line, next_line) {
            (Some(p), Some(n)) => vec![p.as_bytes(), n.as_bytes()],
            (Some(p), None) => vec![p.as_bytes()],
            (None, Some(n)) => vec![n.as_bytes()],
            _ => vec![],
        };

        let begin = match begin_idx {
            0 => 0,
            n => n - 1,
        };

        let mut neighbors: Vec<&u8> = (begin..=end_idx + 1)
            .flat_map(|i| lines_to_check.iter().filter_map(move |l| l.get(i)))
            .collect();

        if begin_idx != 0
            && let Some(neighbor) = self_line.get(begin)
        {
            neighbors.push(neighbor);
        };
        if let Some(neighbor) = self_line.get(end_idx + 1) {
            neighbors.push(neighbor);
        }

        neighbors
    }

    pub fn with_neighbors(
        value: u128,
        begin_idx: usize,
        end_idx: usize,
        self_line: &'a str,
        previous_line: Option<&'a str>,
        next_line: Option<&'a str>,
    ) -> Self {
        Self {
            value,
            neighbors: Self::neighbors(begin_idx, end_idx, self_line, previous_line, next_line),
        }
    }

    pub fn has_in_neighbors(&self, looking_for: &[u8]) -> bool {
        self.neighbors.iter().any(|&x| looking_for.contains(x))
    }
}

pub enum CharToDigit {
    Digit(u8),
    NotDigit,
}

#[derive(Clone, Debug)]
pub struct GameLine<'a> {
    pub line: &'a str,
    pub numbers: Vec<Number<'a>>,
}

impl<'a> GameLine<'a> {
    fn to_digit(ch: u8) -> CharToDigit {
        if (b'0'..=b'9').contains(&ch) {
            CharToDigit::Digit(ch - b'0')
        } else {
            CharToDigit::NotDigit
        }
    }

    pub fn from_full_window(line: &'a str, previous: &'a str, next: &'a str) -> Self {
        Self::from_window(line, Some(previous), Some(next))
    }

    pub fn from_window(line: &'a str, previous: Option<&'a str>, next: Option<&'a str>) -> Self {
        let mut numbers: Vec<Number> = vec![];
        let line_bytes = line.as_bytes();
        let mut end_idx = None;
        let mut cur_mul: u32 = 1;
        let mut cur_number: u128 = 0;
        for (idx, new_char) in line_bytes.iter().enumerate().rev() {
            if let CharToDigit::Digit(digit) = Self::to_digit(*new_char) {
                let new_digit: u128 = u128::from(cur_mul * u32::from(digit));
                cur_number += new_digit;
                cur_mul *= 10;
                if end_idx == None {
                    end_idx = Some(idx);
                }
            } else if let Some(end) = end_idx {
                numbers.push(Number::with_neighbors(
                    cur_number,
                    idx + 1,
                    end,
                    line,
                    previous,
                    next,
                ));

                end_idx = None;
                cur_mul = 1;
                cur_number = 0;
            }
        }

        if let Some(end) = end_idx {
            numbers.push(Number::with_neighbors(
                cur_number, 0, end, line, previous, next,
            ));
        }

        Self { line, numbers }
    }
}

#[derive(Clone, Debug)]
pub struct Game<'a> {
    lines: Vec<GameLine<'a>>,
}

impl<'a> Game<'a> {
    pub fn from_input(game: &'a str) -> Self {
        let mut game_lines: Vec<GameLine<'a>> = game
            .lines()
            .map_windows(|[prev, cur, next]| GameLine::from_full_window(cur, prev, next))
            .collect();

        let count = game.lines().count();
        if count >= 2 {
            let mut lines = game.lines();
            let first_line = lines.next().unwrap();
            let second_line = lines.next().unwrap();
            let first_partial_window = GameLine::from_window(first_line, None, Some(second_line));

            let mut lines = game.lines();
            let penultimate_line = lines.nth(count - 2).unwrap();
            let last_line = lines.last().unwrap();
            let last_partial_window =
                GameLine::from_window(last_line, Some(penultimate_line), None);

            game_lines.push(first_partial_window);
            game_lines.push(last_partial_window);
        }

        Self { lines: game_lines }
    }

    pub fn part_numbers(&self) -> Vec<&Number> {
        let symbols = b"*@=%+$&/-#";
        self.lines
            .iter()
            .flat_map(|l| l.numbers.iter())
            .filter(|n| n.has_in_neighbors(symbols))
            .collect()
    }

    fn populate_gears<'b>(
        mut gears_map: HashMap<ByAddress<&'b u8>, Vec<&'b Number<'b>>>,
        element: &'b Number,
    ) -> HashMap<ByAddress<&'b u8>, Vec<&'b Number<'b>>> {
        for gear in element.neighbors.iter().filter(|x| ***x == b'*') {
            let gear_neighbors = gears_map.entry(ByAddress(gear)).or_insert(vec![]);
            (*gear_neighbors).push(element);
        }
        gears_map
    }

    pub fn gears(&self) -> Vec<Vec<&Number>> {
        self.lines
            .iter()
            .flat_map(|l| l.numbers.iter())
            .filter(|n| n.has_in_neighbors(b"*"))
            .fold(HashMap::new(), Self::populate_gears)
            .into_values()
            .filter(|neighbors| neighbors.len() >= 2)
            .collect()
    }
}
