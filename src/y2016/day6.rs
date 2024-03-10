use std::{cmp::Reverse, collections::HashMap};

use itertools::Itertools;

use crate::read_input_lines;

fn solution(sort_by_key: impl Fn(usize) -> i32) -> String {
    let len = read_input_lines!().next().unwrap().len();
    let mut results = vec![HashMap::<char, usize>::new(); len];
    for line in read_input_lines!() {
        for (i, c) in line.chars().enumerate() {
            *results[i].entry(c).or_insert(0) += 1;
        }
    }
    results
        .iter()
        .map(|hm| {
            hm.iter()
                .sorted_by_key(|e| sort_by_key(*e.1))
                .map(|e| e.0)
                .next()
                .unwrap()
        })
        .collect()
}

pub fn p1() -> String {
    solution(|e: usize| -1 * (e as i32))
}

pub fn p2() -> String {
    solution(|e: usize| e as i32)
}
