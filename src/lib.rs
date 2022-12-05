use regex::Regex;
use std::collections::binary_heap::Iter;
use std::collections::HashSet;
use std::ops::Range;
use std::str::Chars;
use std::str::FromStr;
use std::mem::swap;

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

fn parse_crate<I: Iterator>(inp: &mut I) -> Option<I::Item> {
    let mut res = None;
    for i in 0..3 {
        if let Some(x) = inp.next() {
            if i == 1 {
                res = Some(x);
            }
        }
    }
    inp.next();
    res
}

pub fn day5(input: &str) {
    let parts: Vec<_> = input.split("\r\n\r\n").collect();

    let mut crates: Vec<_> = parts[0]
        .lines()
        .take_while(|&l| l.contains('['))
        .map(|l| {
            let mut xs = l.chars();
            let mut row = Vec::new();
            while let Some(x) = parse_crate(&mut xs) {
                row.push(x);
            }
            row
        })
        .collect();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let insts: Vec<_> = parts[1]
        .lines()
        .map(|l| {
            re.captures_iter(l)
                .map(|m| {
                    let f = |i| m.get(i).unwrap().as_str().parse::<usize>().unwrap();
                    (f(1), f(2), f(3))
                })
                .next()
                .unwrap()
        })
        .collect();

    let cols = crates.iter().map(|v| v.len()).max().unwrap();
    for r in &mut crates {
        r.resize(cols, ' ');
    }
    let mut bcrates = vec![vec![' '; crates.len()]; cols];
    for r in 0..crates.len() {
        for c in 0..crates[r].len() {
            bcrates[c][r] = crates[r][c];
        }
    }
    for r in &mut bcrates {
        r.reverse();
        r.retain(|&c| c != ' ');
    }
    let mut crates9001 = bcrates.clone();

    for inst in &insts {
        let (n, from, to) = inst;

        let end = bcrates[from-1].len();
        let start = end - n;
        let moved: Vec<_> = bcrates[from-1][start..end].iter().cloned().rev().collect();
        bcrates[to-1].extend_from_slice(moved.as_slice());
        bcrates[from-1].truncate(start);

        let moved: Vec<_> = crates9001[from-1][start..end].to_vec();
        crates9001[to-1].extend_from_slice(&moved);
        crates9001[from-1].truncate(start);
    }


    let solt = bcrates.iter().map(|v| v.last().unwrap()).cloned().collect::<String>();
    println!("{}", solt);
    let solt2 = crates9001.iter().map(|v| v.last().unwrap()).cloned().collect::<String>();
    println!("{}", solt2);
}
