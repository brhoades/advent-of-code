#![allow(dead_code)]
use std::cmp::{max, min};
use std::fmt;
use std::iter::repeat;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

pub use advent_of_code::{coord::Coordinate as BaseCoordinate, map::Map as BaseMap, prelude::*};

type Coordinate = BaseCoordinate<usize>;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum Tile {
    Source,
    Rock,
    Sand,
    #[default]
    Empty,
}
use Tile::*;

#[derive(Debug, Clone)]
pub struct Map {
    data: BaseMap<Tile>,
}

impl Map {
    pub fn get(&self, x: usize, y: usize) -> Result<&Tile> {
        self.data.get(x, y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut Tile> {
        self.data.get_mut(x, y)
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.data.resize(width, height)
    }

    pub fn iter_rows(&self) -> std::slice::Iter<Vec<Tile>> {
        self.data.iter_rows()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    /// bounds returns the topmost and bottommost nonempty coordinates
    /// None is returned if the dataset is empty.
    pub fn bounds(&self) -> Option<(Coordinate, Coordinate)> {
        let mut minc = None;
        let mut maxc = None;
        for (y, row) in self.iter_rows().enumerate() {
            for (x, t) in row.iter().enumerate() {
                if *t == Empty {
                    continue;
                }

                match minc {
                    None => minc = Some(Coordinate { x, y }),
                    Some(c) if c.x <= x || c.y <= y => {
                        minc = Some(Coordinate {
                            x: min(c.x, x),
                            y: min(c.y, y),
                        })
                    }
                    _ => (),
                }

                match maxc {
                    None => maxc = Some(Coordinate { x, y }),
                    Some(c) if c.x >= x || c.y >= y => {
                        maxc = Some(Coordinate {
                            x: max(c.x, x),
                            y: max(c.y, y),
                        })
                    }
                    _ => (),
                }
            }
        }

        minc.and_then(|min| maxc.map(|max| (min, max)))
    }

    pub fn height(&self) -> usize {
        self.dimensions.1
    }

    pub fn width(&self) -> usize {
        self.dimensions.0
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Source => write!(f, "+"),
            Rock => write!(f, "#"),
            Sand => write!(f, "o"),
            Empty => write!(f, "."),
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // we only print the minimal span of tiles which are not empty
        let b = self.bounds();
        if b.is_none() {
            return Ok(());
        }
        let (lower, upper) = b.unwrap();

        for (y, row) in self.data.iter_rows().enumerate() {
            if y > upper.y || y < lower.y {
                continue;
            }
            for col in &row[lower.x..=upper.x] {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for Map {
    type Err = Error;

    // takes line-by-line representation of wall lines
    // and derives a Map
    fn from_str(s: &str) -> Result<Self> {
        let lines: Vec<Vec<Coordinate>> = s.lines().map(parse_line).collect::<Result<_>>()?;
        let first = lines
            .first()
            .and_then(|l| l.first())
            .ok_or_else(|| anyhow!("expect at least one line"))?;
        let upper = lines
            .iter()
            .flatten()
            .fold((first.x, first.y), |last, curr| {
                (max(last.0, curr.x), max(last.1, curr.y))
            });

        let dimensions = ((upper.0 + 1) as usize, (upper.1 + 1) as usize); // add 1 since coordinates include 0

        let mut m = Self {
            data: BaseMap::new_dense(dimensions.0, dimensions.1),
        };

        // finally draw the wall lines on the map
        for line in lines {
            let mut last: Option<Coordinate> = None;
            for tp in line {
                if let Some(lp) = last {
                    let rng = if lp.x == tp.x {
                        repeat(lp.x)
                            .zip(min(lp.y, tp.y)..=max(lp.y, tp.y))
                            .collect::<Vec<(usize, usize)>>()
                    } else if lp.y == tp.y {
                        (min(lp.x, tp.x)..=max(lp.x, tp.x))
                            .zip(repeat(lp.y))
                            .collect::<Vec<(usize, usize)>>()
                    } else {
                        bail!(
                            "only vertical or horizontal line drawing is supported, got: {} -> {}",
                            lp,
                            tp
                        );
                    };

                    for (x, y) in rng {
                        *m.get_mut(x, y)? = Rock;
                    }
                }

                last = Some(tp)
            }
        }

        Ok(m)
    }
}

impl Deref for Map {
    type Target = BaseMap<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

fn parse_line(line: &str) -> Result<Vec<Coordinate>> {
    line.split(" -> ").map(FromStr::from_str).collect()
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_parse_map_basic() {
        let input = r#"2,2 -> 2,5 -> 4,5"#;
        let m: Map = input.parse().expect("should parse");

        let wall_coords = repeat(2)
            .zip(2..=5)
            .chain((2..=4).zip(repeat(5)))
            .collect::<HashSet<(usize, usize)>>();

        for x in 2..=4 {
            for y in 2..=5 {
                let c = m.get(x, y);
                assert!(c.is_ok(), "{:?}", c);
                let c = c.unwrap();

                if wall_coords.contains(&(x, y)) {
                    assert_eq!(Rock, *c, "expected rock at ({}, {})", x, y);
                } else {
                    assert_eq!(Empty, *c, "expected empty at ({}, {})", x, y);
                }
            }
        }
    }

    #[test]
    fn test_parse_map_ex1() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        let expected = r#"......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.
"#;

        let mut m: Map = input.parse().expect("should parse");
        // sand falls from (500, 0)
        *m.get_mut(500, 0).unwrap() = Source;

        assert_eq!(
            expected,
            &m.to_string(),
            "maps should equal:\nexpected:\n{}\n\nactual:\n{}\n",
            expected,
            m
        );
    }
}
