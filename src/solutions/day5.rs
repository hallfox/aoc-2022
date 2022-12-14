use anyhow::Result;

use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter,
    path::PathBuf,
};

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

pub fn day5(input: &str) -> Result<()> {
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

        let end = bcrates[from - 1].len();
        let start = end - n;
        let moved: Vec<_> = bcrates[from - 1][start..end]
            .iter()
            .cloned()
            .rev()
            .collect();
        bcrates[to - 1].extend_from_slice(moved.as_slice());
        bcrates[from - 1].truncate(start);

        let moved: Vec<_> = crates9001[from - 1][start..end].to_vec();
        crates9001[to - 1].extend_from_slice(&moved);
        crates9001[from - 1].truncate(start);
    }

    let solt = bcrates
        .iter()
        .map(|v| v.last().unwrap())
        .cloned()
        .collect::<String>();
    println!("{}", solt);
    let solt2 = crates9001
        .iter()
        .map(|v| v.last().unwrap())
        .cloned()
        .collect::<String>();
    println!("{}", solt2);
    Ok(())
}
