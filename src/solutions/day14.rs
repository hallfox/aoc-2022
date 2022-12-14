
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

#[derive(Debug, Clone)]
struct Data;


impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Data { })
    }
}

pub fn day14(input: &str) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_it() {

    }
}
