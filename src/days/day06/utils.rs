pub fn read_input() -> &'static str {
    include_str!("input")
}

pub fn parse_input_a<'a>(input: &'a str) -> impl Iterator<Item = Race> + 'a {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse::<usize>().unwrap());
    let records = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse::<usize>().unwrap());

    times
        .zip(records)
        .map(|(time, record)| Race { time, record })
}

pub fn parse_input_b<'a>(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let record = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    Race { time, record }
}

pub struct Race {
    pub time: usize,
    pub record: usize,
}

use std::cmp::Ordering;
use std::ops::Range;

impl Race {
    fn all_timings<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        let pertinent_values = self.time / 2 + 1;
        (0..pertinent_values).map(|held| (self.time - held) * held)
    }

    pub fn beating_the_record(&self) -> usize {
        let all_timings: Vec<_> = self.all_timings().collect();
        let first_bigger = binary_search_first_bigger(&self.record, &all_timings);
        let timings_n = all_timings.len();
        if first_bigger == timings_n {
            return 0;
        }
        let not_beating_the_record = first_bigger * 2;
        let total_number_of_tries = self.time + 1;
        total_number_of_tries - not_beating_the_record
    }
}

fn binary_search_first_bigger(val: &usize, l: &[usize]) -> usize {
    do_binary_search_first_bigger(val, l, 0..l.len())
}
fn do_binary_search_first_bigger(val: &usize, l: &[usize], range: Range<usize>) -> usize {
    if range.is_empty() {
        return range.start;
    }
    let mid = range.len() / 2 + range.start;
    let first_bigger = mid + 1;
    match l[mid].cmp(val) {
        Ordering::Equal => first_bigger,
        Ordering::Less => do_binary_search_first_bigger(val, l, first_bigger..range.end),
        Ordering::Greater => do_binary_search_first_bigger(val, l, range.start..mid),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binary_search_first_bigger_test() {
        let one_elem = [1];
        let two_elem = [1, 2];
        let three_elem = [1, 2, 3];
        let four_elem = [1, 2, 3, 4];

        assert_eq!(binary_search_first_bigger(&0, &one_elem), 0);
        assert_eq!(binary_search_first_bigger(&1, &one_elem), 1);
        assert_eq!(binary_search_first_bigger(&2, &one_elem), 1);
        assert_eq!(binary_search_first_bigger(&0, &two_elem), 0);
        assert_eq!(binary_search_first_bigger(&1, &two_elem), 1);
        assert_eq!(binary_search_first_bigger(&2, &two_elem), 2);
    }
}
