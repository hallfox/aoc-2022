use std::{
    str::FromStr,
};

use anyhow::Result;
use cached::proc_macro::cached;
use itertools::Itertools;


use rayon::prelude::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blueprint {
    ore: i32,
    clay: i32,
    obsid: (i32, i32),
    geode: (i32, i32),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Data {
    d: Vec<Blueprint>,
}

type Pos = (isize, isize);
type Coord = (isize, isize, isize);

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let data = s
            .lines()
            .map(|l| {
                let rs = l.split(':').nth(1).unwrap();
                let (ore, clay, obsid, geode, _) = rs
                    .split('.')
                    .map(|r| {
                        r.chars()
                            .filter(|c| "0123456789 ".contains(*c))
                            .collect::<String>()
                    })
                    .collect_tuple()
                    .unwrap();
                Blueprint {
                    ore: ore.trim().parse::<i32>().unwrap(),
                    clay: clay.trim().parse::<i32>().unwrap(),
                    obsid: obsid
                        .split_whitespace()
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                    geode: geode
                        .split_whitespace()
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                }
            })
            .collect_vec();
        Ok(Data { d: data })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Mine {
    rs: (i32, i32, i32, i32),
    ss: (i32, i32, i32, i32),
}

#[cached]
fn run_mine(blu: Blueprint, m: Mine, n: i32) -> i32 {
    let (or, cr, br, gr) = m.rs;
    let (mut o, mut c, mut b, g) = m.ss;

    if n <= 0 {
        return m.ss.3;
    }

    let mo = *[blu.ore, blu.clay, blu.obsid.0, blu.geode.0]
        .iter()
        .max()
        .unwrap();
    o = o.min((n * mo) - ((n - 1) * or));
    c = c.min((n * blu.obsid.1) - ((n - 1) * cr));
    b = b.min((n * blu.geode.1) - ((n - 1) * br));

    let grn = (blu.geode.0 <= o && blu.geode.1 <= b).then(|| {
        let o = o + or;
        let c = c + cr;
        let b = b + br;
        let g = g + gr;
        run_mine(
            blu.clone(),
            Mine {
                rs: (or, cr, br, gr + 1),
                ss: (o - blu.geode.0, c, b - blu.geode.1, g),
            },
            n - 1,
        )
    });
    let brn = (blu.obsid.0 <= o && blu.obsid.1 <= c).then(|| {
        let o = o + or;
        let c = c + cr;
        let b = b + br;
        let g = g + gr;
        run_mine(
            blu.clone(),
            Mine {
                rs: (or, cr, br + 1, gr),
                ss: (o - blu.obsid.0, c - blu.obsid.1, b, g),
            },
            n - 1,
        )
    });
    let crn = (blu.clay <= o).then(|| {
        let o = o + or;
        let c = c + cr;
        let b = b + br;
        let g = g + gr;
        run_mine(
            blu.clone(),
            Mine {
                rs: (or, cr + 1, br, gr),
                ss: (o - blu.clay, c, b, g),
            },
            n - 1,
        )
    });
    let orn = (blu.ore <= o).then(|| {
        let o = o + or;
        let c = c + cr;
        let b = b + br;
        let g = g + gr;
        run_mine(
            blu.clone(),
            Mine {
                rs: (or + 1, cr, br, gr),
                ss: (o - blu.ore, c, b, g),
            },
            n - 1,
        )
    });

    let o = o + or;
    let c = c + cr;
    let b = b + br;
    let g = g + gr;
    let xn = run_mine(
        blu,
        Mine {
            ss: (o, c, b, g),
            ..m
        },
        n - 1,
    );

    [Some(xn), orn, crn, brn, grn]
        .iter()
        .filter_map(|mn| *mn)
        .max()
        .unwrap()
}

impl Blueprint {
    fn geodes(&self, t: i32) -> i32 {
        let m = Mine {
            rs: (1, 0, 0, 0),
            ss: (0, 0, 0, 0),
        };

        run_mine(self.clone(), m, t)
    }
}

fn quality(d: &Data) -> i32 {
    d.d.iter()
        .enumerate()
        .map(|(i, b)| b.geodes(24) * (i as i32 + 1))
        .sum()
}

fn best_three(d: &Data) -> i32 {
    d.d.iter()
        .take(3)
        .enumerate()
        .map(|(_i, b)| b.geodes(32))
        .product()
}

pub fn day19(input: &str) -> Result<()> {
    let data = input.parse::<Data>()?;
    println!("{}", quality(&data));
    println!("{}", best_three(&data));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it() {
        let i = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert!(day19(i).is_ok());
    }
}
