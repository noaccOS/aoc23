use super::utils::*;

pub fn solve(input: &str) -> String {
    let race = parse_input_b(input);
    race.beating_the_record().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = include_str!("sample_b");
        let result = solve(sample);
        assert_eq!(result, "71503");
    }
}
