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
    // Brute,
    Planned,
}

impl RowSpec {
    // returns all possible arrangements of tiles with the given row's sequence.
    fn combinations(&self, strategy: Strategy) -> usize {
        match strategy {
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
    level: usize,

    cache: ComboCache,
}

impl PlannedComboFinder {
    fn new(rowspec: &RowSpec, cache: ComboCache) -> Self {
        Self {
            tiles: rowspec.tiles.clone(),
            seq: rowspec.seq.clone(),
            level: 0,
            cache,
        }
    }

    // walks seq like a set of plans, inserting the required combo
    // if it's not possible, inserts a Spring and continues to recurse.
    fn combinations_possible(&mut self) -> usize {
        trace!("============= ITER depth={} =================", self.level);
        if self.seq.is_empty()
            || self.tiles.is_empty()
            || self.seq.len() > self.tiles.len()
            || self.seq.first().is_some_and(|seq| *seq > self.tiles.len())
        {
            trace!(
                "done because: clause 1: {}, clause 2: {}, clause 3: {}, clause 4: {}",
                self.seq.is_empty(),
                self.tiles.is_empty(),
                self.seq.len() > self.tiles.len(),
                self.seq.first().is_some_and(|seq| *seq > self.tiles.len()),
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

        let seq = self.seq.first().copied().unwrap();
        let rng = 0..seq;
        trace!(
            "eval: {} {}",
            self.tiles.iter().map(ToString::to_string).join(""),
            self.seq.iter().map(ToString::to_string).join(",")
        );
        debug!("seq_len: {seq}\trng: {:?}", rng.clone());

        let mut first_broken = None;
        let mut first_spring = None;
        let mut last_broken = None;
        let mut last_spring = None;
        let boundary_tile = self.tiles.get(seq);
        for (idx, t) in self.tiles[rng].iter().enumerate() {
            match *t {
                Tile::Broken => {
                    if first_broken.is_none() {
                        if first_broken.is_none() {
                            first_broken = Some(idx);
                        }
                        last_broken = Some(idx);
                    }
                }
                Tile::Spring => {
                    if first_spring.is_none() {
                        if first_spring.is_none() {
                            first_spring = Some(idx);
                        }
                        last_spring = Some(idx);
                    }
                }
                _ => (),
            }
        }
        debug!("first_broken: {first_broken:?} last_broken: {last_broken:?} first_spring: {first_spring:?} last_spring: {last_spring:?}");

        let mut sum = 0;
        match (last_spring, last_broken) {
            // no spring, boundary is a separator => pop seq, skip past boundary
            // in this case, we've set all unknowns to be broken, so we don't go to other cases.
            (None, _)
                if boundary_tile.is_none()
                    || !boundary_tile.is_some_and(|t| *t == Tile::Broken) =>
            {
                debug!("\tpopping seq and recursing to {}", seq + 1);
                sum += self.clone_subproblem(seq + 1, true).solve_subproblem();
            }
            // all unknowns but without a boundary tile. skip to boundary.
            (None, None) => {
                debug!("\tall unknowns with bad boundary, recursing to {seq}");
                // treat all skipped as dots
                return self.clone_subproblem(seq, false).solve_subproblem();
            }
            (Some(spring_idx), Some(broken_idx)) => {
                // spring in seq after broken? invalid, can't complete
                if spring_idx > broken_idx {
                    debug!(
                        "\t[invalid] broken found in incompletable branch: {} {}",
                        self.tiles.iter().map(ToString::to_string).join(""),
                        self.seq.iter().map(ToString::to_string).join(",")
                    );
                // before? reevaluate right after spring, treat everything before as springs
                } else {
                    debug!("\tskipping past last spring to {}", spring_idx + 1);
                    return self
                        .clone_subproblem(spring_idx + 1, false)
                        .solve_subproblem();
                }
            }
            (None, Some(idx)) if idx != 0 => {
                debug!("\tskipping to last broken spring @ {idx}");
                // evaluate @ the broken spring, we treat everything before as-is / as springs
                return self.clone_subproblem(idx, false).solve_subproblem();
            }
            (Some(idx), None) => {
                // evaluate one past the spring, treat everything before as-is/as springs
                debug!("\tskipping past last spring to {}", idx + 1);
                return self.clone_subproblem(idx + 1, false).solve_subproblem();
            }
            (None, Some(_)) => {
                debug!(
                    "\t[invalid] no springs in seq, but boundary missing: {} {}",
                    self.tiles.iter().map(ToString::to_string).join(""),
                    self.seq.iter().map(ToString::to_string).join(",")
                );
            }
        }

        // if we can be a spring, try it out
        if self.tiles[0] == Tile::Unknown {
            debug!("\tattempting to be spring");
            sum += self.clone_subproblem(1, false).solve_subproblem();
        }

        trace!(
            "====== iter end with sum={sum} @ level {} ======",
            self.level
        );
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
                "[cached]   all done: {} {} ==> {}",
                self.tiles.iter().map(ToString::to_string).join(""),
                self.seq.iter().map(ToString::to_string).join(","),
                res,
            );
            return *res;
        }

        let res = self.combinations_possible();
        self.cache
            .borrow_mut()
            .insert((self.tiles.clone(), self.seq.clone()), res);
        debug!(
            "[add cache]   {} {} ==> {}",
            self.tiles.iter().map(ToString::to_string).join(""),
            self.seq.iter().map(ToString::to_string).join(","),
            res,
        );

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
            tiles,
            seq,
            level: self.level + 1,
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
    fn test_planned_combos_possible() {
        init_logging();
        let r: RowSpec = "???.### 1,1,3".parse().unwrap();
        let cache: ComboCache = Default::default();
        let mut cf = PlannedComboFinder::new(&r, cache.clone());

        assert_eq!(1, cf.combinations_possible());
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
        assert_eq!(525152, rows.total_combinations(Strategy::Planned));
    }

    #[test]
    fn test_basic_count() {
        init_logging();
        let rows = [
            ("?   1", 1),
            ("?.  1", 1),
            (".?  1", 1),
            ("??  2", 1),
            ("??. 2", 1), // 5
            (".?? 2", 1),
            ("#.  1", 1),
            ("#   1", 1),
            (".#  1", 1),
            (".   1", 0), // 10
            ("... 3", 0),
            ("??  1", 2),
            ("?.? 1", 2),
            (".?? 1", 2),
            ("??. 1", 2), // 15
            ("??.#.# 1,1,1", 2),
            ("#..# 1,1", 1),
            (".?.# 1,1", 1),
            (".?.#   1", 1),
        ]
        .into_iter()
        .map(|(s, c)| (s.parse::<RowSpec>().unwrap(), c))
        .enumerate();

        for (i, (mut row, expected)) in rows {
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

    #[test]
    fn test_planned_regression_6_5() {
        init_logging();
        // this case flares up from time to time
        let row: RowSpec = "??.######..#####. 6,5".parse().unwrap();
        assert_eq!(1, row.combinations(Strategy::Planned));
    }
}
