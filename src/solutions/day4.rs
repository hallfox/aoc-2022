use anyhow::Result;

use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter,
    path::PathBuf, ops::Range,
};

pub fn day4(input: &str) -> Result<()> {
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
    Ok(())
}
