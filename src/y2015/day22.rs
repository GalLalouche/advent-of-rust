use std::collections::BinaryHeap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> usize {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
struct GameState {
    player_hp: usize,
    player_mana: usize,
    player_armor: usize,
    mana_spent: usize,
    boss_hp: i32,
    shield_turns: usize,
    poison_turns: usize,
    recharge_turns: usize,
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            (self.mana_spent, self.boss_hp, -(self.player_hp as i32))
                .cmp(&(other.mana_spent, other.boss_hp, -(other.player_hp as i32)))
                .reverse(),
        )
    }
}

impl GameState {
    fn apply_effects(&mut self) {
        if self.shield_turns > 0 {
            self.player_armor = 7;
            self.shield_turns -= 1;
        } else {
            self.player_armor = 0;
        }
        if self.poison_turns > 0 {
            self.poison_turns -= 1;
            self.boss_hp -= 3;
        }
        if self.recharge_turns > 0 {
            self.recharge_turns -= 1;
            self.player_mana += 101;
        }
    }

    fn next_states(&self) -> Vec<GameState> {
        Spell::iter()
            .filter(|s| {
                self.player_mana >= s.cost()
                    && match s {
                        Spell::MagicMissile | Spell::Drain => true,
                        Spell::Shield => self.shield_turns == 0,
                        Spell::Poison => self.poison_turns == 0,
                        Spell::Recharge => self.recharge_turns == 0,
                    }
            })
            .map(|s| {
                let mut result = self.clone();
                result.player_mana -= s.cost();
                result.mana_spent += s.cost();
                match s {
                    Spell::MagicMissile => {
                        result.boss_hp -= 4;
                    }
                    Spell::Drain => {
                        result.boss_hp -= 2;
                        result.player_hp += 2;
                    }
                    Spell::Shield => result.shield_turns = 6,
                    Spell::Poison => result.poison_turns = 6,
                    Spell::Recharge => result.recharge_turns = 5,
                };
                result
            })
            .collect()
    }

    fn is_win(&self) -> bool {
        self.boss_hp <= 0
    }
}

fn main() {
    let init_state = GameState {
        player_hp: 50,
        player_mana: 500,
        player_armor: 0,
        mana_spent: 0,
        boss_hp: 58,
        shield_turns: 0,
        poison_turns: 0,
        recharge_turns: 0,
    };
    let mut heap = BinaryHeap::new();
    let boss_damage = 9;
    heap.push(init_state);
    loop {
        let mut next = heap.pop().unwrap();
        if next.player_hp == 1 {
            continue;
        }
        next.player_hp -= 1;
        next.apply_effects();
        let next = next;
        if next.is_win() {
            dbg!(&next.mana_spent);
            return;
        }
        for mut next_state in next.next_states() {
            next_state.player_hp -= 1;
            next_state.apply_effects();
            if next.player_hp == 1 {
                continue;
            }
            if next_state.is_win() {
                dbg!(&next_state.mana_spent);
                return;
            }
            let damage_done = std::cmp::max(1, boss_damage - next_state.player_armor);
            if next_state.player_hp <= damage_done {
                continue;
            }
            next_state.player_hp -= damage_done;
            heap.push(next_state);
        }
    }
}
