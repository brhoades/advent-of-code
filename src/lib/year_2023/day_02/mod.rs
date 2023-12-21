mod game;
use game::{CubeKind, Game, Games};

use std::collections::HashMap;

use crate::prelude::*;

pub fn run(input: String) -> Result<()> {
    let games: Games = input.parse()?;
    let rules: GameRules = [
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]
    .into_iter()
    .collect();

    println!(
        "total playable game sum: {}",
        playable_game_sum(&games, &rules)
    );

    println!("game power sum: {}", game_power_sum(&games));

    Ok(())
}

type GameRules = HashMap<CubeKind, usize>;

// sum of all playable game IDs with a set of rules.
fn playable_game_sum(games: &Games, rules: &GameRules) -> usize {
    games
        .iter()
        .filter(|g| g.playable_with_rules(rules))
        .map(|g| g.id)
        .reduce(std::ops::Add::add)
        .unwrap_or_default()
}

// s the sum of all game powers
fn game_power_sum(games: &Games) -> usize {
    games
        .iter()
        .map(|g| g.power())
        .reduce(std::ops::Add::add)
        .unwrap_or_default()
}

trait Playable {
    // given a configuration of inclusive upper counts of cubes,
    // returns if the game can be played.
    fn playable_with_rules(&self, rules: &GameRules) -> bool;
}

impl Playable for Game {
    fn playable_with_rules(&self, rules: &GameRules) -> bool {
        !self
            .cubes()
            // predicate halts iteration, returning **true**, if any constraint
            // is **violated**
            .any(|(cube, cnt)| cnt > rules.get(cube).unwrap_or(&usize::MAX))
    }
}
