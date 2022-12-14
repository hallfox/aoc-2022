use std::{cmp::Ordering, str::FromStr};

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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
struct Data {
    packets: Vec<(Packet, Packet)>,
}

fn num(input: &str) -> IResult<&str, Packet> {
    map(i32, Packet::Int)(input)
}

fn list(input: &str) -> IResult<&str, Packet> {
    delimited(
        tag("["),
        map(
            many0(terminated(alt((num, list)), opt(tag(",")))),
            Packet::List,
        ),
        tag("]"),
    )(input)
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let packets = s
            .split("\n\n")
            .map(|ps| {
                ps.lines()
                    .map(|l| list(l).finish().unwrap().1)
                    .collect_tuple()
                    .unwrap()
            })
            .collect_vec();

        Ok(Data { packets })
    }
}

impl Packet {
    fn vec_of_int(x: i32) -> Self {
        Packet::List(vec![Packet::Int(x)])
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Int(i), Packet::Int(j)) => i.partial_cmp(j),
            (Packet::List(_), Packet::Int(j)) => self.partial_cmp(&Packet::vec_of_int(*j)),
            (Packet::Int(i), Packet::List(_)) => Packet::vec_of_int(*i).partial_cmp(other),
            (Packet::List(l1), Packet::List(l2)) => l1
                .iter()
                .zip(l2.iter())
                .flat_map(|(p1, p2)| p1.partial_cmp(p2))
                .find(|o| *o != Ordering::Equal)
                .or_else(|| l1.len().partial_cmp(&l2.len())),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Data {
    fn check_order(&self) -> usize {
        self.packets
            .iter()
            .map(|(p1, p2)| p1.cmp(p2))
            .enumerate()
            .filter_map(|(i, o)| {
                if o == Ordering::Less {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .sum()
    }

    fn reorder(&self) -> usize {
        let divs = vec![
            Packet::List(vec![Packet::vec_of_int(2)]),
            Packet::List(vec![Packet::vec_of_int(6)]),
        ];
        let (p1, p2): (Vec<_>, Vec<_>) = self.packets.iter().cloned().unzip();
        let mut all_ps = divs.iter().chain(p1.iter()).chain(p2.iter()).collect_vec();

        all_ps.sort();
        divs.iter()
            .map(|p| all_ps.iter().enumerate().find(|t| *t.1 == p).unwrap().0 + 1)
            .product()
    }
}

pub fn day13(input: &str) -> Result<()> {
    let data = input.parse::<Data>()?;

    println!("{}", data.check_order());
    println!("{}", data.reorder());

    Ok(())
}

#[test]
fn test_it() {
    let i = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    day13(i);
}
