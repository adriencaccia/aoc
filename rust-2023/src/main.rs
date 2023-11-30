use clap::Parser;

mod day01;

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
}