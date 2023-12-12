pub fn read_input() -> &'static str {
    include_str!("input")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Vertical,
    Horizontal,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    Ground,
    Starting,
}

use std::fmt::Display;
use std::fmt::Formatter;

impl TileType {
    fn from_char(input: &u8) -> Self {
        match input {
            b'|' => Self::Vertical,
            b'-' => Self::Horizontal,
            b'L' => Self::UpRight,
            b'J' => Self::UpLeft,
            b'7' => Self::DownLeft,
            b'F' => Self::DownRight,
            b'.' => Self::Ground,
            b'S' => Self::Starting,
            other => panic!("unexpected character {other} in input"),
        }
    }
}

impl Display for TileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Vertical => write!(f, "│")?,
            Self::Horizontal => write!(f, "─")?,
            Self::UpRight => write!(f, "└")?,
            Self::UpLeft => write!(f, "┘")?,
            Self::DownLeft => write!(f, "┐")?,
            Self::DownRight => write!(f, "┌")?,
            Self::Ground => write!(f, " ")?,
            Self::Starting => write!(f, "┼")?,
        };

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Map {
    tiles: Vec<Vec<TileType>>,
    starting: (usize, usize),
}

impl Map {
    pub fn from_input(input: &str) -> Self {
        let tiles: Vec<Vec<TileType>> = input
            .lines()
            .map(|line| line.as_bytes().iter().map(TileType::from_char).collect())
            .collect();
        let starting = tiles
            .iter()
            .enumerate()
            .map(move |(i, row)| {
                row.iter().enumerate().filter_map(move |(j, elem)| {
                    if *elem == TileType::Starting {
                        Some((j, i))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .next()
            .unwrap();
        Self { tiles, starting }
    }

    #[inline]
    fn next_move<'a>(
        &'a self,
        x: isize,
        y: isize,
        from: Direction,
        acc: &mut Vec<TileType>,
    ) -> Option<Vec<TileType>> {
        if x < 0 || y < 0 {
            return None;
        }
        let tile = *self.tiles.get(y as usize)?.get(x as usize)?;
        acc.push(tile);
        if tile == TileType::Starting {
            return Some(acc.clone());
        }

        match from {
            Direction::Up => match tile {
                TileType::Vertical => self.next_move(x, y + 1, Direction::Up, acc),
                TileType::UpLeft => self.next_move(x - 1, y, Direction::Right, acc),
                TileType::UpRight => self.next_move(x + 1, y, Direction::Left, acc),
                _ => None,
            },
            Direction::Down => match tile {
                TileType::Vertical => self.next_move(x, y - 1, Direction::Down, acc),
                TileType::DownLeft => self.next_move(x - 1, y, Direction::Right, acc),
                TileType::DownRight => self.next_move(x + 1, y, Direction::Left, acc),
                _ => None,
            },
            Direction::Left => match tile {
                TileType::Horizontal => self.next_move(x + 1, y, Direction::Left, acc),
                TileType::UpLeft => self.next_move(x, y - 1, Direction::Down, acc),
                TileType::DownLeft => self.next_move(x, y + 1, Direction::Up, acc),
                _ => None,
            },
            Direction::Right => match tile {
                TileType::Horizontal => self.next_move(x - 1, y, Direction::Right, acc),
                TileType::UpRight => self.next_move(x, y - 1, Direction::Down, acc),
                TileType::DownRight => self.next_move(x, y + 1, Direction::Up, acc),
                _ => None,
            },
        }
    }

    pub fn find_loop(&self) -> Vec<TileType> {
        let (x, y) = self.starting;
        let x = x as isize;
        let y = y as isize;
        let go_up = (x, y - 1, Direction::Down);
        let go_down = (x, y + 1, Direction::Up);
        let go_left = (x - 1, y, Direction::Right);
        let go_right = (x + 1, y, Direction::Left);
        [go_up, go_down, go_left, go_right]
            .iter()
            .filter_map(|(x, y, from)| self.next_move(*x, *y, from.clone(), &mut vec![]))
            .next()
            .unwrap()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for line in self.tiles.iter() {
            for tile in line {
                write!(f, "{}", tile)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
