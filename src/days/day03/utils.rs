pub fn read_input() -> &'static str {
    include_str!("input")
}

#[derive(Clone, Debug)]
pub struct Number {
    pub value: u128,
    pub neighbors: Vec<u8>,
    begin_idx: usize,
    end_idx: usize,
}

impl Number {
    fn neighbors(
        &self,
        self_line: &str,
        previous_line: Option<&str>,
        next_line: Option<&str>,
    ) -> Vec<u8> {
        let self_line = self_line.as_bytes();

        let lines_to_check = match (previous_line, next_line) {
            (Some(p), Some(n)) => vec![p.as_bytes(), n.as_bytes()],
            (Some(p), None) => vec![p.as_bytes()],
            (None, Some(n)) => vec![n.as_bytes()],
            _ => vec![],
        };

        let begin_idx = match self.begin_idx {
            0 => 0,
            n => n - 1,
        };

        let mut neighbors: Vec<u8> = (begin_idx..=self.end_idx + 1)
            .flat_map(|i| {
                lines_to_check
                    .iter()
                    .filter_map(move |l| l.get(i).and_then(|&x| Some(x)))
            })
            .collect();

        if let Some(&neighbor) = self_line.get(begin_idx) {
            neighbors.push(neighbor);
        };
        if let Some(&neighbor) = self_line.get(self.end_idx + 1) {
            neighbors.push(neighbor);
        }

        neighbors
    }

    pub fn with_neighbors(
        value: u128,
        begin_idx: usize,
        end_idx: usize,
        self_line: &str,
        previous_line: Option<&str>,
        next_line: Option<&str>,
    ) -> Self {
        let mut s = Number {
            value,
            begin_idx,
            end_idx,
            neighbors: vec![],
        };
        s.neighbors = s.neighbors(self_line, previous_line, next_line);
        s
    }

    pub fn has_in_neighbors(&self, looking_for: &[u8]) -> bool {
        looking_for.iter().any(|n| self.neighbors.contains(n))
    }
}

pub enum CharToDigit {
    Digit(u8),
    NotDigit,
}

#[derive(Clone, Debug)]
pub struct GameLine<'a> {
    pub line: &'a str,
    pub numbers: Vec<Number>,
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

    pub fn part_numbers(&self) -> Vec<Number> {
        let symbols = b"*@=%+$&/-#";
        self.lines
            .iter()
            .flat_map(|l| l.numbers.iter())
            .filter_map(|n| {
                if n.has_in_neighbors(symbols) {
                    Some(n.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
