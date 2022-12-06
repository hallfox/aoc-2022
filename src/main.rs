use std::io;
use std::io::Read;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    day: usize,
}

fn read_input() -> String {
    let mut stdin = io::stdin().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    input
}

fn main() {
    let args = Args::parse();
    let solvers = [
        advent2022::day1,
        advent2022::day2,
        advent2022::day3,
        advent2022::day4,
        advent2022::day5,
        advent2022::day6,
    ];

    let solver = solvers[args.day - 1];
    let input = read_input();
    solver(&input);
}
