use super::utils;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    input
    .lines()
    .map(utils::line_to_number_with_letters)
    .sum::<u128>()
    .to_string()

}
