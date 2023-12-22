use crate::prelude::*;
use itertools::Itertools;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Broken,
    Spring,
    Unknown,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Tile::*;

        match self {
            Broken => write!(f, "#"),
            Spring => write!(f, "."),
            Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowSpec {
    pub tiles: Vec<Tile>,
    pub seq: Vec<usize>,
}

impl fmt::Display for RowSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for t in &self.tiles {
            write!(f, "{t}")?;
        }

        write!(
            f,
            " {}",
            self.seq
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        )?;

        Ok(())
    }
}

impl FromStr for RowSpec {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use Tile::*;

        let mut pieces = s.split_whitespace().filter(|s| !s.is_empty());
        let tiles = pieces
            .next()
            .context("expected two space-separated pieces")?;
        let seq = pieces
            .next()
            .context("expected two space-separated pieces")?
            .split(',')
            .filter(|s| !s.is_empty())
            .map(FromStr::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            tiles: tiles
                .split("")
                .filter(|s| !s.is_empty())
                .map(|c| {
                    Ok(match c {
                        "#" => Broken,
                        "." => Spring,
                        "?" => Unknown,
                        other => bail!("failed to parse tile: '{other}'"),
                    })
                })
                .collect::<Result<_>>()?,
            seq,
        })
    }
}

impl RowSpec {
    // modifies the row, repeating the tiles and sequences with 5x
    // concatenated
    #[allow(unstable_name_collisions)]
    pub fn unfold(&mut self) {
        self.tiles = std::iter::repeat(self.tiles.iter())
            .intersperse([Tile::Unknown].iter())
            .flatten()
            .cloned()
            .take((self.tiles.len() + 1) * 5 - 1) // -1 as no trailing ?
            .collect();
        self.seq = std::iter::repeat(self.seq.iter())
            .flatten()
            .copied()
            .take(self.seq.len() * 5)
            .collect();
    }

    pub fn count(&self, tile: Tile) -> usize {
        self.tiles.iter().filter(|t| **t == tile).count()
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct Rows(Vec<RowSpec>);

impl FromStr for Rows {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.trim()
            .lines()
            .filter(|s| !s.is_empty())
            .map(FromStr::from_str)
            .collect::<Result<Vec<_>>>()
            .map(Rows)
    }
}

impl Rows {
    pub fn unfold(&mut self) {
        for row in &mut self.0 {
            row.unfold();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::EXAMPLE_1;
    use super::*;

    #[test]
    fn test_parse() {
        use Tile::*;
        let rows: Rows = EXAMPLE_1.parse().unwrap();

        let fst = rows.first().unwrap();

        assert_eq!(
            vec![Unknown, Unknown, Unknown, Spring, Broken, Broken, Broken],
            fst.tiles
        );
        assert_eq!(vec![1, 1, 3], fst.seq);

        assert_eq!(
            vec![
                Spring, Unknown, Unknown, Spring, Spring, Unknown, Unknown, Spring, Spring, Spring,
                Unknown, Broken, Broken, Spring
            ],
            rows.get(1).unwrap().tiles
        );
    }

    #[test]
    fn test_unfold() {
        let expected: RowSpec = ".#?.#?.#?.#?.# 1,1,1,1,1".parse().unwrap();
        let mut actual: RowSpec = ".# 1".parse().unwrap();
        actual.unfold();

        assert_eq!(
            expected, actual,
            "\nexpected:\t{expected}\ngot:\t\t{actual}"
        );

        let expected: RowSpec =
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
                .parse()
                .unwrap();
        let mut actual: RowSpec = "???.### 1,1,3".parse().unwrap();
        actual.unfold();

        assert_eq!(expected, actual);
    }
}
