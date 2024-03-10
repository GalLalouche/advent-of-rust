use std::str::FromStr;

use combine::error::StringStreamError;

use combine::parser::repeat::take_until;

use combine::parser::Parser;

use combine::{attempt, between, sep_end_by1};
use combine::{
    many1,
    parser::char::{char, digit, letter},
};
use itertools::Itertools;

use crate::read_input;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Room {
    name: String,
    id: usize,
    checksum: String,
}

impl Room {
    fn is_valid(&self) -> bool {
        let actual: String = self
            .name
            .chars()
            .into_group_map_by(|&c| c)
            .into_iter()
            .map(|(c, v)| (c, v.len()))
            .sorted_by_key(|(c, len)| (-(*len as i32), *c))
            .map(|e| e.0)
            .take(5)
            .collect();
        actual == self.checksum
    }

    fn decipher(&self) -> String {
        let amount = (self.id % 26) as u8;
        let for_char = |c: char| {
            let ord = c as u8 - b'a';
            ((ord + amount) % 26 + b'a') as char
        };
        self.name.replace('-', " ").chars().map(for_char).collect()
    }
}

impl FromStr for Room {
    type Err = StringStreamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dash = char('-');
        let num = many1(digit())
            .map(|c: Vec<char>| c.iter().collect::<String>().parse::<usize>().unwrap());
        let mut split = take_until(digit()).map(|s: String| s);
        let (s1, s2) = split.parse(s)?;
        let word = many1(letter());
        let mut words = sep_end_by1(word, attempt(dash)).map(|e: Vec<String>| e.join(""));
        let (name, _) = words.parse(s1.as_ref() as &str)?;
        let mut nums = num
            .and(between(char('['), char(']'), many1(letter())))
            .map(|e: (usize, String)| e);
        let ((id, checksum), _) = nums.parse(s2)?;
        Ok(Self { name, id, checksum })
    }
}

pub fn p1() -> usize {
    read_input!()
        .lines()
        .map(|s| Room::from_str(s.trim()).unwrap())
        .filter(|r| r.is_valid())
        .map(|r| r.id)
        .sum()
}

pub fn p2() -> usize {
    read_input!()
        .lines()
        .map(|s| Room::from_str(s.trim()).unwrap())
        .find(|s| s.decipher().contains("northpole"))
        .unwrap()
        .id
}
