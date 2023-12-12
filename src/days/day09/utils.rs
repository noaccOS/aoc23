pub fn read_input() -> &'static str {
    include_str!("input")
}

pub fn predict_next<T>(line: T) -> isize
where T: IntoIterator<Item = isize> + Clone {
    let line: Vec<isize> = line.into_iter().collect();
    if line.iter().all(|&x| x == 0) {
        return 0;
    }
    let next_iteration: Vec<_> = line.iter().map_windows(|[&first, &second]| second - first).collect();
    let next_prediction = predict_next(next_iteration);
    let last = line.last().unwrap() + next_prediction;

    last
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn predict_next_test() {
        let v = [0, 3, 6, 9, 12, 15];
        assert_eq!(predict_next(v), 18)
    }
}
