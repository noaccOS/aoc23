#[allow(unused_imports)]
use super::utils::*;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    let all_sections = Sections::parse(input);
    all_sections.sections.iter().map(|section| section.value()).sum::<usize>().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = include_str!("sample_a");
        let result = solve(sample);
        assert_eq!(result, "405");
    }
}
