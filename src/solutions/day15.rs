use std::{cmp::Ordering, collections::HashSet, str::FromStr};

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
use rayon::{array::IntoIter, prelude::*};

#[derive(Debug, Clone)]
struct Data {
    data: Vec<(Point, Point)>,
}

type Point = (isize, isize);

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let data = s
            .lines()
            .map(|l| {
                let (x1, y1, x2, y2) = l
                    .chars()
                    .filter(|c| c.is_numeric() || c.is_whitespace() || *c == '-')
                    .collect::<String>()
                    .split_whitespace()
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                ((x1, y1), (x2, y2))
            })
            .collect_vec();
        Ok(Data { data })
    }
}

fn dist(a: Point, b: Point) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn part1(data: &Data, y: isize) -> usize {
    let bound = {
        let mut width = 0;
        let mut left = 0;
        let mut d = 0;
        for (p1 @ (x1, y1), p2 @ (x2, y2)) in &data.data {
            width = width.max(*x1).max(*x2);
            left = left.min(*x1).min(*x2);
            d = d.max(dist(*p1, *p2));
        }
        (left - d as isize, width + d as isize)
    };

    let occ = data
        .data
        .iter()
        .map(|(p1, p2)| p2)
        .cloned()
        .chain(data.data.iter().map(|(p1, p2)| p1).cloned())
        .collect::<HashSet<_>>();

    let mut candidates = Vec::new();
    for ((x1, y1), (x2, y2)) in &data.data {
        let d = dist((*x1, *y1), (*x2, *y2));
        let yd = y.abs_diff(*y1);
        let cs = (*x1 - (d as isize - yd as isize)..=*x1 + (d as isize - yd as isize))
            .into_par_iter()
            .filter(|x| {
                let z = dist((*x, y), (*x1, *y1));
                !occ.contains(&(*x, y)) && z <= d
            })
            .collect::<HashSet<_>>();
        candidates.push(cs);
    }

    let xs = candidates
        .into_iter()
        .reduce(|acc, c| acc.union(&c).cloned().collect::<HashSet<_>>())
        .unwrap();
    xs.len()
}

fn neighbors(p: Point) -> impl Iterator<Item = Point> {
    IntoIterator::into_iter([(1, 0), (0, 1), (-1, 0), (0, -1)]).map(move |n| (p.0 + n.0, p.1 + n.1))
}

fn line(p1: Point, p2: Point) -> impl Iterator<Item = Point> {
    let d1 = (p2.0 - p1.0).signum();
    let d2 = (p2.1 - p1.1).signum();
    itertools::iterate(p1, move |&p| (p.0 + d1, p.1 + d2)).take_while(move |&p| p != p2)
}

fn intersect<'a>(d: &'a Data, p: &Point) -> Option<&'a Point> {
    d.data
        .iter()
        .find(|(s, b)| {
            let r = dist(*s, *b);
            dist(*s, *p) <= r
        })
        .map(|(s, _)| s)
}

fn part2(data: &Data, limit: isize) -> isize {
    let occ = data
        .data
        .iter()
        .flat_map(|(a, b)| vec![a, b].into_iter())
        .cloned()
        .collect::<HashSet<_>>();

    let candidates = data
        .data
        .iter()
        .flat_map(|(s, b)| {
            let r = dist(*s, *b) as isize + 1;
            IntoIterator::into_iter([
                (s.0 + r, s.1),
                (s.0, s.1 + r),
                (s.0 - r, s.1),
                (s.0, s.1 - r),
            ])
            .tuple_windows()
            .flat_map(|(a, b)| line(a, b))
        })
        .filter(|(x, y)| 0 <= *x && *x <= limit && 0 <= *y && *y <= limit)
        .filter(|p| !occ.contains(p))
        .filter(|p| {
            let i = intersect(data, p);
            i.is_none()
        })
        .unique()
        .next()
        .unwrap();

    candidates.0 * limit + candidates.1
}

pub fn day15(input: &str) -> Result<()> {
    let data = input.parse::<Data>()?;
    println!("{}", part1(&data, 2_000_000));
    println!("{}", part2(&data, 4_000_000));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it() {
        let i = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        assert!(day15(i).is_ok());
    }
}
