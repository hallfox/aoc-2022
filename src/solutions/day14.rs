use std::{cmp::Ordering, str::FromStr};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i32,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, terminated},
    Finish, IResult,
};

type Point = (usize, usize);

#[derive(Debug, Clone)]
struct Data {
    rocks: Vec<Vec<Point>>,
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let rocks = s
            .lines()
            .map(|l| {
                l.split(" -> ")
                    .map(|p| {
                        p.split(",")
                            .map(|a| a.parse::<usize>().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect_vec()
            })
            .collect_vec();
        Ok(Data { rocks })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Rock,
    Air,
    Sand,
}

#[derive(Debug, Clone)]
struct Grid {
    g: Vec<Vec<Item>>,
    bounds: (usize, usize),
    width: usize,
}

impl Data {
    fn draw_grid(&self) -> Grid {
        let bounds = {
            let mut bounds = (500, 0);
            for r in &self.rocks {
                for (x, y) in r {
                    bounds = (bounds.0.max(*x), bounds.1.max(*y));
                }
            }
            (bounds.0 as usize + 1, bounds.1 as usize + 1)
        };

        let mut grid = vec![vec![Item::Air; bounds.0]; bounds.1];
        self.rocks.iter().for_each(|l| {
            l.iter().tuple_windows().for_each(|(a, b)| {
                if a.0 == b.0 {
                    let low = a.1.min(b.1);
                    let high = a.1.max(b.1);
                    for y in low..=high {
                        grid[y][a.0] = Item::Rock;
                    }
                } else {
                    let low = a.0.min(b.0);
                    let high = a.0.max(b.0);
                    for x in low..=high {
                        grid[a.1][x] = Item::Rock;
                    }
                };
            })
        });

        Grid {
            g: grid,
            bounds,
            width: 0,
        }
    }
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<Item> {
        if self.within(x, y) {
            Some(self.g[y as usize][(x + self.width as i32) as usize])
        } else {
            None
        }
    }

    fn get_p(&self, p: (i32, i32)) -> Option<Item> {
        self.get(p.0, p.1)
    }

    fn within(&self, x: i32, y: i32) -> bool {
        -(self.width as i32) <= x
            && x < (self.bounds.0 + self.width) as i32
            && y < self.bounds.1 as i32
    }
}

fn fill_sand(grid: Grid) -> usize {
    let dropper = (500, 0);
    (0..)
        .scan(grid, |grid, _| {
            let rest = (0..)
                .scan(Some(dropper), |curr, _| {
                    if curr.is_none() {
                        return None;
                    }
                    let c = curr.unwrap();
                    if c == dropper && grid.get_p(c) == Some(Item::Sand) {
                        return None;
                    }

                    match grid.get_p(c) {
                        None => {
                            *curr = None;
                            Some(c)
                        }
                        Some(Item::Air) => {
                            *curr = Some((c.0, c.1 + 1));
                            Some(c)
                        }
                        Some(Item::Rock | Item::Sand) => {
                            let left = (c.0 - 1, c.1);
                            let right = (c.0 + 1, c.1);
                            match grid.get_p(left) {
                                Some(Item::Air) | None => {
                                    *curr = Some(left);
                                    Some(left)
                                }
                                _ => match grid.get_p(right) {
                                    Some(Item::Air) | None => {
                                        *curr = Some(right);
                                        Some(right)
                                    }
                                    _ => {
                                        *curr = None;
                                        None
                                    }
                                },
                            }
                        }
                    }
                })
                .fuse()
                .last();
            if let Some(r) = rest {
                if grid.within(r.0, r.1) {
                    grid.g[r.1 as usize][r.0 as usize + grid.width] = Item::Sand;
                }
            }
            rest
        })
        .fuse()
        .count()
}

fn fill_floor(mut grid: Grid) -> usize {
    grid.width = 250;
    let space = vec![Item::Air; grid.width];
    for r in &mut grid.g {
        let mut rn = space.clone();
        rn.extend(r.iter());
        rn.extend(space.iter());
        *r = rn;
    }
    grid.g.push(vec![Item::Air; grid.bounds.0 + grid.width * 2]);
    grid.g
        .push(vec![Item::Rock; grid.bounds.0 + grid.width * 2]);
    grid.bounds = (grid.bounds.0, grid.bounds.1 + 2);
    fill_sand(grid)
}

pub fn day14(input: &str) -> Result<()> {
    let data = input.parse::<Data>()?;
    //println!("{:?}", data);
    let mut grid = data.draw_grid();
    //println!("{:?}", grid);
    //println!("{}", fill_sand(grid.clone()));
    println!("{}", fill_floor(grid));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it() {
        let i = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert!(day14(i).is_ok());
    }
}
