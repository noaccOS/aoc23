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
            let n: f64 = l as f64 / 2f64;
            return Some(n.ceil() as usize);
        }
        let rest = forwards.iter().skip(total_len - l);
        if starts_with(backwards.clone(), rest) {
            let n: f64 = l as f64 / 2f64;
            return Some(n.ceil() as usize + total_len - l);
        }
    }
    None
}

use std::borrow::Borrow;

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
