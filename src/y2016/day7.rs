use std::str::FromStr;

use combine::{
    attempt, between,
    error::StringStreamError,
    many1,
    parser::char::{char, letter},
    Parser,
};
use itertools::Itertools;

use crate::parse_input_lines;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Code {
    sequences: Vec<String>,
    hypernets: Vec<String>,
}

fn contains_abba(s: &str) -> bool {
    for (c1, c2, c3, c4) in s.chars().tuple_windows() {
        if c1 == c4 && c2 == c3 && c1 != c2 {
            return true;
        }
    }
    false
}
fn abas(s: &str) -> impl Iterator<Item = (char, char)> + '_ {
    s.chars()
        .tuple_windows()
        .filter(|(c1, c2, c3)| c1 == c3 && c1 != c2)
        .map(|e| (e.0, e.1))
}
impl Code {
    fn supports_tsl(&self) -> bool {
        self.sequences.iter().any(|s| contains_abba(s))
            && !self.hypernets.iter().any(|s| contains_abba(s))
    }
    fn supports_ssl(&self) -> bool {
        let hypernet_abas = self.hypernets.iter().flat_map(|s| abas(s)).collect_vec();
        self.sequences
            .iter()
            .flat_map(|s| abas(s))
            .any(|(c1, c2)| hypernet_abas.iter().any(|(c3, c4)| c1 == *c4 && c2 == *c3))
    }
}

// abba[mnop]qrst
impl FromStr for Code {
    type Err = StringStreamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let word = || many1(letter()).map(|v: String| v);
        let hypernet = between(char('['), char(']'), word());
        let mut parser = many1(attempt(word().and(hypernet)))
            .map(|s: Vec<(String, String)>| s)
            .and(word());
        let res = parser.parse(s).map(|e| e.0)?;
        let (mut sequences, hypernets): (Vec<_>, Vec<_>) = res.0.iter().cloned().unzip();
        sequences.push(res.1);
        Ok(Code {
            sequences,
            hypernets,
        })
    }
}

pub fn p1() -> usize {
    parse_input_lines!(Code)
        .filter(|c| c.supports_tsl())
        .count()
}

pub fn p2() -> usize {
    parse_input_lines!(Code)
        .filter(|c| c.supports_ssl())
        .count()
}
