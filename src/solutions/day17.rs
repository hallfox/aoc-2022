use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i32,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, terminated},
    Finish, IResult,
};
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug, Clone, Hash)]
struct Data {
    dirs: Vec<Dir>,
}

#[derive(Debug, Clone, Hash)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Rock {
    Dash,
    T,
    L,
    I,
    O,
}

#[derive(Debug, Clone, Hash)]
struct Game {
    rocks: Vec<[char; 7]>,
    i: isize,
    n: usize,
    skipped: usize,
}

type Pos = (isize, isize);

#[derive(Debug, Clone, Hash)]
enum Collide {
    Wall,
    Rock,
}

impl Rock {
    fn body(&self) -> Vec<(isize, isize)> {
        match self {
            Rock::Dash => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::T => vec![(0, 0), (0, 1), (0, 2), (-1, 1), (1, 1)],
            Rock::L => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Rock::I => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Rock::O => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }
}

impl Game {
    fn height(&self) -> isize {
        self.rocks
            .iter()
            .enumerate()
            .find(|(_, r)| r.iter().all(|c| *c == '.'))
            .map_or(0, |(i, _)| i as isize)
    }

    fn draw(&mut self, r: &Rock, p: &Pos) {
        let b = r.body();
        for (x, y) in b {
            let pn = (p.0 + x, p.1 + y);
            self.rocks[pn.1 as usize][pn.0 as usize] = 'x';
        }
    }

    fn delete(&mut self, r: &Rock, p: &Pos) {
        let b = r.body();
        for (x, y) in b {
            let pn = (p.0 + x, p.1 + y);
            self.rocks[pn.1 as usize][pn.0 as usize] = '.';
        }
    }

    fn check_move(&self, r: &Rock, p: &Pos) -> Option<Collide> {
        // D: X@@@
        // T: .@.
        //    @@@
        //    .X.
        // L: ..@
        //    ..@
        //    X@@
        // I: @
        //    @
        //    @
        //    X
        // O: @@
        //    X@
        let spaces = r.body();
        for (x, y) in spaces {
            let pn = (p.0 + x, p.1 + y);
            if pn.0 < 0 || pn.0 >= 7 || pn.1 < 0 {
                return Some(Collide::Wall);
            } else if self.rocks[pn.1 as usize][pn.0 as usize] != '.' {
                return Some(Collide::Rock);
            }
        }
        None
    }
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let data = s
            .chars()
            .filter(|c| "<>".contains(*c))
            .map(|c| match c {
                '<' => Dir::Left,
                '>' => Dir::Right,
                _ => unreachable!(),
            })
            .collect_vec();
        Ok(Data { dirs: data })
    }
}

fn p2_add(p1: Pos, p2: Pos) -> Pos {
    (p1.0 + p2.0, p1.1 + p2.1)
}

impl Data {
    fn run_sim(&self, n: usize) -> Game {
        let g = Game {
            rocks: vec![['.'; 7]; 20],
            i: 0,
            n: 0,
            skipped: 0,
        };
        let mut wind = self.dirs.iter().cycle();
        let mut pats = HashMap::new();
        [Rock::Dash, Rock::T, Rock::L, Rock::I, Rock::O]
            .iter()
            .cycle()
            .fold_while(g, |mut g, r| {
                let h = g.height();
                let mut p = match r {
                    Rock::Dash => (2, h + 3),
                    Rock::T => (3, h + 3),
                    Rock::L => (2, h + 3),
                    Rock::I => (2, h + 3),
                    Rock::O => (2, h + 3),
                };
                if (p.1 + 4) as usize >= g.rocks.len() {
                    g.rocks.append(&mut vec![['.'; 7]; g.rocks.len()]);
                }

                loop {
                    g.i += 1;
                    let x = wind.next().unwrap();
                    let d = match *x {
                        Dir::Left => -1,
                        Dir::Right => 1,
                    };
                    if g.check_move(r, &(p.0 + d, p.1)).is_none() {
                        p = (p.0 + d, p.1);
                    }
                    match g.check_move(r, &(p.0, p.1 - 1)) {
                        Some(_) => {
                            g.draw(r, &p);
                            g.n += 1;
                            let hn = g.height();
                            let m = (
                                g.i as usize % self.dirs.len(),
                                *r,
                                g.rocks[(hn - 6).max(0) as usize..hn as usize].to_owned(),
                            );
                            if let Some(old) = pats.get(&m) {
                                let (old_n, old_h) = old;
                                let dh = (hn - *old_h) as usize;
                                let dn = g.n - *old_n;
                                let a = (n - g.n) / dn;
                                g.skipped += a * dh;
                                g.n += a * dn;
                            } else {
                                pats.insert(m, (g.n, hn));
                            }
                            if g.n >= n {
                                return Done(g);
                            }
                            break;
                        }
                        None => p = (p.0, p.1 - 1),
                    }
                }
                Continue(g)
            })
            .into_inner()
    }
}

pub fn day17(input: &str) -> Result<()> {
    let data = input.parse::<Data>()?;
    let a = data.run_sim(2022);
    let a2 = data.run_sim(1_000_000_000_000);
    println!("{}", a.height() + a.skipped as isize);
    println!("{}", a2.height() + a2.skipped as isize);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it() {
        let i = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert!(day17(i).is_ok());
    }
}
