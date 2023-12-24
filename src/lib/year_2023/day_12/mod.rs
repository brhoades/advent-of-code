mod rows;

use std::io::Write;

use crate::prelude::*;
use itertools::{EitherOrBoth, Itertools};
use rows::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
        let v = row.combinations(Strategy::Planned);
        println!("{v}");
        sum += v;
    }

    println!("total unfolded combinations: {}", sum);
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Strategy {
    Brute,
    Planned,
}

impl RowSpec {
    // returns all possible arrangements of tiles with the given row's sequence.
    fn combinations(&self, strategy: Strategy) -> usize {
        match strategy {
            Strategy::Brute => {
                let mut cf = BruteComboFinder::new(self);
                cf.combinations_possible()
            }
            Strategy::Planned => {
                let mut cf = PlannedComboFinder::new(self, Default::default());
                cf.combinations_possible()
            }
        }
    }
}

impl Rows {
    #[allow(dead_code)]
    fn total_combinations(&self, strat: Strategy) -> usize {
        self.iter().map(|rspec| rspec.combinations(strat)).sum()
    }
}

struct BruteComboFinder {
    tiles: Vec<Tile>,
    seq: Vec<usize>,

    // counts
    unknown: usize,
    _spring: usize,
    broken: usize,
    cache: HashMap<Vec<Tile>, usize>,
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
            cache: Default::default(),
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
        sum += self.get_or_insert();

        *self.tiles.get_mut(idx).unwrap() = Tile::Broken;
        sum += self.get_or_insert();

        // reset so the parent stack can recurse cleanly
        *self.tiles.get_mut(idx).unwrap() = Tile::Unknown;

