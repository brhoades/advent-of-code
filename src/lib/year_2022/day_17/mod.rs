mod jet;
mod map;
use jet::*;

use crate::prelude::*;

pub fn run(input: String) -> Result<()> {
    let _: JetPattern = input.parse()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    const _EXAMPLE_ONE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
}
