pub fn read_input() -> &'static str {
    include_str!("input")
}

#[derive(Debug, PartialEq, Clone)]
pub struct Sections {
    pub sections: Vec<Section>,
}

impl Sections {
    pub fn parse(from: &str) -> Self {
        let sections = from.split("\n\n").map(Section::parse).collect();
        Self { sections }
    }

    pub fn parse_b(from: &str) -> Self {
        let sections = from.split("\n\n").map(Section::parse_b).collect();
        Self { sections }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Section {
    kind: SectionKind,
    value: usize,
}

impl Section {
    pub fn parse(from: &str) -> Self {
        let input: Vec<Vec<_>> = from
            .lines()
            .map(|s| s.as_bytes().into_iter().map(|b| *b).collect())
            .collect();

        if let Some(n) = find_split(&input) {
            return Self {
                kind: SectionKind::Horizontal,
                value: n,
            };
        }
        let input = transpose(&input);
        let Some(n) = find_split(&input) else {
            panic!("Not found for input {:?}", input)
        };
        Self {
            kind: SectionKind::Vertical,
            value: n,
        }
    }

    pub fn parse_b(from: &str) -> Self {
        let input: Vec<Vec<_>> = from
            .lines()
            .map(|s| s.as_bytes().into_iter().map(|b| *b).collect())
            .collect();

        if let Some(n) = find_split_b(&input) {
            return Self {
                kind: SectionKind::Horizontal,
                value: n,
            };
        }
        let input = transpose(&input);
        let Some(n) = find_split_b(&input) else {
            panic!("Not found for input {input:?}")
        };
        Self {
            kind: SectionKind::Vertical,
            value: n,
        }
    }

    pub fn value(&self) -> usize {
        match self.kind {
            SectionKind::Horizontal => 100 * self.value,
            SectionKind::Vertical => self.value,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SectionKind {
    Horizontal,
    Vertical,
}

fn find_split(input: &Vec<Vec<u8>>) -> Option<usize> {
    let forwards = input;
    let backwards = input.iter().rev();
    let total_len = forwards.len();

    for l in (2..=total_len).rev() {
        let init = forwards.iter().take(l);
        if ends_with(backwards.clone(), init) {
            return Some(l / 2);
        }
        let rest = forwards.iter().skip(total_len - l);
        if starts_with(backwards.clone(), rest) {
            return Some(l / 2 + total_len - l);
        }
    }
    None
}

fn find_split_b(input: &Vec<Vec<u8>>) -> Option<usize> {
    let forwards = input;
    let backwards = input.iter().rev();
    let total_len = forwards.len();

    for l in (2..=total_len).rev() {
        println!("Len: {l:?}");
        let init = forwards.iter().take(l);
        if ends_with_b(backwards.clone(), init) {
            return Some(l / 2);
        }
        let rest = forwards.iter().skip(total_len - l);
        if starts_with_b(backwards.clone(), rest) {
            return Some(l / 2 + total_len - l);
        }
    }
    None
}

use std::{borrow::Borrow, ops::ControlFlow};

fn starts_with(
    collection: impl IntoIterator<Item = impl Borrow<Vec<u8>>>,
    sub: impl IntoIterator<Item = impl Borrow<Vec<u8>>>,
) -> bool {
    collection
        .into_iter()
        .zip(sub.into_iter())
        .all(|(a, b)| a.borrow() == b.borrow())
}

fn ends_with<T, U, V, W, _T, _U>(collection: T, sub: U) -> bool
where
    V: Borrow<Vec<u8>>,
    W: Borrow<Vec<u8>>,
    T: IntoIterator<Item = V, IntoIter = _T>,
    _T: Iterator<Item = V> + DoubleEndedIterator,
    U: IntoIterator<Item = W, IntoIter = _U>,
    _U: Iterator<Item = W> + DoubleEndedIterator,
{
    starts_with(collection.into_iter().rev(), sub.into_iter().rev())
}

use strsim::generic_hamming;

#[derive(PartialEq, Eq, Debug)]
enum SingleDifferenceFoldResult {
    NoDifference,
    OneDifference,
    Broken,
}

fn starts_with_b(
    collection: impl IntoIterator<Item = impl Borrow<Vec<u8>>>,
    sub: impl IntoIterator<Item = impl Borrow<Vec<u8>>>,
) -> bool {
    use ControlFlow::*;
    use SingleDifferenceFoldResult::*;
    dbg!(collection
        .into_iter()
        .zip(sub.into_iter())
        .try_fold(NoDifference, |acc, (a, b)| {
            match (acc, generic_hamming(a.borrow(), b.borrow())) {
                (x, Ok(0)) => Continue(x),
                (NoDifference, Ok(1)) => Continue(OneDifference),
                _ => Break(Broken),
            }
        }))
    .continue_value()
    .is_some_and(|x| dbg!(x) == OneDifference)
}

fn ends_with_b<T, U, V, W, _T, _U>(collection: T, sub: U) -> bool
where
    V: Borrow<Vec<u8>>,
    W: Borrow<Vec<u8>>,
    T: IntoIterator<Item = V, IntoIter = _T>,
    _T: Iterator<Item = V> + DoubleEndedIterator,
    U: IntoIterator<Item = W, IntoIter = _U>,
    _U: Iterator<Item = W> + DoubleEndedIterator,
{
    starts_with_b(collection.into_iter().rev(), sub.into_iter().rev())
}

fn transpose(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = input.len();
    let m = input.get(0).map(|l| l.len()).unwrap_or(0);
    let mut out: Vec<Vec<u8>> = vec![vec![0u8; n]; m];
    for i in 0..n {
        for j in 0..m {
            out[j][i] = input[i][j];
        }
    }
    out
}

#[test]
fn test() {
    assert_eq!(1,1)
}
