mod map;

use anyhow::Result;
use map::Map;

pub fn run(input: String) -> Result<()> {
    let mut map: Map = input.parse()?;

    Ok(())
}
