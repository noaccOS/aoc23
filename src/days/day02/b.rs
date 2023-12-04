use super::utils;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    let colors = ["red", "green", "blue"];
    input.lines()
         .map(|line| utils::power_set(line, &colors))
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
        assert_eq!(result, "2286");
    }
}
