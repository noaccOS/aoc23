use super::utils::Card;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    input
        .lines()
        .map(Card::from_input)
        .map(|c| c.points())
        .sum::<u128>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = include_str!("sample_a");
        let result = solve(sample);
        assert_eq!(result, "13");
    }
}
