mod jet;
mod map;
use jet::*;

use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    let _: JetPattern = input.parse()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    const _EXAMPLE_ONE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
}
