mod rows;

use std::io::Write;

use crate::prelude::*;
use itertools::{EitherOrBoth, Itertools};
use rows::*;

pub fn run(input: String) -> Result<()> {
    let mut rows: Rows = input.parse()?;

    // println!("total combinations: {}", rows.total_combinations());

    print!("unfolding... ");
    rows.unfold();
    println!("done");

    let mut sum = 0;
    for (i, row) in rows.iter().enumerate() {
        print!("{i}: ");
        let _ = std::io::stdout().flush();
        let v = row.combinations();
        println!("{v}");
        sum += v;
    }

    println!("total unfolded combinations: {}", sum);
    Ok(())
}

impl RowSpec {
    // returns all possible arrangements of tiles with the given row's sequence.
    fn combinations(&self) -> usize {
        let mut cf = PlannedComboFinder::new(self);
        cf.combinations_possible()
    }
}

impl Rows {
    fn total_combinations(&self) -> usize {
        self.iter().map(|rspec| rspec.combinations()).sum()
    }
}

struct BruteComboFinder {
    tiles: Vec<Tile>,
    seq: Vec<usize>,

    // counts
    unknown: usize,
    _spring: usize,
    broken: usize,
}

impl BruteComboFinder {
    fn new(rowspec: &RowSpec) -> Self {
        Self {
            tiles: rowspec.tiles.clone(),
            seq: rowspec.seq.clone(),
            unknown: rowspec
                .tiles
                .iter()
                .filter(|t| **t == Tile::Unknown)
                .count(),
            _spring: rowspec.tiles.iter().filter(|t| **t == Tile::Spring).count(),
            broken: rowspec.tiles.iter().filter(|t| **t == Tile::Broken).count(),
        }
    }

    fn combinations_possible(&mut self) -> usize {
        if !self.is_possible_v2() {
            return 0;
        }
        let Some((idx, _)) = self
            .tiles
            .iter()
            .enumerate()
            .find(|(_, t)| **t == Tile::Unknown)
        else {
            if is_valid(&self.tiles, &self.seq) {
                debug!(
                    "[valid]   all done: {} {}",
                    self.tiles.iter().map(ToString::to_string).join(""),
                    self.seq.iter().map(ToString::to_string).join(",")
                );
                return 1;
            } else {
                debug!(
                    "[invalid] all done: {} {}",
                    self.tiles.iter().map(ToString::to_string).join(""),
                    self.seq.iter().map(ToString::to_string).join(",")
                );
                return 0;
            }
        };

        trace!("idx: {idx}");

        let mut sum = 0;
        *self.tiles.get_mut(idx).unwrap() = Tile::Spring;
        sum += self.combinations_possible();

        *self.tiles.get_mut(idx).unwrap() = Tile::Broken;
        sum += self.combinations_possible();

        // reset so the parent stack can recurse cleanly
        *self.tiles.get_mut(idx).unwrap() = Tile::Unknown;

        sum
    }

    // does quick match to determine if it's a possible tree.
    // If the remaining unknown tiles is less than the sum of unknowns required, bial.
    fn is_possible_v2(&self) -> bool {
        (self.seq.iter().sum::<usize>() as i64 - self.broken as i64) <= self.unknown as i64
    }
}

// is_possible returns whether the given tiles can still match seq.
fn is_possible_v1(tiles: &[Tile], seq: &[usize]) -> bool {
    tiles
        .split(|t| *t == Tile::Spring)
        .map(|s| {
            let (broken, unknown) = s.iter().partition::<Vec<&Tile>, _>(|s| **s == Tile::Broken);
            (broken.len(), unknown.len())
        })
        .filter(|(l, r)| *l != 0 || *r != 0)
        .zip_longest(seq.iter())
        .all(|pair| match pair {
            EitherOrBoth::Both((broken, unknown), expected_broken) => {
                broken == *expected_broken
                    || ((broken as i64 - *expected_broken as i64).unsigned_abs() as usize) < unknown
            }
            EitherOrBoth::Right(_) => true,
            EitherOrBoth::Left(_) => false,
        })
}

