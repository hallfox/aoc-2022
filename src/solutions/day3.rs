use std::collections::HashSet;

use anyhow::Result;
use crate::util::ord;

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
                .unwrap_or_else(|| panic!("Missing common string {:?} {:?}", s1, s2))
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
                .unwrap_or_else(|| panic!("Missing common string {:?}", arrs))
        })
        .map(|&x| prioritize(x))
        .sum();
    println!("{}", solt2);
    Ok(())
}
