use super::utils::*;
use itertools::Itertools;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    input
        .lines()
        .map(Hand::from_str)
        .sorted_by(|h1, h2| h1.simple_cmp(&h2))
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = include_str!("sample_a");
        let result = solve(sample);
        assert_eq!(result, "6440");
    }
}
