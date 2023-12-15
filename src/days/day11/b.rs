#[allow(unused_imports)]
use super::utils::*;
use itertools::Itertools;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    let empty_space = 1000000 - 1;
    let space = Space::from_input(input, empty_space);
    space
        .galaxies
        .iter()
        .combinations(2)
        .map(|galaxies| space.distance(&galaxies[0], &galaxies[1]))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = include_str!("sample_a");
        let result = solve(sample);
        assert_eq!(result, "1030");
    }
}
