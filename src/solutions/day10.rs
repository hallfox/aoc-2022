use anyhow::Result;

use itertools::Itertools;
use std::iter;

enum Op {
    Noop,
    Addx(i32),
}

struct Program(Vec<Op>);

impl Program {
    fn run(&self) -> Vec<i32> {
        let mut x = 1;
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
        .iter()
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
