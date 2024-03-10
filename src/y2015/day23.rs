use std::fs::read_to_string;

use Instruction::*;
use Reg::*;

enum Reg {
    A,
    B,
    C,
}
impl Reg {
    fn to_idx(&self) -> usize {
        match self {
            A => 0,
            B => 1,
            C => 2,
        }
    }
}
enum Instruction {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(i32),
    Jie(Reg, i32),
    Jio(Reg, i32),
}
impl Instruction {
    fn parse(str: &str) -> Self {
        let parse_reg = || -> Reg {
            match str.chars().nth(4).unwrap() {
                'a' => A,
                'b' => B,
                'c' => C,
                _ => panic!(),
            }
        };
        fn parse_offset(str: &str) -> i32 {
            str.parse().unwrap()
        }
        match &str[0..3] {
            "hlf" => Hlf(parse_reg()),
            "tpl" => Tpl(parse_reg()),
            "inc" => Inc(parse_reg()),
            "jmp" => Jmp(parse_offset(&str[4..])),
            "jie" => Jie(parse_reg(), parse_offset(&str[7..])),
            "jio" => Jio(parse_reg(), parse_offset(&str[7..])),
            _ => panic!(),
        }
    }
}

pub fn p1() -> usize {
    let instructions = read_to_string("inputs/day23.txt")
        .unwrap()
        .lines()
        .map(|s| Instruction::parse(s.trim()))
        .collect::<Vec<_>>();
    let mut state = vec![1, 0, 0];
    let mut pc: i32 = 0;
    while pc >= 0 && pc < instructions.len() as i32 {
        match &instructions[pc as usize] {
            Hlf(r) => {
                state[r.to_idx()] /= 2;
                pc += 1;
            }
            Tpl(r) => {
                state[r.to_idx()] *= 3;
                pc += 1;
            }
            Inc(r) => {
                state[r.to_idx()] += 1;
                pc += 1;
            }
            Jmp(o) => pc += o,
            Jie(r, o) => {
                if state[r.to_idx()] % 2 == 0 {
                    pc += o
                } else {
                    pc += 1
                }
            }
            Jio(r, o) => {
                if state[r.to_idx()] == 1 {
                    pc += o
                } else {
                    pc += 1
                }
            }
        }
    }
    dbg!(&state);
    state[1]
}
