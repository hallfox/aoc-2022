use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    iter,
    str::FromStr,
};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i32,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, terminated},
    Finish, IResult,
};
use petgraph::{prelude::*, visit};
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug, Clone)]
struct Data {
    d: Vec<Coord>,
}

type Pos = (isize, isize);
type Coord = (isize, isize, isize);

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let data = s
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|i| i.parse::<isize>().unwrap())
                    .collect_tuple::<Coord>()
                    .unwrap()
            })
            .collect_vec();
        Ok(Data { d: data })
    }
}

fn coord_add(a: &Coord, b: &Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn part1(data: &Data) {
    let coords = data.d.iter().collect::<HashSet<_>>();
    let adjs = [
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];
    let sa: usize = data
        .d
        .iter()
        .map(|c| {
            adjs.iter()
                .map(|a| coord_add(c, a))
                .filter(|c| !coords.contains(&c))
                .count()
        })
        .sum();

    println!("{}", sa);
}

fn part2(data: &Data) {
    let (x1, x2, y1, y2, z1, z2) = {
        let min = |a: &Vec<isize>| *a.iter().min().unwrap();
        let max = |a: &Vec<isize>| *a.iter().max().unwrap();
        let (xs, ys, zs): (Vec<_>, Vec<_>, Vec<_>) = data.d.iter().cloned().multiunzip();
        (
            min(&xs) - 1,
            max(&xs) + 1,
            min(&ys) - 1,
            max(&ys) + 1,
            min(&zs) - 1,
            max(&zs) + 1,
        )
    };

    let coords = data.d.iter().collect::<HashSet<_>>();
    let adjs = [
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];

    let mut visited = HashSet::new();
    let mut sa = 0;
    let mut q = VecDeque::new();
    q.push_back((x1, y1, z1));
    visited.insert((x1, y1, z1));
    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        adjs.iter().map(|a| coord_add(&cur, a)).for_each(|n| {
            if coords.contains(&n) {
                sa += 1;
            } else if !visited.contains(&n)
                && n.0 >= x1
                && n.0 <= x2
                && n.1 >= y1
                && n.1 <= y2
                && n.2 >= z1
                && n.2 <= z2
            {
                q.push_back(n);
                visited.insert(n);
            }
        });
    }
    println!("{}", sa);
}

pub fn day18(input: &str) -> Result<()> {
    let data = input.parse::<Data>()?;
    println!("{:?}", data);
    part1(&data);
    part2(&data);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it() {
        let i = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert!(day18(i).is_ok());
    }
}
