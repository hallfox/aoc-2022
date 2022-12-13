use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

use anyhow::Result;
use itertools::Itertools;

fn ord(x: char) -> u32 {
    x.into()
}

#[derive(Clone)]
struct Weight {
    pos: (usize, usize),
    map: Rc<RefCell<Vec<Vec<u32>>>>,
}

impl Weight {
    fn get(&self) -> u32 {
        let (r, c) = self.pos;
        self.map.borrow()[r][c]
    }
}

impl PartialOrd for Weight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }
}

impl PartialEq for Weight {
    fn eq(&self, other: &Self) -> bool {
        self.get().eq(&other.get())
    }
}

impl Eq for Weight {}

impl Ord for Weight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(&other.get())
    }
}

// fn garbage()
//     let mut visited = HashSet::new();
//     let mut weights = vec![vec![None; c_l as usize]; r_l as usize];
//     let mut stack = BinaryHeap::new();
//     let mut steps = 0;
//     weights.insert(start, RefCell::new(Weight { pos: start, w: 0 }));
//     stack.push(weights.get_mut(&start).unwrap());
//     while let Some(w) = stack.pop() {
//         let p @ (r, c) = w.borrow().pos;
//         let z = map[r as usize][c as usize];
//         visited.insert(p);
//         let neighs = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];
//         let to_visit = neighs.iter().filter_map(|(r, c)| {
//             if *r >= 0 && *r < r_l && *c >= 0 && *c < c_l {
//                 let z1 = map[*r as usize][*c as usize];
//                 if z1 <= z || (z < z1 && z1 - z <= 1) {
//                     let Weight { w, .. } = *weights[&p].borrow();
//                     let p1 = (*r, *c);
//                     let w1 = weights.entry(p1).or_insert(RefCell::new(Weight {
//                         pos: p1,
//                         w: w + 1,
//                     }));
//                     let new_w = (w + 1).min(w1.borrow().w);
//                     w1.borrow_mut().w = new_w;
//                     if !visited.contains(&p1) {
//                         return Some(*w1);
//                     }
//                 }
//             }
//             None
//         });
//         for v in to_visit {
//             stack.push(v);
//         }
//     }
//     weights[&end].borrow().w

fn walk(map: &Vec<Vec<u32>>, start: &(usize, usize), end: &(usize, usize)) -> u32 {
    let r_l = map.len();
    let c_l = map[0].len();

    let mut visited = HashSet::new();
    let weights = Rc::new(RefCell::new(vec![vec![u32::MAX; c_l]; r_l]));
    let mut stack = VecDeque::new();
    weights.borrow_mut()[start.0][start.1] = 0;
    stack.push_back(Weight {
        pos: *start,
        map: Rc::clone(&weights),
    });
    visited.insert(*start);
    while let Some(w) = stack.pop_front() {
        let p @ (r, c) = w.pos;
        let z = map[r][c];
        let r = r as i32;
        let c = c as i32;
        let neighs = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];
        neighs.iter().for_each(|(r, c)| {
            if *r >= 0 && *r < r_l as i32 && *c >= 0 && *c < c_l as i32 {
                let z1 = map[*r as usize][*c as usize];
                if z1 <= z || z1 - z <= 1 {
                    let w = weights.borrow_mut()[p.0][p.1];
                    let p1 = (*r as usize, *c as usize);
                    let w1 = weights.borrow_mut()[p1.0][p1.1];
                    weights.borrow_mut()[p1.0][p1.1] = (w + 1).min(w1);
                    if !visited.contains(&p1) {
                        visited.insert(p1);
                        stack.push_back(Weight {
                            pos: p1,
                            map: Rc::clone(&weights),
                        });
                    }
                }
            }
        });
        stack.make_contiguous().sort();
    }
    let ans = weights.borrow();
    ans[end.0][end.1]
}

pub fn day12(input: &str) -> Result<()> {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let data = input
        .lines()
        .enumerate()
        .map(|(r, l)| {
            l.chars()
                .enumerate()
                .map(|(c, z)| match z {
                    'S' => {
                        start = (r, c);
                        1
                    }
                    'E' => {
                        end = (r, c);
                        26
                    }
                    x => ord(x) - ord('a') + 1,
                })
                .collect_vec()
        })
        .collect_vec();

    println!("{}", walk(&data, &start, &end));

    let solt2 = data
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, z)| if *z == 1 { Some((r, c)) } else { None })
        })
        .map(|start| walk(&data, &start, &end))
        .min();
    println!("{}", solt2.unwrap());

    Ok(())
}

#[test]
fn test_it() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    day12(input);
}
