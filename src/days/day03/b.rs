use super::utils::Game;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    Game::from_input(input)
        .gears()
        .iter()
        .map(|gear| gear.iter().map(|number| number.value).product::<u128>())
        .sum::<u128>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let sample = include_str!("sample_b");
        let result = solve(sample);
        assert_eq!(result, "467835");
    }
}
