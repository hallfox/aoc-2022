use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    iter,
    str::FromStr,
};

use anyhow::Result;
use cached::proc_macro::cached;
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
use petgraph::prelude::*;
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Data {
    data: BTreeMap<String, Valve>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    pressure: i32,
    next: Vec<String>,
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let r = Regex::new(r"Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();
        let data = s
            .lines()
            .map(|l| {
                let c = r.captures_iter(l).next().unwrap();
                (
                    c[1].to_string(),
                    Valve {
                        pressure: c[2].parse::<i32>().unwrap(),
                        next: c[3].split(", ").map(String::from).collect_vec(),
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();
        Ok(Data { data })
    }
}

#[cached(
    key = "(i32, String, BTreeSet<String>)",
    convert = r#"{ (n, c.clone(), open.clone()) }"#
)]
fn find_best(d: &Data, n: i32, c: String, open: BTreeSet<String>) -> i32 {
    if n <= 1 {
        return 0;
    }

    let v = &d.data[&c];

    let alt = (v.pressure > 0 && !open.contains(&c)).then(|| {
        let mut open1 = open.clone();
        open1.insert(c);
        v.next
            .iter()
            .cloned()
            .map(|z| find_best(d, n - 2, z, open1.clone()))
            .max()
            .unwrap()
    });

    let walk = v
        .next
        .iter()
        .cloned()
        .map(|z| find_best(d, n - 1, z, open.clone()))
        .max()
        .unwrap();

    alt.map(|m1| walk.max((v.pressure * (n - 1)) + m1)).unwrap_or(walk)
}

// #[cached(
//     key = "(i32, Vec<String>, BTreeSet<String>)",
//     convert = r#"{ (n, c.clone(), open.clone()) }"#
// )]
// fn find_best_with_help(d: &Data, n: i32, c: Vec<String>, open: BTreeSet<String>) -> i32 {
//     if n <= 1 {
//         return 0;
//     }

//     let vs = c.iter().map(|v| &d.data[v]).collect_vec();

//     // Open both
//     let mut open1 = open.clone();
//     let alt = vs.iter().scan(&mut open1, |open, v|
//         if v.pressure > 0 && !open.contains(&c) {

//         open.insert(c);
//         v.pressure * (n-1)
//         } else {
//             0
//         }
//     )
//     .sum();

//     vs.iter().map(|v|
//         v.next
//             .iter()
//             .cloned()
//     )
//     .multi_cartesian_product()
//     .map(|(z1, z2)|
//          find_best(d, n - 2, vec![z1, z2], open1.clone()))
//             .max()
//             .unwrap()

//     let walk = v
//         .next
//         .iter()
//         .cloned()
//         .map(|z| find_best(d, n - 1, z, open.clone()))
//         .max()
//         .unwrap();

//     alt.map(|m1| walk.max((v.pressure * (n - 1)) + m1)).unwrap_or(walk)
// }

fn dists(data: &Data) -> i32 {
    let n = 30;
    find_best(data, n, String::from("AA"), BTreeSet::new())
}

pub fn day16(input: &str) -> Result<()> {
    let data = input.parse::<Data>()?;
    println!("{:?}", data);
    println!("{}", dists(&data));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it() {
        let i = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert!(day16(i).is_ok());
    }
}