        sum
    }

    fn get_or_insert(&mut self) -> usize {
        if let Some(res) = self.cache.get(&self.tiles) {
            *res
        } else {
            let res = self.combinations_possible();
            self.cache.insert(self.tiles.clone(), res);
            res
        }
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
type ComboCache = Rc<RefCell<HashMap<(Vec<Tile>, Vec<usize>), usize>>>;

#[derive(Clone)]
struct PlannedComboFinder {
    tiles: Vec<Tile>,
    seq: Vec<usize>,

    cache: ComboCache,
}

impl PlannedComboFinder {
    fn new(rowspec: &RowSpec, cache: ComboCache) -> Self {
        Self {
            tiles: rowspec.tiles.clone(),
            seq: rowspec.seq.clone(),
            cache,
        }
    }

    // walks seq like a set of plans, inserting the required combo
    // if it's not possible, inserts a Spring and continues to recurse.
    fn combinations_possible(&mut self) -> usize {
        if !self.is_possible() {
            return 0;
        }
        trace!("============= ITER =================");
        if self.seq.is_empty() || self.tiles.is_empty() || self.seq.len() > self.tiles.len() {
            trace!(
                "done because: clause 1: {}, clause 2: {}, clause 3: {}",
                self.seq.is_empty(),
                self.tiles.is_empty(),
                self.seq.len() > self.tiles.len()
            );
            trace!(
                "tiles.len(): {}, seq.len(): {}",
                self.tiles.len(),
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

        let seq_len = self.seq.len();
        let seq_rng = 0..self.seq.first().copied().unwrap_or_default();
        let boundary_idx = seq_rng.len();

        trace!("seq: {:?}", self.seq);
        trace!(
            "tiles: {}",
            self.tiles.iter().map(ToString::to_string).join("")
        );
        trace!("seq_len: {seq_len}, seq_rng: {seq_rng:?}, boundary_idx: {boundary_idx}");

        // look forward:
        // Tiles must not be a spring to set the seq properly.
        if let Some(last_spring_idx) = self.tiles[seq_rng.clone()]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, t)| **t == Tile::Spring)
            .map(|(i, _)| i + seq_rng.start)
        {
            let mut s = self.clone_subproblem(last_spring_idx + 1, false);
            trace!(
                "\trecursing as all tiles in seq are spring / setting all tiles in rng to Spring"
            );
            let sum = s.solve_subproblem();

            return sum;
        }

        // look forward:
        // the bounary tile needs to be the end of the tiles, a ., or a ?
        if !(boundary_idx == self.tiles.len()
            || self.tiles[boundary_idx] == Tile::Spring
            || self.tiles[boundary_idx] == Tile::Unknown)
        {
            // skip forward, try again after
            // find the next broken tile for evluation, excluding this tile
            let first_broken = self.tiles[0..=boundary_idx]
                .iter()
                .enumerate()
                .skip(1)
                .find_or_first(|(_, t)| **t == Tile::Broken)
                .map(|(i, _)| i)
                .unwrap_or(boundary_idx);
            trace!("\trecursing to next unknown after setting all tiles in rng to Spring");
            let mut s = self.clone_subproblem(first_broken, false);
            return s.solve_subproblem();
        }

        // we can now set all tiles in rng to Tile::Broken. We also set boundary_idx to
        // spring if it's in range.
        let mut s = self.clone_subproblem(boundary_idx + 1, true);
        // s.broken += seq_rng.len();

        // and recurse.
        let mut sum = 0;
        trace!("\trecursing after setting this seq broken + 1 spring at {boundary_idx}");
        sum += s.solve_subproblem();
        std::mem::drop(s);

        // check just setting ourself to spring
        if self.tiles[0] == Tile::Unknown {
            trace!("\trecursing after setting unknown to spring");
            let mut s = self.clone_subproblem(1, false);
            sum += s.solve_subproblem();
        }

        sum
    }

    // basics--- can this still work out?
    fn is_possible(&self) -> bool {
        if !self.seq.is_empty() && self.tiles.is_empty() {
            return false;
        }

        self.seq.iter().sum::<usize>() <= self.tiles.iter().filter(|t| **t != Tile::Spring).count()
    }

    // clones this struct and solves it as a subproblem, reading from the cache if possible.
    fn solve_subproblem(&mut self) -> usize {
        if let Some(res) = self
            .cache
            .borrow()
            .get(&(self.tiles.clone(), self.seq.clone()))
        {
            debug!(
                "[cache]   all done: {} {}",
                self.tiles.iter().map(ToString::to_string).join(""),
                self.seq.iter().map(ToString::to_string).join(",")
            );
            return *res;
        }

        let res = self.combinations_possible();
        self.cache
            .borrow_mut()
            .insert((self.tiles.clone(), self.seq.clone()), res);

        res
    }

    // clones this struct, returning a subproblem with the reamining unsolved portion
    //
    // It trims tiles to be only after the provided offset. Seq is cloned
    // to exclude the first item. tile_idx and seq idx set to 0.
    fn clone_subproblem(&mut self, tile_offset: usize, step_seq: bool) -> Self {
        let tiles = if tile_offset >= self.tiles.len() {
            vec![]
        } else {
            self.tiles[tile_offset..].to_vec()
        };
        let seq = if step_seq {
            self.seq[1..].to_vec()
        } else {
            self.seq.to_vec()
        };
        Self {
            // broken: 0,      // tiles.iter().filter(|t| **t == Tile::Broken).count(),
            // rem_unknown: 0, // tiles.iter().filter(|t| **t == Tile::Unknown).count(),
            tiles,
            seq,
            cache: self.cache.clone(),
        }
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
        init_logging();
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
        init_logging();
        let r: RowSpec = "???.### 1,1,3".parse().unwrap();
        let mut cf = BruteComboFinder::new(&r);

        assert_eq!(1, cf.combinations_possible());
    }

    #[test]
    fn test_planned_combos_possible() {
        init_logging();
        let r: RowSpec = "???.### 1,1,3".parse().unwrap();
        let cache: ComboCache = Default::default();
        let mut cf = PlannedComboFinder::new(&r, cache.clone());

        assert_eq!(1, cf.combinations_possible());
    }

    #[test]
    fn test_example_1_brute_solution() {
        init_logging();
        let rows: Rows = EXAMPLE_1.parse().unwrap();
        let expected = [1, 4, 1, 1, 4, 10];

        for (i, (row, expected)) in rows.iter().zip(expected.iter()).enumerate() {
            println!(
                "========================\nROW #{}\n========================",
                i + 1
            );
            println!("{row}");
            assert_eq!(
                *expected,
                row.combinations(Strategy::Brute),
                "row #{} failed check",
                i + 1
            );
        }

        println!("========================\nALL COMBOS\n========================");
        assert_eq!(21, rows.total_combinations(Strategy::Brute));
    }

    // #[test]
    fn test_example_1_unfolded_brute_solution() {
        init_logging();
        let mut rows: Rows = EXAMPLE_1.parse().unwrap();
        let expected = [1, 16384, 1, 16, 2500, 506250];
        rows.unfold();

        for (i, (row, expected)) in rows.iter().zip(expected.iter()).enumerate() {
            println!(
                "========================\nROW #{}\n========================",
                i + 1
            );
            println!("{row}");
            assert_eq!(
                *expected,
                row.combinations(Strategy::Brute),
                "row #{} failed check",
                i + 1
            );
        }

        println!("========================\nALL COMBOS\n========================");
        assert_eq!(21, rows.total_combinations(Strategy::Brute));
    }

    #[test]
    fn test_example_1_planned_solution() {
        init_logging();
        let rows: Rows = EXAMPLE_1.parse().unwrap();
        let expected = [1, 4, 1, 1, 4, 10];

        for (i, (row, expected)) in rows.iter().zip(expected.iter()).enumerate() {
            println!(
                "========================\nROW #{}\n========================",
                i + 1
            );
            println!("{row}");
            assert_eq!(
                *expected,
                row.combinations(Strategy::Planned),
                "row #{} failed check",
                i + 1
            );
        }

        println!("========================\nALL COMBOS\n========================");
        assert_eq!(21, rows.total_combinations(Strategy::Planned));
    }

    #[test]
    fn test_example_1_unfolded_planned_solution() {
        init_logging();
        let mut rows: Rows = EXAMPLE_1.parse().unwrap();
        let expected = [1, 16384, 1, 16, 2500, 506250];
        rows.unfold();

        for (i, (row, expected)) in rows.iter().zip(expected.iter()).enumerate() {
            println!(
                "========================\nROW #{}\n========================",
                i + 1
            );
            println!("{row}");
            assert_eq!(
                *expected,
                row.combinations(Strategy::Planned),
                "row #{} failed check",
                i + 1
            );
        }

        println!("========================\nALL COMBOS\n========================");
        assert_eq!(21, rows.total_combinations(Strategy::Planned));
    }

    #[test]
    fn test_custom_unfolded_planned_solution() {
        init_logging();
        let rows = [
            ("?   1", 1),
            ("?.  1", 2_usize.pow(4)),
            (".?  1", 2_usize.pow(4)),
            ("??  2", 1),
            ("??. 2", 2_usize.pow(4)),
            (".?? 2", 2_usize.pow(4)),
            ("#.  1", 1),
            ("#   1", 1),
            (".#  1", 1),
            (".   1", 1),
        ]
        .into_iter()
        .map(|(s, c)| (s.parse::<RowSpec>().unwrap(), c))
        .enumerate();

        for (i, (mut row, expected)) in rows {
            row.unfold();
            println!(
                "========================\nROW #{}\n========================",
                i + 1
            );
            println!("{row}");
            assert_eq!(
                expected,
                row.combinations(Strategy::Planned),
                "row #{} failed check",
                i + 1
            );
        }
    }
}
