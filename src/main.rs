use clap::Parser;

use day_info::DayInfo;

mod day_info;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = clap::value_parser!(DayInfo))]
    day: DayInfo,
}

fn main() {
    let Cli { day } = Cli::parse();

    println!("{:?}", day);
}
