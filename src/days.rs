mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
pub mod day_info;
use day_info::DayVariant;

pub fn solve(day: &day_info::DayInfo) -> String {
    match day.variant {
        DayVariant::A => variant_a(day.number),
        DayVariant::B => variant_b(day.number),
        DayVariant::Both => {
            let (res_a, res_b) = variant_both(day.number);
            format!("A: {res_a}\nB: {res_b}")
        }
    }
}

fn variant_a(day: u8) -> String {
    match day {
        1 => day01::solve_a(),
        2 => day02::solve_a(),
        3 => day03::solve_a(),
        4 => day04::solve_a(),
        5 => day05::solve_a(),
        6 => day06::solve_a(),
        7 => day07::solve_a(),
        8 => day08::solve_a(),
        9 => day09::solve_a(),
        10 => day10::solve_a(),
        11 => day11::solve_a(),
        12 => day12::solve_a(),
        13 => day13::solve_a(),
        14 => day14::solve_a(),
        15 => day15::solve_a(),
        16 => day16::solve_a(),
        17 => day17::solve_a(),
        18 => day18::solve_a(),
        19 => day19::solve_a(),
        20 => day20::solve_a(),
        21 => day21::solve_a(),
        22 => day22::solve_a(),
        23 => day23::solve_a(),
        24 => day24::solve_a(),
        25 => day25::solve_a(),
        _ => panic!("Advent of code only has 25 days duh!"),
    }
}

fn variant_b(day: u8) -> String {
    match day {
        1 => day01::solve_b(),
        2 => day02::solve_b(),
        3 => day03::solve_b(),
        4 => day04::solve_b(),
        5 => day05::solve_b(),
        6 => day06::solve_b(),
        7 => day07::solve_b(),
        8 => day08::solve_b(),
        9 => day09::solve_b(),
        10 => day10::solve_b(),
        11 => day11::solve_b(),
        12 => day12::solve_b(),
        13 => day13::solve_b(),
        14 => day14::solve_b(),
        15 => day15::solve_b(),
        16 => day16::solve_b(),
        17 => day17::solve_b(),
        18 => day18::solve_b(),
        19 => day19::solve_b(),
        20 => day20::solve_b(),
        21 => day21::solve_b(),
        22 => day22::solve_b(),
        23 => day23::solve_b(),
        24 => day24::solve_b(),
        25 => day25::solve_b(),
        _ => panic!("Advent of code only has 25 days duh!"),
    }
}

fn variant_both(day: u8) -> (String, String) {
    match day {
        1 => day01::solve(),
        2 => day02::solve(),
        3 => day03::solve(),
        4 => day04::solve(),
        5 => day05::solve(),
        6 => day06::solve(),
        7 => day07::solve(),
        8 => day08::solve(),
        9 => day09::solve(),
        10 => day10::solve(),
        11 => day11::solve(),
        12 => day12::solve(),
        13 => day13::solve(),
        14 => day14::solve(),
        15 => day15::solve(),
        16 => day16::solve(),
        17 => day17::solve(),
        18 => day18::solve(),
        19 => day19::solve(),
        20 => day20::solve(),
        21 => day21::solve(),
        22 => day22::solve(),
        23 => day23::solve(),
        24 => day24::solve(),
        25 => day25::solve(),
        _ => panic!("Advent of code only has 25 days duh!"),
    }
}
