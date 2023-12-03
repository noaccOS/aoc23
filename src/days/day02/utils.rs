use lazy_static::lazy_static;
use regex::Regex;

pub fn read_input() -> &'static str {
    include_str!("input")
}

pub enum GameStatus {
    Valid(u128),
    Invalid(u128),
}

lazy_static! {
    static ref GAME_ID_RE: Regex = Regex::new(r"^\s*Game (\d+):").unwrap();
}

pub fn check_game_validity(game: &str, maximums: &[(&str, u128)]) -> GameStatus {
    let game_id = GAME_ID_RE
        .captures(game)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u128>()
        .unwrap();

    match maximums
        .iter()
        .map(|(color, max)| (all_color_values(game, color), max))
        .all(|(values, max)| values.iter().all(|x| x <= max))
    {
        true => GameStatus::Valid(game_id),
        false => GameStatus::Invalid(game_id),
    }
}

pub fn power_set(game: &str, colors: &[&str]) -> u128 {
    colors.iter()
       .map(|color| match all_color_values(game, color).iter().max() {
        Some(n) => *n,
        None => 0
       })
       .product()
}

fn all_color_values<'a>(game: &'a str, color: &'a str) -> Vec<u128> {
    let color = format!("(\\d+) {color}");
    Regex::new(&color)
        .unwrap()
        .captures_iter(game)
        .map(|c| c.get(1).unwrap().as_str().parse::<u128>().unwrap())
        .collect()
}
