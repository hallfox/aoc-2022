use anyhow::Result;

use itertools::Itertools;
use std::{
    collections::{HashSet},
};

type Pair<T> = (T, T);
fn t_add(xs: Pair<i32>, ys: Pair<i32>) -> Pair<i32> {
    (xs.0 + ys.0, xs.1 + ys.1)
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
            let c = m;
            ts[0] = t_add(ts[0], c);
            for t in 1..l {
                let _v = ts[t];
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
                h = t;
            }
            seen.insert(*ts.last().unwrap());
        }
        println!("{:?}", ts);
    }
    println!("{}", seen.len());
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