use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub day: u8,
}

fn main() {
    let args = Args::parse();
    println!("day {}", args.day);

    if args.day == 1 {
        day01::main();
    }
    if args.day == 2 {
        day02::main();
    }
    if args.day == 3 {
        day03::main();
    }
    if args.day == 4 {
        day04::main();
    }
}
