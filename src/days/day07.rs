mod a;
mod b;
pub mod utils;

#[allow(dead_code)]
pub fn solve_a() -> String {
    let input = utils::read_input();
    a::solve(input)
}

#[allow(dead_code)]
pub fn solve_b() -> String {
    let input = utils::read_input();
    b::solve(input)
}

#[allow(dead_code)]
pub fn solve() -> (String, String) {
    let input = utils::read_input();
    let a = a::solve(input);
    let b = b::solve(input);
    (a, b)
}
