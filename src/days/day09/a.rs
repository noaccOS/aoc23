#[allow(unused_imports)]
use super::utils::*;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    let lines = input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse::<isize>().unwrap()));
    lines.map(|l| predict_next(l)).sum::<isize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = include_str!("sample_a");
        let result = solve(sample);
        assert_eq!(result, "114");
    }
}
