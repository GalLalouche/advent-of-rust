use std::str::FromStr;
use Instruction::*;

use combine::{
    attempt, choice,
    error::StringStreamError,
    many1,
    parser::char::{char, digit, string},
    Parser,
};
use itertools::Itertools;

use crate::parse_input_lines;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Rect { width: usize, height: usize },
    RotateRow { row: usize, by: usize },
    RotateColumn { col: usize, by: usize },
}

impl Instruction {
    fn transform(&self, board: &mut Vec<Vec<bool>>) {
        match self {
            Rect { width, height } => {
                for x in 0..*width {
                    for y in 0..*height {
                        board[y][x] = true
                    }
                }
            }
            RotateRow { row, by } => {
                let old_row = board[*row].clone();
                for i in 0..old_row.len() {
                    let new_index = (i + by) % old_row.len();
                    board[*row][new_index] = old_row[i];
                }
            }
            RotateColumn { col, by } => {
                let old_col = board.iter().map(|v| v[*col]).collect_vec();
                for i in 0..old_col.len() {
                    let new_index = (i + by) % old_col.len();
                    board[new_index][*col] = old_col[i];
                }
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = StringStreamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = || many1(digit()).map(|v: String| v.parse::<usize>().unwrap());
        // rect AxB
        let rect = string("rect")
            .with(char(' '))
            .with(num().skip(char('x')).and(num()))
            .map(|e| Rect {
                width: e.0,
                height: e.1,
            });
        // rotate row y=A by B
        let rot_row = string("rotate row y=")
            .with(num())
            .skip(string(" by "))
            .and(num())
            .map(|e| RotateRow { row: e.0, by: e.1 });
        let rot_column = string("rotate column x=")
            .with(num())
            .skip(string(" by "))
            .and(num())
            .map(|e| RotateColumn { col: e.0, by: e.1 });
        let mut parser = choice((attempt(rect), attempt(rot_row), rot_column));
        parser.parse(s).map(|e| e.0)
    }
}

fn display(board: &Vec<Vec<bool>>) -> String {
    board
        .iter()
        .map(|r| {
            r.iter()
                .map(|&b| if b { '#' } else { '.' })
                .collect::<String>()
        })
        .join("\n")
}

pub fn p1() -> usize {
    let mut board = vec![vec![false; 50]; 6];
    for i in parse_input_lines!(Instruction) {
        i.transform(&mut board)
    }
    println!("{}", display(&board));
    board.iter().map(|r| r.iter().filter(|b| **b).count()).sum()
}

// Too lazy to properly implement part2, so I just read the screen
