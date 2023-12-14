use std::mem;

use advent_of_code::prelude::*;
mod parse;
use parse::*;

pub fn run(input: String) -> Result<()> {
    let mut lines = input.lines();
    let mut dirs: Directions = lines.next().unwrap().parse()?;
    let _ = lines.next();
    let map: Map = lines.collect::<Vec<_>>().join("\n").parse()?;

    let cnt = traverse(&map, &mut dirs, "AAA", "ZZZ")?;
    println!("steps to ZZZ: {cnt}");

    let cnt = traverse_parallel(
        &map,
        &mut dirs,
        map.keys()
            .filter(|n| n.ends_with('A'))
            .map(|s| s.as_str())
            .collect(),
    )?;
    println!("steps for all nodes to end with Z: {cnt}");

    Ok(())
}

// traverses the map, returning the total number of
// nodes visited to get from start to end
pub fn traverse(map: &Map, dirs: &mut Directions, start: &str, end: &str) -> Result<usize> {
    let mut cnt = 0;
    let mut cur = start;

    while cur != end {
        let fork = map.get(cur).ok_or_else(|| anyhow!("invalid node {cur}"))?;
        cur = match dirs.next().unwrap() {
            Dir::Right => fork.right.as_ref(),
            Dir::Left => fork.left.as_ref(),
        };
        cnt += 1;
    }

    Ok(cnt)
}

// brute force traverses the map from all start points in parallel until all
// nodes end with a Z. It doesn't terminate.
#[allow(dead_code)]
pub fn traverse_parallel_dumb(map: &Map, dirs: &mut Directions, start: Vec<&str>) -> Result<usize> {
    let mut cnt = 0;
    let mut curs = start.clone();
    let mut done = !curs.iter().any(|n| !n.ends_with('Z'));
    let mut next = Vec::with_capacity(curs.len());

    while !done {
        done = true;
        // do one tick, tracking if any node does not end with Z
        for cur in &curs {
            let fork = map.get(*cur).ok_or_else(|| anyhow!("invalid node {cur}"))?;
            let cur: &str = match dirs.next().unwrap() {
                Dir::Right => fork.right.as_ref(),
                Dir::Left => fork.left.as_ref(), // wish I could borrow?
            };
            done &= cur.ends_with('Z');
            next.push(cur);
        }
        mem::swap(&mut next, &mut curs);
        next.clear();

        cnt += 1;
    }

    Ok(cnt)
}

// traverses the map from all start points in parallel until all
// nodes end with a Z. Cheats by taking the LCM of all start nodes
// cycle times, since cycles end on ZZZ
pub fn traverse_parallel(map: &Map, dirs: &mut Directions, start: Vec<&str>) -> Result<usize> {
    let mut cnt = 0;
    let mut curs = start.clone();
    let mut next = vec![""; curs.len()];
    let mut cycles = vec![0; curs.len()];

    while cycles.iter().any(|c| *c == 0) {
        let dir = dirs.next().unwrap();
        cnt += 1;

        // do one tick, tracking if any node does not end with Z
        for (i, cur) in curs.iter().enumerate() {
            if cycles[i] != 0 {
                continue;
            }
            let fork = map.get(*cur).ok_or_else(|| anyhow!("invalid node {cur}"))?;
            let cur: &str = match dir {
                Dir::Right => fork.right.as_ref(),
                Dir::Left => fork.left.as_ref(),
            };

            if cur.ends_with('Z') {
                cycles[i] = cnt;
            }

            next[i] = cur;
        }

        mem::swap(&mut next, &mut curs);
    }

    println!("cycles: {cycles:?}");

    Ok(cycles.into_iter().reduce(num::integer::lcm).unwrap())
}
