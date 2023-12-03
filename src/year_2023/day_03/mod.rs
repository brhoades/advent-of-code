mod map;
use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Symbol(String),
    Number(usize),
}
