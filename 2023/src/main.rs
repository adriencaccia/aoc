use clap::Parser;
use rust_2023::*;

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
        1 => day01::main(),
        2 => day02::main(),
        3 => day03::main(),
        4 => day04::main(),
        5 => {
            day05::main();
            (0, 0)
        }
        6 => day06::main(),
        7 => day07::main(),
        8 => {
            day08::main();
            (0, 0)
        }
        9 => {
            day09::main();
            (0, 0)
        }
        10 => day10::main(),
        11 => {
            day11::main();
            (0, 0)
        }
        12 => {
            day12::main();
            (0, 0)
        }
        13 => day13::main(),
        14 => day14::main(),
        15 => day15::main(),
        16 => day16::main(),
        17 => day17::main(),
        18 => {
            day18::main();
            (0, 0)
        }
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
