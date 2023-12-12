#[allow(unused_imports)]
use super::utils::*;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    let map = Map::from_input(input);
    (map.find_loop().len() / 2).to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = include_str!("sample_a");
        let result = solve(sample);
        let map = Map::from_input(sample);
        println!("{}", map);

        assert_eq!(result, "a");
    }
}