// returns true when contiguous broken tiles are in the size of groups provided
// by seq
fn is_valid(tiles: &[Tile], seq: &[usize]) -> bool {
    tiles
        .split(|t| *t != Tile::Broken)
        .map(|s| s.len())
        .filter(|l| *l != 0)
        .zip_longest(seq.iter())
        .all(|pair| match pair {
            EitherOrBoth::Both(l, r) => l == *r,
            _ => false,
        })
}

#[derive(Clone)]
struct PlannedComboFinder {
    tiles: Vec<Tile>,
    seq: Vec<usize>,
    seq_idx: usize,  // the current sequence entry to satisfy
    tile_idx: usize, // the current tile we are evaluating an insert on

    // optimizations to bail early when we don't have enough left
    rem_unknown: usize, // unknown tiles left in the tiles vec
    broken: usize,      // total broken tiles
}

impl PlannedComboFinder {
    fn new(rowspec: &RowSpec) -> Self {
        Self {
            tiles: rowspec.tiles.clone(),
            seq: rowspec.seq.clone(),
            seq_idx: 0,
            tile_idx: 0,
            rem_unknown: rowspec.count(Tile::Unknown),
            broken: rowspec.count(Tile::Broken),
        }
    }

    // walks seq like a set of plans, inserting the required combo
    // if it's not possible, inserts a Spring and continues to recurse.
    fn combinations_possible(&mut self) -> usize {
        if !self.is_possible() {
            return 0;
        }
        trace!("============= ITER =================");
        if self.seq_idx >= self.seq.len()
            || self.tile_idx >= self.tiles.len()
            || self.seq[self.seq_idx] + self.tile_idx > self.tiles.len()
        {
            trace!(
                "done because: clause 1: {}, clause 2: {}, clause 3: {}",
                self.seq_idx >= self.seq.len(),
                self.tile_idx >= self.tiles.len(),
                self.seq_idx < self.seq.len()
                    && self.seq[self.seq_idx] + self.tile_idx >= self.tiles.len()
            );
            trace!(
                "tile_idx: {}, tiles.len(): {}, seq_idx: {}, seq.len(): {}",
                self.tile_idx,
                self.tiles.len(),
                self.seq_idx,
                self.seq.len()
            );
            if is_valid(&self.tiles, &self.seq) {
                debug!(
                    "[valid]   all done: {} {}",
                    self.tiles.iter().map(ToString::to_string).join(""),
                    self.seq.iter().map(ToString::to_string).join(",")
                );
                return 1; // TODO it's probably more complicated than this
            } else {
                debug!(
                    "[invalid] all done: {} {}",
                    self.tiles.iter().map(ToString::to_string).join(""),
                    self.seq.iter().map(ToString::to_string).join(",")
                );
                return 0;
            }
        }

        let seq_len = self.seq[self.seq_idx];
        let boundary_idx = self.tile_idx + seq_len;
        let seq_rng = self.tile_idx..boundary_idx;

        trace!("seq: {:?}", self.seq);
        trace!(
            "tiles: {}",
            self.tiles.iter().map(ToString::to_string).join("")
        );
        trace!("seq_len: {seq_len}, seq_rng: {seq_rng:?}, boundary_idx: {boundary_idx}, tile_idx: {}, seq_idx: {}",self.tile_idx, self.seq_idx);

        // look forward:
        // Tiles must not be a spring to set the seq properly.
        if let Some(last_spring_idx) = self.tiles[seq_rng.clone()]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, t)| **t == Tile::Spring)
            .map(|(i, _)| i + seq_rng.start)
        {
            // skip past the last spring and set everything in between to springs
            let mut s = self.clone();
            for t in &mut s.tiles[seq_rng.start..last_spring_idx] {
                if *t == Tile::Unknown {
                    *t = Tile::Spring;
                }
            }
            s.tile_idx = last_spring_idx + 1;
            trace!(
                "\trecursing as all tiles in seq are spring / setting all tiles in rng to Spring"
            );
            return s.combinations_possible();
        }

        // look forward:
        // the bounary tile needs to be the end of the tiles, a ., or a ?
        if !(boundary_idx == self.tiles.len()
            || self.tiles[boundary_idx] == Tile::Spring
            || self.tiles[boundary_idx] == Tile::Unknown)
        {
            // skip forward, try again after
            let mut s = self.clone();
            let mut first_broken = boundary_idx; // default to moving to the end if we don't find one
            for i in seq_rng.clone() {
                if s.tiles[i] == Tile::Unknown {
                    s.tiles[i] = Tile::Spring;
                } else if first_broken == boundary_idx
                    && i != seq_rng.start
                    && s.tiles[i] == Tile::Broken
                {
                    first_broken = i;
                }
            }
            // find the next broken tile for evluation, excluding this tile
            s.tile_idx = s.tiles[(self.tile_idx + 1)..=boundary_idx]
                .iter()
                .enumerate()
                .find_or_first(|(_, t)| **t == Tile::Broken)
                .map(|(i, _)| i + self.tile_idx + 1)
                .unwrap_or(boundary_idx);
            trace!("\trecursing to next unknown after setting all tiles in rng to Spring");
            return s.combinations_possible();
        }

        // we can now set all tiles in rng to Tile::Broken. We also set boundary_idx to
        // spring if it's in range.
        let mut s = self.clone();
        s.broken += seq_rng.len();
        for idx in seq_rng {
            if s.tiles[idx] == Tile::Unknown {
                s.tiles[idx] = Tile::Broken;
                s.rem_unknown -= 1;
            }
        }
        if boundary_idx != self.tiles.len() {
            s.tiles[boundary_idx] = Tile::Spring;
        }

        // and recurse.
        let mut sum = 0;
        s.tile_idx = boundary_idx + 1;
        s.seq_idx += 1;
        trace!("\trecursing after setting this seq to broken");
        sum += s.combinations_possible();

        // check just setting ourself to spring
        if self.tiles[self.tile_idx] == Tile::Unknown {
            self.tiles[self.tile_idx] = Tile::Spring;
            trace!("\trecursing after setting unknown to spring");
            self.tile_idx += 1;
            sum += self.combinations_possible();

            // reset for caller
            self.tile_idx -= 1;
            self.tiles[self.tile_idx] = Tile::Unknown;
        }

        sum
    }

    // basics--- can this still work out?
    fn is_possible(&self) -> bool {
        if self.tile_idx >= self.tiles.len() || self.seq_idx > self.seq.len() {
            return true;
        }
        self.seq[self.seq_idx..].iter().sum::<usize>()
            <= self.tiles[self.tile_idx..]
                .iter()
                .filter(|t| **t != Tile::Spring)
                .count()
    }
}

