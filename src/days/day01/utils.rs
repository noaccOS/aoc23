use lazy_static::lazy_static;
use regex::Regex;

pub fn read_input() -> &'static str {
    include_str!("input")
}

lazy_static! {
    static ref FIRST_AND_LAST_DIGITS_RE: Regex =
        Regex::new(r"\D*(?<first>\d)(?:.*(?<last>\d))?\D*").unwrap();
}

pub fn line_to_number(line: &str) -> u128 {
    let captures = FIRST_AND_LAST_DIGITS_RE.captures(line).unwrap();

    let first = captures.name("first").unwrap().as_str();

    let last = match captures.name("last") {
        None => first,
        Some(last) => last.as_str(),
    };

    let res = format!("{first}{last}");
    res.parse::<u128>().unwrap()
}
