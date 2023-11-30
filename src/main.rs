use clap::Parser;

use day::Day;

mod day;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = clap::value_parser!(Day))]
    day: Day,
}

fn main() {
    let Cli { day } = Cli::parse();

    println!("{:?}", day);
}