pub const EXAMPLE_1: &str = r"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_valid() {
        use Tile::*;

        assert!(is_valid(
            &[Spring, Spring, Broken, Spring, Spring, Broken, Spring],
            &[1, 1]
        ));
        assert!(!is_valid(
            &[Spring, Spring, Broken, Spring, Spring, Broken, Spring],
            &[1, 1, 1]
        ));
        assert!(!is_valid(
            &[Spring, Spring, Broken, Spring, Spring, Broken, Spring],
            &[2]
        ));
        assert!(!is_valid(
            &[Spring, Spring, Broken, Spring, Spring, Broken, Spring],
            &[1]
        ));
    }

    #[test]
    fn test_brute_combos_possible() {
        let r: RowSpec = "???.### 1,1,3".parse().unwrap();
        let mut cf = BruteComboFinder::new(&r);

        assert_eq!(1, cf.combinations_possible());
    }

    #[test]
    fn test_planned_combos_possible() {
        let r: RowSpec = "???.### 1,1,3".parse().unwrap();
        let mut cf = PlannedComboFinder::new(&r);

        assert_eq!(1, cf.combinations_possible());
    }

    #[test]
    fn test_example_1_solution() {
        let rows: Rows = EXAMPLE_1.parse().unwrap();
        let expected = [1, 4, 1, 1, 4, 10];

        for (i, (row, expected)) in rows.iter().zip(expected.iter()).enumerate() {
            println!(
                "========================\nROW #{}\n========================",
                i + 1
            );
            println!("{row}");
            assert_eq!(*expected, row.combinations(), "row #{} failed check", i + 1);
        }

        println!("========================\nALL COMBOS\n========================");
        assert_eq!(21, rows.total_combinations());
    }
}
