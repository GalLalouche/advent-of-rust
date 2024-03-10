use std::borrow::Borrow;

use Digit::*;
use DigitExtra::*;

use crate::{
    common::{Direction, Move},
    read_input,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Digit {
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
}
fn move_all<A: Move + Copy>(a: &A, s: &str) -> A {
    s.chars()
        .fold(*a, |d, c| d.mv(Direction::try_from(c).unwrap()))
}
impl Move for Digit {
    fn mv(&self, d: Direction) -> Self {
        match d {
            Direction::Up => match self {
                D1 | D2 | D3 => *self,
                d => (usize::from(d) - 3).borrow().try_into().unwrap(),
            },
            Direction::Down => match self {
                D7 | D8 | D9 => *self,
                d => (usize::from(d) as usize + 3).borrow().try_into().unwrap(),
            },
            Direction::Left => match self {
                D1 | D4 | D7 => *self,
                d => (usize::from(d) as usize - 1).borrow().try_into().unwrap(),
            },
            Direction::Right => match self {
                D3 | D6 | D9 => *self,
                d => (usize::from(d) as usize + 1).borrow().try_into().unwrap(),
            },
        }
    }
}
impl From<&Digit> for usize {
    fn from(d: &Digit) -> usize {
        match d {
            D1 => 1,
            D2 => 2,
            D3 => 3,
            D4 => 4,
            D5 => 5,
            D6 => 6,
            D7 => 7,
            D8 => 8,
            D9 => 9,
        }
    }
}
impl TryFrom<&usize> for Digit {
    type Error = ();

    fn try_from(d: &usize) -> Result<Digit, Self::Error> {
        match d {
            1 => Ok(D1),
            2 => Ok(D2),
            3 => Ok(D3),
            4 => Ok(D4),
            5 => Ok(D5),
            6 => Ok(D6),
            7 => Ok(D7),
            8 => Ok(D8),
            9 => Ok(D9),
            _ => Err(()),
        }
    }
}

impl From<Digit> for char {
    fn from(value: Digit) -> Self {
        match value {
            D1 => '1',
            D2 => '2',
            D3 => '3',
            D4 => '4',
            D5 => '5',
            D6 => '6',
            D7 => '7',
            D8 => '8',
            D9 => '9',
        }
    }
}

fn solution<A: Move + Copy + Into<char>>(init: A) -> String {
    let mut current = init;
    let mut vec: Vec<char> = Vec::new();
    for l in read_input!().lines() {
        current = move_all(&current, l);
        vec.push(current.into());
    }
    vec.iter().collect()
}

pub fn p1() -> String {
    solution(D5)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DigitExtra {
    DE1,
    DE2,
    DE3,
    DE4,
    DE5,
    DE6,
    DE7,
    DE8,
    DE9,
    A,
    B,
    C,
    D,
}

//     1
//   2 3 4
// 5 6 7 8 9
//   A B C
//     D
impl Move for DigitExtra {
    fn mv(&self, d: Direction) -> Self {
        match d {
            Direction::Up => match self {
                DE1 | DE2 | DE5 | DE4 | DE9 => *self,
                DE3 => DE1,
                DE6 => DE2,
                DE7 => DE3,
                DE8 => DE4,
                A => DE6,
                B => DE7,
                C => DE8,
                D => B,
            },
            Direction::Down => match self {
                A | D | C | DE5 | DE9 => *self,
                DE1 => DE3,
                DE2 => DE6,
                DE3 => DE7,
                DE4 => DE8,
                DE6 => A,
                DE7 => B,
                DE8 => C,
                B => D,
            },
            Direction::Right => match self {
                DE1 | DE4 | DE9 | C | D => *self,
                DE2 => DE3,
                DE3 => DE4,
                DE5 => DE6,
                DE6 => DE7,
                DE7 => DE8,
                DE8 => DE9,
                A => B,
                B => C,
            },
            Direction::Left => match self {
                DE1 | DE2 | DE5 | A | D => *self,
                DE3 => DE2,
                DE4 => DE3,
                DE6 => DE5,
                DE7 => DE6,
                DE8 => DE7,
                DE9 => DE8,
                B => A,
                C => B,
            },
        }
    }
}
impl From<DigitExtra> for char {
    fn from(value: DigitExtra) -> Self {
        match value {
            DE1 => '1',
            DE2 => '2',
            DE3 => '3',
            DE4 => '4',
            DE5 => '5',
            DE6 => '6',
            DE7 => '7',
            DE8 => '8',
            DE9 => '9',
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
        }
    }
}

pub fn p2() -> String {
    solution(DE5)
}
