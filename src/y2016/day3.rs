use std::str::FromStr;

use itertools::Itertools;

use crate::{common::ParseError, read_input};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Triangle(usize, usize, usize);
impl Triangle {
    fn is_valid(&self) -> bool {
        self.0 + self.1 > self.2 && self.0 + self.2 > self.1 && self.1 + self.2 > self.0
    }
}

impl FromStr for Triangle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<_>>();
        if split.len() != 3 {
            return Err(ParseError::new(s, "Input did not have 3 words"));
        }
        let s1 = split[0].parse()?;
        let s2 = split[1].parse()?;
        let s3 = split[2].parse()?;
        Ok(Triangle(s1, s2, s3))
    }
}

pub fn p1() -> usize {
    read_input!()
        .lines()
        .map(|s| s.trim().parse::<Triangle>().unwrap())
        .filter(|t| t.is_valid())
        .count()
}

pub fn p2() -> usize {
    let mut result = 0;
    for c in read_input!().lines().chunks(3).into_iter() {
        let vec = c.collect_vec();
        let joined = format!("{} {} {}", vec[0], vec[1], vec[2]);
        let split = joined.split_whitespace().collect_vec();
        for i in [0, 1, 2] {
            if Triangle::from_str(&format!("{} {} {}", split[i], split[i + 3], split[i + 6]))
                .unwrap()
                .is_valid()
            {
                result += 1;
            }
        }
    }
    result
}
