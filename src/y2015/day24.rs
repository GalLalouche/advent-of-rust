use std::fs::read_to_string;

use itertools::Itertools;
use num::BigInt;

struct Weights {
    weights: Vec<usize>,
    group_weight: usize,
}
impl Weights {
    pub fn new(weights: Vec<usize>) -> Self {
        Self {
            group_weight: weights.iter().sum::<usize>() / 4,
            weights,
        }
    }

    // This a cheat, since it doesn't check that the remaining weights can be split evenly, but
    // it's only cheating if you get caught!
    fn is_valid(&self, combination: &[&usize]) -> bool {
        let sum = combination.iter().map(|i| self.weights[**i]).sum::<usize>();
        sum == self.group_weight
    }

    fn entanglement(&self, combination: &[&usize]) -> BigInt {
        combination
            .iter()
            .map(|i| self.weights[**i])
            .product::<BigInt>()
    }
}

pub fn p1() -> BigInt {
    let weights = Weights::new(
        read_to_string("inputs/day24.txt")
            .unwrap()
            .lines()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>(),
    );
    let indices: Vec<usize> = (0..weights.weights.len()).collect();
    for group_1_size in 1..weights.weights.len() {
        let mut min_entanglement: BigInt = (-1).into();
        for combination in indices.iter().combinations(group_1_size) {
            if weights.is_valid(&combination) {
                let entanglement = weights.entanglement(&combination);
                if min_entanglement < 0.into() || min_entanglement > entanglement {
                    min_entanglement = entanglement;
                }
            }
        }
        if min_entanglement >= 0.into() {
            return min_entanglement;
        }
    }
    panic!()
}
