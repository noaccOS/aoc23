#![feature(iter_map_windows)]

use clap::Parser;

use days::day_info::DayInfo;

mod days;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = clap::value_parser!(DayInfo))]
    day: DayInfo,
}

fn main() {
    let Cli { day } = Cli::parse();
    let result = days::solve(&day);
    println!("{result}");
}
