use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub fn read_input() -> &'static str {
    include_str!("input")
}

const NUMBER_REGEX: &str = r"(\d|one|two|three|four|five|six|seven|eight|nine)";

lazy_static! {
    static ref FIRST_AND_LAST_DIGITS_RE: Regex =
        Regex::new(r"\D*(?<first>\d)(?:.*(?<last>\d))?\D*").unwrap();
    static ref FIRST_AND_LAST_DIGITS_WITH_LETTERS_RE: Regex = Regex::new(&format!(
        r".*?(?<first>{NUMBER_REGEX})(?:.*(?<last>{NUMBER_REGEX}))?.*?"
    ))
    .unwrap();
    static ref LETTER_TO_DIGIT: HashMap<&'static str, &'static str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
}

pub fn line_to_number(line: &str) -> u128 {
    let captures = FIRST_AND_LAST_DIGITS_RE.captures(line);
    if let None = captures {
        return 0;
    }

    let captures = captures.unwrap();

    let first = match captures.name("first") {
        None => "0",
        Some(first) => first.as_str(),
    };

    let last = match captures.name("last") {
        None => first,
        Some(last) => last.as_str(),
    };

    let res = format!("{first}{last}");
    res.parse::<u128>().unwrap()
}

pub fn line_to_number_with_letters(line: &str) -> u128 {
    let captures = FIRST_AND_LAST_DIGITS_WITH_LETTERS_RE.captures(line);
    if let None = captures {
        return 0;
    }

    let captures = captures.unwrap();

    let first = match captures.name("first") {
        None => "0",
        Some(first) => {
            let first_str = first.as_str();
            LETTER_TO_DIGIT.get(first_str).unwrap_or(&first.as_str())
        }
    };

    let last = match captures.name("last") {
        None => first,
        Some(last) => {
            let last_str = last.as_str();
            LETTER_TO_DIGIT.get(last_str).unwrap_or(&last.as_str())
        }
    };

    let res = format!("{first}{last}");
    res.parse::<u128>().unwrap()
}
