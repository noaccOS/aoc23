pub fn read_input() -> &'static str {
    include_str!("input")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Character {
    Undefined,
    Present,
    Missing,
}

impl Character {
    pub fn from(ch: &u8) -> Self {
        match ch {
            b'?' => Self::Undefined,
            b'#' => Self::Present,
            b'.' => Self::Missing,
            other => panic!("unexpected character '{other}'"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    input: Vec<Character>,
    sections: Vec<usize>,
}

impl Line {
    pub fn new(input: &str) -> Self {
        let mut iter = input.split_whitespace();
        let input = iter
            .next()
            .unwrap()
            .as_bytes()
            .iter()
            .map(Character::from)
            .collect();
        let sections: Vec<_> = iter
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Self { input, sections }
    }

    pub fn arrangements(&self) -> usize {
        let last_idx = self.input.len() as isize - 1;
        let last_group = self.sections.len() as isize - 1;
        let initial_group_size = 0;
        self._arrangements(last_idx, last_group, initial_group_size)
    }

    fn _arrangements(
        &self,
        idx: isize,
        current_group_idx: isize,
        current_group_size: usize,
    ) -> usize {
        let group_size = match usize::try_from(current_group_idx) {
            Ok(n) => self.sections[n],
            _ => 0,
        };

        if current_group_size > group_size {
            return 0;
        }

        if idx < 0 {
            return if group_size == current_group_size && current_group_idx <= 0 {
                1
            } else {
                0
            };
        }

        let next = idx - 1;
        match self.input[idx as usize] {
            Character::Present => {
                self._arrangements(next, current_group_idx, current_group_size + 1)
            }
            Character::Missing => {
                self.check_missing(next, current_group_idx, current_group_size, group_size)
            }
            Character::Undefined => match current_group_size {
                n if n == group_size => self._arrangements(next, current_group_idx - 1, 0),
                n => {
                    let as_missing =
                        self.check_missing(next, current_group_idx, current_group_size, group_size);
                    let as_present = self._arrangements(next, current_group_idx, n + 1);
                    as_missing + as_present
                }
            },
        }
    }

    fn check_missing(
        &self,
        next: isize,
        current_group_idx: isize,
        current_group_size: usize,
        group_size: usize,
    ) -> usize {
        match current_group_size {
            0 => self._arrangements(next, current_group_idx, 0),
            g if g == group_size => self._arrangements(next, current_group_idx - 1, 0),
            _ => 0,
        }
    }
}
