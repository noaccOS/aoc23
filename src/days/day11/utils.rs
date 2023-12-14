pub fn read_input() -> &'static str {
    include_str!("input")
}

type Galaxy = (usize, usize);

pub struct Space {
    pub galaxies: Vec<Galaxy>
}

use itertools::Itertools;
use std::collections::HashSet;

impl Space {
    fn empty_rows<'a>(input_lines: impl Iterator<Item = &'a str>, _l: usize, empty_space: &usize) -> Vec<usize> {
        let mut count = 0;
        let empty_rows: Vec<usize> = input_lines
            .enumerate()
            .filter_map(|(i, l)| {
                count += 1;
                if l.as_bytes().iter().all_equal() {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        to_direct_access(empty_rows, count, &empty_space)
    }

    fn empty_columns<'a>(input_lines: impl Iterator<Item = &'a str>, l: usize, empty_space: &usize) -> Vec<usize> {
        let mut empty_cols: HashSet<_> = (0..l).collect();
        input_lines.for_each(|line| {
            line.as_bytes().iter().enumerate().for_each(|(i, c)| {
                if *c == b'#' {
                    empty_cols.remove(&i);
                }
            })
        });
        to_direct_access(empty_cols.into_iter().sorted().collect(), l, &empty_space)
    }

    pub fn from_input(input: &str, empty_space: usize) -> Self {
        let mut lines = input.lines().peekable();
        let line_size = lines.peek().unwrap().as_bytes().len();
        let empty_rows = Self::empty_rows(lines.clone(), line_size, &empty_space);
        let empty_columns = Self::empty_columns(lines.clone(), line_size, &empty_space);
        let empty_columns_ref = &empty_columns;
        let galaxies: Vec<_> = lines.enumerate().flat_map(|(j, line)| {
            let j = j + empty_rows[j];
            line.as_bytes().into_iter().enumerate().filter_map(move |(i, character)| match character {
                b'#' => Some((i + empty_columns_ref[i], j)),
                _ => None
            })
        }).collect();

        Self { galaxies }
    }

    pub fn distance(&self, galaxy_a: &(usize, usize), galaxy_b: &(usize, usize)) -> usize {
        let (a_x, a_y) = galaxy_a;
        let (b_x, b_y) = galaxy_b;

        ((*a_x as isize - *b_x as isize).abs() + (*a_y as isize - *b_y as isize).abs()) as usize
    }
}

fn to_direct_access(input: Vec<usize>, size: usize, empty_space: &usize) -> Vec<usize> {
    let mut res = vec![0; size];
    let mut last = 0;
    let mut n = 0;
    for r in input {
        for i in last..r {
            res[i] = n;
        }
        n += empty_space;
        last = r;
    }
    for i in last..size {
        res[i] = n;
    }

    assert_eq!(res.len(), size);
    res
}
