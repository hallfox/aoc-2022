use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    iter,
    str::FromStr,
};

use anyhow::Result;
use cached::proc_macro::cached;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i64,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, terminated},
    Finish, IResult,
};
use petgraph::{prelude::*, visit};
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Data {
    d: VecDeque<i64>,
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let data = s
            .lines()
            .map(|l| l.parse::<i64>().unwrap())
            .collect::<VecDeque<_>>();
        Ok(Data { d: data })
    }
}

fn mix_code(xs: &mut VecDeque<(usize, i64)>) -> i64 {
    let n = xs.len();

    for i in 0..n {
        let m = xs
            .iter()
            .enumerate()
            .find_map(|(k, (j, _))| if i == *j { Some(k) } else { None })
            .unwrap();
        let p = xs.remove(m).unwrap();
        let q = ((((m as i64 + p.1) % (xs.len() as i64)) + (xs.len() as i64)) % (xs.len() as i64))
            as usize;
        xs.insert(q, p);
    }
    let r = xs
        .iter()
        .enumerate()
        .find_map(|(k, (_, v))| (*v == 0).then_some(k))
        .unwrap();
    [1000, 2000, 3000].iter().map(|&a| xs[(r + a) % n].1).sum()
}

fn part1(d: &Data) -> i64 {
    let mut xs = d.d.iter().cloned().enumerate().collect::<VecDeque<_>>();
    mix_code(&mut xs)
}

fn part2(d: &Data) -> i64 {
    let dec = 811589153;
    let mut xs =
        d.d.iter()
            .map(|x| x * dec)
            .enumerate()
            .collect::<VecDeque<_>>();
    for _ in 0..9 {
        mix_code(&mut xs);
    }
    mix_code(&mut xs)
}

pub fn day20(input: &str) -> Result<()> {
    let data = input.parse::<Data>()?;
    println!("{}", part1(&data));
    println!("{}", part2(&data));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it() {
        let i = "1
2
-3
3
-2
0
4";
        assert!(day20(i).is_ok());
    }
}
