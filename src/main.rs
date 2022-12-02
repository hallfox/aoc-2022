use std::io;
use std::io::Read;
use clap::Parser;
use advent2022::day1;

#[derive(Parser, Debug)]
struct Args {
    day: i32,
}

fn read_input() -> String {
    let mut stdin = io::stdin().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    input
}

fn main() {
    let args = Args::parse();

    let solver = match args.day {
        1 => day1,
        _ => panic!("Invalid day"),
    };

    let input = read_input();
    solver(&input);
}
