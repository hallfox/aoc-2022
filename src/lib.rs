use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alpha1, char, line_ending, satisfy},
        is_alphabetic,
    },
    combinator::{map_res, value},
    sequence::delimited,
    IResult,
};
use regex::Regex;
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    iter,
    path::{Path, PathBuf},
};
use std::{ops::Range, str::Lines};

pub fn day1(input: &str) -> Result<()> {
    let mut calories: Vec<i32> = input
        .split("\r\n\r\n")
        .map(|elf| elf.lines().map(|food| food.parse::<i32>().unwrap()).sum())
        .collect();

    // Part 1
    println!("{}", calories.iter().max().unwrap());

    // Part 2
    calories.sort();
    println!("{}", calories.iter().rev().take(3).sum::<i32>());
    Ok(())
}

pub fn day2(input: &str) -> Result<()> {
    let game = [
        [1 + 3, 2 + 6, 3 + 0], // A
        [1 + 0, 2 + 3, 3 + 6], // B
        [1 + 6, 2 + 0, 3 + 3], // C
    ];
    let plays: Vec<_> = input
        .lines()
        .map(|strat| {
            strat
                .split_whitespace()
                .map(|play| match play {
                    "A" | "X" => 0,
                    "B" | "Y" => 1,
                    "C" | "Z" => 2,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let score = plays.iter().map(|play| game[play[0]][play[1]]).sum::<i32>();
    println!("{}", score);

    let strats = [
        [3, 4, 8], // A
        [1, 5, 9], // B
        [2, 6, 7], // C
    ];

    let score2 = plays
        .iter()
        .map(|play| strats[play[0]][play[1]])
        .sum::<i32>();

    println!("{}", score2);
    Ok(())
}

fn ord(x: char) -> u32 {
    x.into()
}

pub fn day3(input: &str) -> Result<()> {
    let xs = input
        .lines()
        .map(|bag| {
            let n = bag.len() / 2;
            let s1 = &bag[0..n];
            let s2 = &bag[n..];
            let b1: HashSet<_> = s1.chars().collect();
            let b2: HashSet<_> = s2.chars().collect();
            *b1.intersection(&b2)
                .next()
                .expect(&format!("Missing common string {:?} {:?}", s1, s2))
        })
        .collect::<Vec<_>>();

    fn prioritize(x: char) -> u32 {
        if x.is_lowercase() {
            ord(x) - ord('a') + 1
        } else {
            ord(x) - ord('A') + 27
        }
    }

    let solt: u32 = xs.iter().map(|&x| prioritize(x)).sum();

    println!("{}", solt);

    let ys: Vec<HashSet<_>> = input.lines().map(|bag| bag.chars().collect()).collect();
    let solt2: u32 = ys
        .chunks(3)
        .map(|arrs| {
            arrs[0]
                .iter()
                .filter(|&x| arrs[1].contains(x))
                .find(|&x| arrs[2].contains(x))
                .expect(&format!("Missing common string {:?}", arrs))
        })
        .map(|&x| prioritize(x))
        .sum();
    println!("{}", solt2);
    Ok(())
}

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

fn crate_parser(input: &str) -> IResult<&str, char> {
    alt((crate_value, empty_crate))(input)
}

fn empty_crate(input: &str) -> IResult<&str, char> {
    value(' ', tag("   "))(input)
}

fn crate_value(input: &str) -> IResult<&str, char> {
    delimited(char('['), satisfy(|c| c.is_alphabetic()), char(']'))(input)
}

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

pub fn day6(input: &str) -> Result<()> {
    fn find_first_uniq(xs: &[char], n: usize) -> Option<usize> {
        let (m, _) = xs
            .windows(n)
            .enumerate()
            .find(|(_, s)| s.iter().collect::<HashSet<_>>().len() == n)?;
        Some(m + n)
    }

    let xs = input.trim().chars().collect::<Vec<_>>();

    let solt1 = find_first_uniq(&xs, 4).unwrap();
    println!("{}", solt1);

    let solt2 = find_first_uniq(&xs, 14).unwrap();
    println!("{}", solt2);
    Ok(())
}

fn dir_size(tree: &HashMap<PathBuf, Vec<Vec<&str>>>, d: &PathBuf) -> usize {
    let cd = tree.get(d).unwrap();

    let sz: usize = cd.iter().filter_map(|xs| xs[0].parse::<usize>().ok()).sum();
    cd.iter()
        .filter_map(|xs| if xs[0] == "dir" { Some(xs[1]) } else { None })
        .map(|r| {
            let mut e = d.clone();
            e.push(r);
            dir_size(tree, &e)
        })
        .sum::<usize>()
        + sz
}

trait Unwrap {
    type T;
    fn u(self) -> Self::T;
}

impl<U> Unwrap for Option<U> {
    type T = U;
    fn u(self) -> Self::T {
        self.unwrap()
    }
}

impl<U, E: std::fmt::Debug> Unwrap for std::result::Result<U, E> {
    type T = U;
    fn u(self) -> Self::T {
        self.unwrap()
    }
}

pub fn day7(input: &str) -> Result<()> {
    let cd = Regex::new(r"\$ cd (.*)").unwrap();
    let mut cur_dir = PathBuf::new();
    cur_dir.push("/");
    let mut tree: HashMap<std::path::PathBuf, Vec<_>> = HashMap::new();

    for l in input.lines() {
        for cap in cd.captures_iter(l) {
            if cap[1].starts_with('/') {
                cur_dir = PathBuf::from(&cap[1]);
            } else if &cap[1] == ".." {
                cur_dir.pop();
            } else {
                cur_dir.push(&cap[1]);
            }
        }

        if !l.starts_with('$') {
            let xs = l.split_whitespace().collect::<Vec<_>>();
            tree.entry(cur_dir.clone()).or_default().push(xs);
        }
    }

    let mut szs = HashMap::new();
    let mut solt1 = 0;
    for d in tree.keys() {
        let sz = dir_size(&tree, d);
        szs.insert(d, sz);

        if sz < 100000 {
            solt1 += sz;
        }
    }
    println!("{}", solt1);

    let free = 70000000 - szs.get(&PathBuf::from("/")).unwrap();
    let solt2 = szs.values().filter(|&x| x + free >= 30000000).min().u();
    println!("{}", solt2);

    Ok(())
}

pub fn day8(input: &str) -> Result<()> {
    let ts: Vec<_> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let n = ts.len();
    let m = ts[0].len();
    // n x m
    let mut see = HashSet::new();
    for i in 0..m {
        let mut seen: i32 = -1;
        // Up
        for j in 0..n - 1 {
            if ts[j][i] > seen {
                see.insert((j, i));
                seen = ts[j][i];
            }
        }
        // Down
        seen = -1;
        for j in (1..n).rev() {
            if ts[j][i] > seen {
                see.insert((j, i));
                seen = ts[j][i];
            }
        }
    }
    for j in 0..n {
        let mut seen = -1;
        // Left
        for i in 0..m - 1 {
            if ts[j][i] > seen {
                see.insert((j, i));
                seen = ts[j][i];
            }
        }
        // Right
        seen = -1;
        for i in (1..m).rev() {
            if ts[j][i] > seen {
                see.insert((j, i));
                seen = ts[j][i];
            }
        }
    }

    println!("{}", see.len());

    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut scores = Vec::new();
    for (i, r) in ts.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            let mut score = 1;
            for d in &dirs {
                let mut p = (i as i32 + d.0, j as i32 + d.1);
                let mut x = 0;
                while p.0 >= 0 && p.0 < n as i32 && p.1 >= 0 && p.1 < m as i32 {
                    x += 1;
                    if ts[p.0 as usize][p.1 as usize] >= *c {
                        break;
                    }

                    p = (p.0 + d.0, p.1 + d.1);
                }
                score *= x;
            }
            scores.push(score);
        }
    }
    println!("{}", scores.iter().max().unwrap());

    Ok(())
}

fn paths(xs: &Vec<Vec<i32>>, p: &(usize, usize)) -> impl Iterator {
    let m = xs.len();
    let n = xs[0].len();
    let (i, j) = *p;
    (i + 1..m)
        .map(move |a| (a, j))
        .chain((0..i).rev().map(move |a| (a, j)))
        .chain((j + 1..n).map(move |a| (i, a)))
        .chain((0..j).rev().map(move |a| (i, a)))
}

type Pair<T> = (T, T);
fn t_add(xs: Pair<i32>, ys: Pair<i32>) -> Pair<i32> {
    (xs.0 + ys.0, xs.1 + ys.1)
}
fn t_sub(xs: Pair<i32>, ys: Pair<i32>) -> Pair<i32> {
    (xs.0 - ys.0, xs.1 - ys.1)
}
fn norm(x: i32) -> i32 {
    if x != 0 {
        x / x.abs()
    } else {
        x
    }
}

pub fn day9(input: &str) -> Result<()> {
    let cmds: Vec<_> = input
        .lines()
        .map(|x| {
            let (d, n) = x.split_whitespace().collect_tuple().unwrap();
            (d, n.parse::<usize>().unwrap())
        })
        .collect();

    let mut h_pos = (0, 0);
    let mut t_pos = (0, 0);
    let mut seen = HashSet::new();
    seen.insert(t_pos);
    for (d, n) in &cmds {
        let m = match *d {
            "U" => (0, 1),
            "R" => (1, 0),
            "D" => (0, -1),
            "L" => (-1, 0),
            _ => unreachable!(),
        };
        for _ in 0..*n {
            let old_pos = h_pos;
            h_pos = t_add(h_pos, m);
            if h_pos.0.abs_diff(t_pos.0) > 1 || h_pos.1.abs_diff(t_pos.1) > 1 {
                t_pos = old_pos;
                seen.insert(t_pos);
            }
        }
    }
    println!("{}", seen.len());

    let mut ts = vec![(0, 0); 10];
    let mut seen = HashSet::new();
    seen.insert((0, 0));
    for (d, n) in &cmds {
        let m = match *d {
            "U" => (0, 1),
            "R" => (1, 0),
            "D" => (0, -1),
            "L" => (-1, 0),
            _ => unreachable!(),
        };
        for _ in 0..*n {
            let l = ts.len();
            let mut h = 0;
            let mut c = m;
            let mut old_pos = ts[0];
            ts[0] = t_add(ts[0], c);
            for t in 1..l {
                let v = ts[t];
                if ts[h].0.abs_diff(ts[t].0) > 1 && ts[h].1.abs_diff(ts[t].1) > 1 {
                    ts[t].0 = ts[h].0 - (ts[h].0 - ts[t].0).signum();
                    ts[t].1 = ts[h].1 - (ts[h].1 - ts[t].1).signum();
                }
                if ts[h].0.abs_diff(ts[t].0) > 1 {
                    ts[t].0 = ts[h].0 - (ts[h].0 - ts[t].0).signum();
                    ts[t].1 = ts[h].1;
                } else if ts[h].1.abs_diff(ts[t].1) > 1 {
                    ts[t].0 = ts[h].0;
                    ts[t].1 = ts[h].1 - (ts[h].1 - ts[t].1).signum();
                }
                old_pos = v;
                h = t;
            }
            seen.insert(*ts.last().unwrap());
        }
        println!("{:?}", ts);
    }
    println!("{}", seen.len());
    Ok(())
}

enum Op {
    Noop,
    Addx(i32),
}

struct Program(Vec<Op>);

impl Program {
    fn run(&self) -> Vec<i32> {
        let mut x = 1;
        let mut n = 1;
        let mut states = vec![x];
        for i in &self.0 {
            match *i {
                Op::Noop => {
                    states.push(x);
                }
                Op::Addx(a) => {
                    states.push(x);
                    x += a;
                    states.push(x);
                    n += 2;
                }
            }
        }
        states
    }

    fn render(&self) -> Vec<char> {
        let mut x: i32 = 1;
        let mut n = 0;
        let mut states = iter::repeat('.').take(40 * 6).collect::<Vec<_>>();
        let mut draw = |z, x: i32| {
            let m = z % 40;
            if (x % 40).abs_diff(m) < 2 {
                states[z as usize] = '#';
            }
        };
        for i in &self.0 {
            match *i {
                Op::Noop => {
                    draw(n, x);
                    n += 1;
                }
                Op::Addx(a) => {
                    draw(n, x);
                    n += 1;
                    draw(n, x);
                    x += a;
                    n += 1;
                }
            }
        }
        states
    }
}

pub fn day10(input: &str) -> Result<()> {
    let prog = Program(
        input
            .lines()
            .map(|l| {
                if l == "noop" {
                    Op::Noop
                } else {
                    let (_, i) = l.split_whitespace().collect_tuple().unwrap();
                    Op::Addx(i.parse::<i32>().unwrap())
                }
            })
            .collect_vec(),
    );

    let states = prog.run();
    let part1: i32 = [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|&i| states[i - 1] * (i as i32))
        .sum();
    println!("{}", part1);
    let part2 = prog.render();
    part2
        .as_slice()
        .chunks(40)
        .for_each(|l| println!("{}", l.iter().collect::<String>()));

    Ok(())
}

#[test]
fn test_it() {
    let i = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    day9(i);
}
