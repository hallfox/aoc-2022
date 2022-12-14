use anyhow::Result;

use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter,
    path::PathBuf,
};

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
    let solt2 = szs.values().filter(|&x| x + free >= 30000000).min().unwrap();
    println!("{}", solt2);

    Ok(())
}
