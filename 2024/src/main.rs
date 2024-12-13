use clap::Parser;
use rust_2024::*;

const N_DAYS: u8 = 25;

#[derive(Parser, Debug)]
pub struct Args {
    /// Day to run, if not specified all days will be run
    #[arg(short, long)]
    pub day: Option<u8>,
}

macro_rules! run_day {
    ($day:expr, $module:ident) => {{
        let input = include_str!(concat!("day", stringify!($day), "/input.txt"));
        println!("part1: {}", $module::part1(input));
        println!("part2: {}", $module::part2(input));
    }};
}

fn run_day(day: u8) {
    println!("Running day {}", day);

    match day {
        1 => run_day!(1, day1),
        2 => run_day!(2, day2),
        3 => run_day!(3, day3),
        4 => run_day!(4, day4),
        5 => run_day!(5, day5),
        6 => run_day!(6, day6),
        7 => run_day!(7, day7),
        8 => run_day!(8, day8),
        9 => run_day!(9, day9),
        10 => run_day!(10, day10),
        11 => run_day!(11, day11),
        12 => run_day!(12, day12),
        13 => run_day!(12, day13),
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
