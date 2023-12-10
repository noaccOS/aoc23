use super::utils::*;

pub fn solve(input: &str) -> String {
    parse_input_a(input).map(|race| race.beating_the_record()).product::<usize>().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = include_str!("sample_a");
        let result = solve(sample);
        assert_eq!(result, "288");
    }
}
