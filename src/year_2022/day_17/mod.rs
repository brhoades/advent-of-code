mod jet;
mod map;
use jet::*;

use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    let g: JetPattern = input.parse()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
}
