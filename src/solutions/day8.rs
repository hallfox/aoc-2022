use anyhow::Result;

use std::collections::HashSet;

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
