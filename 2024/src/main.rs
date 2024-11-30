use clap::Parser;
use rust_2024::*;

const N_DAYS: u8 = 25;

#[derive(Parser, Debug)]
pub struct Args {
    /// Day to run, if not specified all days will be run
    #[arg(short, long)]
    pub day: Option<u8>,
}

fn run_day(day: u8) {
    println!("Running day {}", day);

    // Use a match statement to call the corresponding day module
    match day {
        1 => day1::main(),
        _ => Default::default(),
    };
}

fn run_all_days() {
    println!("Running all days");

    // Use a loop to iterate over all days and call their main functions
    for day in 1..=N_DAYS {
        run_day(day);
    }
}

fn main() {
    let args = Args::parse();

    match args.day {
        Some(day) => run_day(day),
        None => run_all_days(),
    }
}
