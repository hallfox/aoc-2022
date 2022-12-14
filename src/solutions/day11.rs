use anyhow::Result;

use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter,
    path::{PathBuf},
};

#[derive(Debug, Clone)]
enum Oper {
    Add(u32),
    Mul(u32),
    Double,
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u128>,
    oper: Oper,
    test: u32,
    yes: u32,
    no: u32,
}

fn big_mul(a: u128, b: u128, z: u128) -> u128 {
    let a = a % z;
    iter::successors(Some(a), |a| Some((a * 2) % z))
        .zip((0..).map_while(|p| {
            let b = b >> p;
            if b == 0 {
                None
            } else {
                Some(b)
            }
        }))
        .fold(0, |r, (a, b)| (r + ((b & 1) * a)) % z)
}

trait MonkeyBusiness {
    fn monkey_business<F>(self, rounds: usize, worry_fn: F) -> u128
    where
        F: Fn(u128, &Oper) -> u128;
}

impl MonkeyBusiness for Vec<Monkey> {
    fn monkey_business<F>(mut self, rounds: usize, worry_fn: F) -> u128
    where
        F: Fn(u128, &Oper) -> u128,
    {
        let mut inspect = vec![0; self.len()];
        for _ in 0..rounds {
            for i in 0..self.len() {
                while !self[i].items.is_empty() {
                    let worry = worry_fn(self[i].items.pop_front().unwrap(), &self[i].oper);
                    let n = if worry % self[i].test as u128 == 0 {
                        self[i].yes
                    } else {
                        self[i].no
                    };
                    self[n as usize].items.push_back(worry);
                    inspect[i] += 1;
                }
            }
        }
        inspect.sort();
        inspect.reverse();
        inspect[0] * inspect[1]
    }
}

fn monkey_business_1(ms: Vec<Monkey>) -> u128 {
    ms.monkey_business(20, |worry, oper| {
        let worry = match oper {
            Oper::Add(x) => worry + (*x as u128),
            Oper::Mul(x) => worry * (*x as u128),
            Oper::Double => worry * 2,
            Oper::Square => worry * worry,
        };
        worry / 3
    })
}

fn monkey_business_2(ms: Vec<Monkey>) -> u128 {
    let z: u128 = ms.iter().map(|m| m.test as u128).product();
    ms.monkey_business(10000, |worry, oper| match oper {
        Oper::Add(x) => (worry + (*x as u128)) % z,
        Oper::Mul(x) => big_mul(worry, *x as u128, z),
        Oper::Double => big_mul(worry, 2, z),
        Oper::Square => big_mul(worry, worry, z),
    })
}

pub fn day11(input: &str) -> Result<()> {
    let data = input
        .split("\r\n\r\n")
        .map(|l| {
            let (_, items, op, test, tru, fals) = l.lines().collect_tuple().unwrap();
            Monkey {
                items: items[18..]
                    .split(", ")
                    .map(|x| x.parse::<u32>().unwrap().into())
                    .collect::<VecDeque<_>>(),
                oper: match &op[19..].split_whitespace().collect_tuple().unwrap() {
                    ("old", x, "old") => {
                        if *x == "*" {
                            Oper::Square
                        } else {
                            Oper::Double
                        }
                    }
                    (_, "*", x) => Oper::Mul(x.parse::<u32>().unwrap()),
                    (_, "+", x) => Oper::Add(x.parse::<u32>().unwrap()),
                    _ => unreachable!(),
                },
                test: test[21..].parse::<u32>().unwrap(),
                yes: tru[29..].parse::<u32>().unwrap(),
                no: fals[30..].parse::<u32>().unwrap(),
            }
        })
        .collect_vec();

    println!("{}", monkey_business_1(data.clone()));
    println!("{}", monkey_business_2(data));
    Ok(())
}

#[test]
fn test_big_mul() {
    assert_eq!(119, big_mul(426, 964, 235));
}