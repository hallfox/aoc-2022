use clap::Parser;
use std::io;
use std::io::Read;

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
        advent2022::day7,
        advent2022::day8,
        advent2022::day9,
        advent2022::day10,
        advent2022::day11,
        advent2022::day12,
        advent2022::day13,
        advent2022::day14,
        advent2022::day15,
        advent2022::day16,
        advent2022::day17,
        advent2022::day18,
        advent2022::day19,
    ];

    let solver = solvers[args.day - 1];
    let input = read_input();
    solver(&input).unwrap();
}
