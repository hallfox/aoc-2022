use anyhow::Result;


use std::{
    collections::{HashSet},
};

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