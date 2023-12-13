pub fn read_input() -> &'static str {
    include_str!("input")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        acc: &mut Vec<(TileType, isize, isize)>,
    ) -> Option<Vec<(TileType, isize, isize)>> {
        if x < 0 || y < 0 {
            return None;
        }
        let tile = *self.tiles.get(y as usize)?.get(x as usize)?;
        acc.push((tile, x, y));
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

    pub fn find_loop(&self) -> Vec<(TileType, isize, isize)> {
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

    pub fn find_area(&self) -> usize {
        let full_loop = self.find_loop();
        let base_loop: Vec<_> = full_loop
            .iter()
            .filter(|(t, _, _)| *t != TileType::Vertical && *t != TileType::Horizontal)
            .map(|t| *t)
            .collect();
        gauss(&base_loop) - perimeter_area(&full_loop)
    }
}

use itertools::Itertools;
use std::collections::HashMap;

fn perimeter_area(full_loop: &Vec<(TileType, isize, isize)>) -> usize {
    let mut full_loop = full_loop.clone();
    full_loop.sort_unstable_by_key(|tile| tile.0);
    let items: HashMap<TileType, usize> = full_loop
        .iter()
        .group_by(|tile| tile.0)
        .into_iter()
        .map(|(key, values)| (key, values.count()))
        .collect();

    let simple = items.get(&TileType::Horizontal).unwrap_or(&0)
        + items.get(&TileType::Vertical).unwrap_or(&0);
    let simple = simple / 2;

    let ones_and_quarters = [
        TileType::DownLeft,
        TileType::DownRight,
        TileType::UpLeft,
        TileType::UpRight,
    ]
    .iter()
    .map(|k| items.get(k).unwrap_or(&0))
    .map(|v| (*v as f64 / 2f64, v % 2))
    .collect::<Vec<_>>();
    let ones: usize = ones_and_quarters.iter().map(|x| x.0).sum::<f64>() as usize;
    let quarters: usize = ones_and_quarters.iter().map(|x| x.1).sum::<usize>() / 4;

    simple + ones + quarters
}

fn gauss(polygon: &Vec<(TileType, isize, isize)>) -> usize {
    let (_, x_first, y_first) = polygon[0];
    let (_, x_last, y_last) = polygon.last().unwrap();
    let sum: isize = polygon
        .iter()
        .map_windows(|[(_, x1, y1), (_, x2, y2)]| x1 * y2 - x2 * y1)
        .sum();
    let sum: isize = sum + x_last * y_first - x_first * y_last;
    (sum.abs() / 2) as usize
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
