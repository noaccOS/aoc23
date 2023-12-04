use super::utils;
use utils::GameStatus;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    let maximums = [("red", 12), ("green", 13), ("blue", 14)];
    input
        .lines()
        .filter_map(|line| match utils::check_game_validity(line, &maximums) {
            GameStatus::Valid(id) => Some(id),
            GameStatus::Invalid(_) => None,
        })
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
        assert_eq!(result, "8");
    }
}
