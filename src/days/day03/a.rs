#[allow(dead_code)]
#[allow(unused_variables)]
pub fn solve(input: &str) -> String {
    panic!("Not implemented yet")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let sample = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = solve(sample);
        assert_eq!(result, "2286");
    }
}
