use std::collections::HashSet;
use std::ops::Range;

pub fn day1(input: &str) {
    let mut calories: Vec<i32> = input
        .split("\r\n\r\n")
        .map(|elf| elf.lines().map(|food| food.parse::<i32>().unwrap()).sum())
        .collect();

    // Part 1
    println!("{}", calories.iter().max().unwrap());

    // Part 2
    calories.sort();
    println!("{}", calories.iter().rev().take(3).sum::<i32>());
}

pub fn day2(input: &str) {
    let game = [
        [1 + 3, 2 + 6, 3 + 0], // A
        [1 + 0, 2 + 3, 3 + 6], // B
        [1 + 6, 2 + 0, 3 + 3], // C
    ];
    let plays: Vec<_> = input
        .lines()
        .map(|strat| {
            strat
                .split_whitespace()
                .map(|play| match play {
                    "A" | "X" => 0,
                    "B" | "Y" => 1,
                    "C" | "Z" => 2,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let score = plays.iter().map(|play| game[play[0]][play[1]]).sum::<i32>();
    println!("{}", score);

    let strats = [
        [3, 4, 8], // A
        [1, 5, 9], // B
        [2, 6, 7], // C
    ];

    let score2 = plays
        .iter()
        .map(|play| strats[play[0]][play[1]])
        .sum::<i32>();

    println!("{}", score2);
}

fn ord(x: char) -> u32 {
    x.into()
}

pub fn day3(input: &str) {
    let xs = input
        .lines()
        .map(|bag| {
            let n = bag.len() / 2;
            let s1 = &bag[0..n];
            let s2 = &bag[n..];
            let b1: HashSet<_> = s1.chars().collect();
            let b2: HashSet<_> = s2.chars().collect();
            *b1.intersection(&b2)
                .next()
                .expect(&format!("Missing common string {:?} {:?}", s1, s2))
        })
        .collect::<Vec<_>>();

    fn prioritize(x: char) -> u32 {
        if x.is_lowercase() {
            ord(x) - ord('a') + 1
        } else {
            ord(x) - ord('A') + 27
        }
    }

    let solt: u32 = xs.iter().map(|&x| prioritize(x)).sum();

    println!("{}", solt);

    let ys: Vec<HashSet<_>> = input.lines().map(|bag| bag.chars().collect()).collect();
    let solt2: u32 = ys
        .chunks(3)
        .map(|arrs| {
            arrs[0]
                .iter()
                .filter(|&x| arrs[1].contains(x))
                .find(|&x| arrs[2].contains(x))
                .expect(&format!("Missing common string {:?}", arrs))
        })
        .map(|&x| prioritize(x))
        .sum();
    println!("{}", solt2);
}

pub fn day4(input: &str) {
    let ids: Vec<_> = input
        .lines()
        .map(|xs| {
            let ys: Vec<_> = xs
                .split(',')
                .map(|x| {
                    let ys: Vec<_> = x.split('-').map(|a| a.parse::<i32>().unwrap()).collect();
                    ys[0]..ys[1]
                })
                .collect();
            (ys[0].clone(), ys[1].clone())
        })
        .collect();

    let covers = ids
        .iter()
        .filter(|(a, b)| {
            let contains = |a: &Range<i32>, b: &Range<i32>| a.start <= b.start && a.end >= b.end;
            contains(a, b) || contains(b, a)
        })
        .count();
    println!("{}", covers);

    let overlaps = ids
        .iter()
        .filter(|(a, b)| {
            let contains = |a: &Range<i32>, b: &Range<i32>| a.start <= b.start && a.end >= b.start;
            contains(a, b) || contains(b, a)
        })
        .count();
    println!("{}", overlaps);
}
