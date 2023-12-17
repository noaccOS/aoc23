#[allow(unused_imports)]
use super::utils::*;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    input.lines().map(|line| Line::new_long(line).arrangements()).sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = include_str!("sample_b");
        let result = solve(sample);
        assert_eq!(result, "525152");
    }
}
