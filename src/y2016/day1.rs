use std::collections::HashSet;

use crate::common::Orientation;
use crate::common::Orientation::*;
use crate::read_input;
use num::abs;
use std::file;

fn rotate(d: Orientation, c: char) -> Orientation {
    match (c, d) {
        ('R', North) => East,
        ('R', South) => West,
        ('R', West) => North,
        ('R', East) => South,
        ('L', North) => West,
        ('L', South) => East,
        ('L', West) => South,
        ('L', East) => North,
        _ => panic!(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    x: i32,
    y: i32,
    direction: Orientation,
}
impl State {
    pub fn next(&self, n: &str) -> Self {
        let amount = n[1..].parse::<i32>().unwrap();
        let direction = rotate(self.direction, n.chars().next().unwrap());
        let (x, y) = match direction {
            North => (self.x, self.y + amount),
            South => (self.x, self.y - amount),
            West => (self.x - amount, self.y),
            East => (self.x + amount, self.y),
        };
        Self { x, y, direction }
    }
    pub fn next_all(&self, n: &str) -> Vec<(i32, i32)> {
        let amount = n[1..].parse::<i32>().unwrap();
        assert!(amount > 0);
        let direction = rotate(self.direction, n.chars().next().unwrap());
        let next_coordinate = |(x, y)| match direction {
            North => (x, y + 1),
            South => (x, y - 1),
            West => (x - 1, y),
            East => (x + 1, y),
        };

        let mut result = Vec::new();
        let mut current = (self.x, self.y);
        for _ in 0..amount {
            current = next_coordinate(current);
            result.push(current);
        }
        result
    }
    pub fn distance(&self) -> usize {
        (abs(self.x) + abs(self.y)) as usize
    }
}
impl Default for State {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: North,
        }
    }
}
pub fn p1() -> usize {
    read_input!()
        .split(", ")
        .fold(State::default(), |curr, s| curr.next(s))
        .distance()
}
pub fn p2() -> usize {
    let mut visited = HashSet::new();
    let mut current = State::default();
    visited.insert((current.x, current.y));
    for s in read_input!().split(", ") {
        for c in current.next_all(s) {
            if visited.contains(&c) {
                return (abs(c.0) + abs(c.1)) as usize;
            }
            visited.insert(c);
        }
        current = current.next(s);
    }
    panic!()
}
