mod map;
mod physics;

use anyhow::Result;
use map::Map;
use physics::{time_until_full, time_until_source_covered};

pub fn run(input: String) -> Result<()> {
    let mut m: Map = input.parse()?;
    let cnt = time_until_full(&mut m, (500, 0));

    println!("took {} rounds to fill", cnt);


    println!("========= part 2 =========");
    let mut m: Map = input.parse()?;
    let cnt = time_until_source_covered(&mut m, (500, 0));

    println!("took {} rounds for sand to cover source", cnt);

    Ok(())
}
