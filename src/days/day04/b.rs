use super::utils::Card;
use super::utils;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    let scratchcards: Vec<Card> = input.lines().map(Card::from_input).collect();

    utils::total_scratchcards(&scratchcards).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn sample() {
        let sample = include_str!("sample_b");
        let result = solve(sample);
        assert_eq!(result, "30");
    }
}
