use std::cmp::max;

use super::map::{Map, Tile::*};


/// resizes to be larger, then draws a long line at bounds.1.1 + 2
pub fn time_until_source_covered(m: &mut Map, spawn: (usize, usize)) -> usize {
    let dimens = m.dimensions();
    let b = m.bounds().unwrap();
    let (bminx, bmaxx, bmaxy) = (b.0.x, b.1.x, b.1.y);

    let line_width = dimens.1 - spawn.1; // the bottom portion of our 45-45-90 triangle
    let bmaxx = if bminx < line_width {
        bmaxx + line_width + line_width - bminx
    } else {
        bmaxx + line_width
    };
    let bminx = max(0, bminx-line_width);

    if spawn.0 > dimens.0 || spawn.1 > dimens.1 || bmaxy + 2 > dimens.1 {
        let ndimens = (max(bmaxx + 1, spawn.0), max(max(dimens.1, spawn.1), bmaxy + 3));
        println!("resize m from {:?} to {:?} so that it includes spawn ({:?}) and line (y={})", dimens, ndimens, spawn, bmaxy + 2);
        m.resize(ndimens.0, ndimens.1);
    }


    println!("drawing line from x={} to x={}", bminx, bmaxx);
    *m.get_mut(spawn.0, spawn.1).unwrap() = Source;
    for x in bminx..=bmaxx {
        *m.get_mut(x, bmaxy + 2).unwrap() = Rock;
        // println!("({}, {}) = {}", x, bmaxy + 2, m.get(x, bmaxy + 2).unwrap());
    }

    let mut cnt = 0;
    loop {
        // println!("\n{}", m);
        if spawn_sand(m, (spawn.0, spawn.1)).is_none() {
            println!("could no longer place sand while trying to cover source:\n{}", m);
            return 0;
        }
        cnt += 1;

        match m.get(spawn.0, spawn.1) {
            Ok(Sand) => break,
            _ => (),
        }
    }

    cnt
}

pub fn time_until_full(m: &mut Map, spawn: (usize, usize)) -> usize {
    let dimens = m.dimensions();
    if spawn.0 > dimens.0 || spawn.1 > dimens.1 {
        println!("resize m from {:?} to include spawn: {:?}", dimens, spawn);
        m.resize(max(dimens.0, spawn.0), max(dimens.1, spawn.1));
    }
    *m.get_mut(spawn.0, spawn.1).unwrap() = Source;

    let mut cnt = 0;
    loop {
        if spawn_sand(m, (spawn.0, spawn.1)).is_none() {
            break;
        }
        // println!("\n{}", m);
        cnt += 1;
    }

    cnt
}


/// spawn_sand spawns sand at the passed coordinates and simulates
/// phyiscs on it until it drops. If it does not settle and instead falls
/// off the map, None is returned.
pub fn spawn_sand(m: &mut Map, at: (usize, usize)) -> Option<()> {
    // We won't modify the map until we are sure the sand settles. There's no
    // point since it doesn't affect the result.
    match m.get(at.0, at.1) {
        Ok(Empty) | Ok(Source) => (),
        Ok(_) => {
            println!("cannot spawn at {:?}, already taken", at);
            return None;
        },
        _ => (),
    }

    for (x, y) in vec![(at.0, at.1+1), (at.0-1, at.1+1), (at.0+1, at.1+1)] {
        match m.get(x, y) {
            Ok(Empty) => {
                // println!("{:?} => ({}, {})", at, x, y);
                return spawn_sand(m, (x, y))
            },
            Err(_) => return None,
            Ok(_) => (),

        }
    }

    // sand settles, no empty spot
    match m.get_mut(at.0, at.1) {
        Ok(ref mut t) => {
            **t = Sand;
            Some(())
        },
        _ => None,
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sand_drop_simple() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        let mut m: Map = input.parse().expect("should parse");
        // sand falls from (500, 0)
        *m.get_mut(500, 0).unwrap() = Source;

        assert_eq!(Some(()), spawn_sand(&mut m, (500, 0)));
        assert_eq!(Sand, *m.get(500, 8).unwrap(), "{}", m);
    }

    #[test]
    fn test_sand_drop_ex1() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        let mut m: Map = input.parse().expect("should parse");
        let cnt = time_until_full(&mut m, (500, 0));

        assert_eq!(24, cnt, "\n{}", m);
    }

    #[test]
    fn test_sand_drop_ex1_pt2() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        let mut m: Map = input.parse().expect("should parse");
        let cnt = time_until_source_covered(&mut m, (500, 0));

        assert_eq!(93, cnt, "\n{}", m);
    }
}
