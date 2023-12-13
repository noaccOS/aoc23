#[allow(unused_imports)]
use super::utils::*;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    let map = Map::from_input(input);
    map.find_area().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = include_str!("sample_b");
        println!("{}", Map::from_input(sample));
        let result = solve(sample);
        assert_eq!(result, "8");
    }
}
