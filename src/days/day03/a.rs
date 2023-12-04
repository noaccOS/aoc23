use super::utils::Game;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    Game::from_input(input)
        .part_numbers()
        .iter()
        .map(|n| n.value)
        .sum::<u128>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let sample = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = solve(sample);
        assert_eq!(result, "4361");
    }
}
