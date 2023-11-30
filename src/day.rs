use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref FROM_STR_REGEX: Regex =
        Regex::new(r"(?<day>\d+)(?:\-?(?<variant>a|A|b|B))?").unwrap();
}

#[derive(Debug, Clone)]
pub struct Day {
    number: u8,
    variant: DayVariant,
}

impl FromStr for Day {
    type Err = InvalidDayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = FROM_STR_REGEX.captures(s).ok_or(InvalidDayError)?;
        let number = captures
            .name("day")
            .ok_or(InvalidDayError)?
            .as_str()
            .parse::<u8>()
            .expect("Day number too large");
        let variant = match captures.name("variant") {
            None => DayVariant::Both,
            Some(variant) => {
                let variant = variant.as_str().to_lowercase();
                match &variant[..] {
                    "a" => DayVariant::A,
                    "b" => DayVariant::B,
                    _ => return Err(InvalidDayError),
                }
            }
        };

        Ok(Day { number, variant })
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Invalid date")]
pub struct InvalidDayError;

#[derive(Debug, Clone)]
pub enum DayVariant {
    A,
    B,
    Both,
}
