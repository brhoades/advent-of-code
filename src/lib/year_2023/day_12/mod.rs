mod rows;

use crate::prelude::*;
use itertools::{EitherOrBoth, Itertools};
use rows::*;

pub fn run(input: String) -> Result<()> {
    let mut rows: Rows = input.parse()?;

    println!("total combinations: {}", rows.total_combinations());

    print!("unfolding... ");
    rows.unfold();
    println!("done");

    let mut sum = 0;
    for (i, row) in rows.iter().enumerate() {
        let v = row.combinations();
        println!("{i}: {}", v.len());
        sum += v.len();
    }

    println!("total unfolded combinations: {}", sum);
    Ok(())
}

impl RowSpec {
    // returns all possible arrangements of tiles with the given row's sequence.
    fn combinations(&self) -> Vec<Vec<Tile>> {
        combinations_possible(&self.tiles, &self.seq)
    }
}

impl Rows {
    fn total_combinations(&self) -> usize {
        self.iter()
            .map(|rspec| combinations_possible(&rspec.tiles, &rspec.seq).len())
            .sum()
    }
}

// Using tiles, returns all possible, valid combinations of tiles.
// If tiles are all known, returns an empty vec.
fn combinations_possible(tiles: &[Tile], seq: &[usize]) -> Vec<Vec<Tile>> {
    let Some(idx) = tiles.iter().enumerate().find(|(_, t)| **t == Tile::Unknown) else {
        if is_valid(&tiles, seq) {
            debug!("[valid] all done: {tiles:?}");
            return vec![tiles.to_owned()];
        } else {
            debug!("[invalid] all done: {tiles:?}");
            return vec![];
        }
    };
    if !is_possible(&tiles, &seq) {
        return vec![];
    }

    let idx = idx.0;
    debug!("idx: {idx}");

    let mut tiles: Vec<Tile> = tiles.to_vec();
    *tiles.get_mut(idx).unwrap() = Tile::Spring;

    let mut opts = combinations_possible(&tiles, seq);
    *tiles.get_mut(idx).unwrap() = Tile::Broken;

    opts.extend(combinations_possible(&tiles, seq));
    opts
}

// is_possible returns whether the given tiles can still match seq.
fn is_possible(tiles: &[Tile], seq: &[usize]) -> bool {
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
    fn test_combos_possible() {
        let r: RowSpec = "???.### 1,1,3".parse().unwrap();

        let combos = combinations_possible(&r.tiles, &r.seq);
        assert_eq!(1, combos.len());
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
            assert_eq!(
                *expected,
                row.combinations().len(),
                "row #{} failed check",
                i + 1
            );
        }

        assert_eq!(21, rows.total_combinations());
    }
}
