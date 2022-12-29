#![allow(dead_code)]
use std::cmp::{max, min};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::iter::repeat;

pub use advent_of_code::{prelude::*, coord::Coordinate as BaseCoordinate, map::Map as BaseMap};

type Coordinate = BaseCoordinate<i32>;

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
    // offsets gets
    offset: (i32, i32),
    bounds: ((i32, i32), (i32, i32)), // ((min_x, min_y), (max_x, max_y))
    dimensions: (usize, usize),       // width and height
}

impl Map {
    pub fn get(&self, x: i32, y: i32) -> Result<&Tile> {
        if x > self.bounds.1.0 || x < self.bounds.0.0 {
            bail!("x coordinate in get out of range: ({}, {}) w/ bounds {:?}", x, y, self.bounds);
        } else if y > self.bounds.1.1 || y < self.bounds.0.1 {
            bail!("y coordinate in get out of range: ({}, {}) w/ bounds {:?}", x, y, self.bounds);
        }

        self.data.get(
            (x as i32 + self.offset.0).try_into()?,
            (y as i32 + self.offset.1).try_into()?,
        )
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Result<&mut Tile> {
        if x > self.bounds.1.0 || x < self.bounds.0.0 {
            bail!("x coordinate in get out of range: ({}, {}) w/ bounds {:?}", x, y, self.bounds);
        } else if y > self.bounds.1.1 || y < self.bounds.0.1 {
            bail!("y coordinate in get out of range: ({}, {}) w/ bounds {:?}", x, y, self.bounds);
        }

        self.data.get_mut(
            (x as i32 + self.offset.0).try_into()?,
            (y as i32 + self.offset.1).try_into()?,
        )
    }

    /// bounds: ((min_x, min_y), (max_x, max_y))
    /// must be larger than existing bounds
    pub fn resize(&mut self, bounds: ((i32, i32), (i32, i32))) {
        let (lower, upper) = bounds;
        let dimensions = ((upper.0 - lower.0 + 1) as usize, (upper.1 - lower.1 + 1) as usize); // add 1 since coordinates include 0

        let mut newdata = BaseMap::new_dense(dimensions.0, dimensions.1);

        for (y, row) in self.iter_rows().enumerate().map(|(y, row)| (y as i32 - self.offset.1, row)) {
            for (x, col) in row.iter().enumerate().map(|(x, col)| (x as i32 - self.offset.0, col)) {
                *newdata.get_mut(x.try_into().unwrap(), y.try_into().unwrap()).unwrap() = col.clone();
            }
        }

        self.dimensions = dimensions;
        self.data = newdata;
        self.bounds = bounds;
    }

    pub fn iter_rows(&self) -> std::slice::Iter<Vec<Tile>> {
        self.data.iter_rows()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    pub fn bounds(&self) -> ((i32, i32), (i32, i32)) {
        self.bounds
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
        for row in self.data.iter_rows() {
            for col in row {
                write!(f, "{}", col)?;
            }
            write!(f, "\n")?;
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
        let mut bounds = (0, 0);
        let first = lines.first()
            .and_then(|l| l.first())
            .ok_or_else(|| anyhow!("expect at least one line"))?;
        let upper = lines.iter().flatten().fold((first.x, first.y), |last, curr| {
            (max(last.0, curr.x), max(last.1, curr.y))
        });
        let lower = lines.iter().flatten().fold((first.x, first.y), |last, curr| {
            (min(last.0, curr.x), min(last.1, curr.y))
        });

        let dimensions = ((upper.0 - lower.0 + 1) as usize, (upper.1 - lower.1 + 1) as usize); // add 1 since coordinates include 0

        let mut m = Self {
            data: BaseMap::new_dense(dimensions.0, dimensions.1),
            offset: (0 as i32 - lower.0 as i32, 0 as i32 - lower.1 as i32),
            bounds: (lower, upper),
            dimensions,
        };

        // finally draw the wall lines on the map
        for line in lines {
            let mut last: Option<Coordinate> = None;
            for tp in line {
                if let Some(lp) = last {
                    let rng = if lp.x == tp.x {
                        repeat(lp.x).zip(min(lp.y, tp.y)..=max(lp.y, tp.y)).collect::<Vec<(i32, i32)>>()
                    } else if lp.y == tp.y {
                        (min(lp.x, tp.x)..=max(lp.x, tp.x)).zip(repeat(lp.y)).collect::<Vec<(i32, i32)>>()
                    } else {
                        bail!("only vertical or horizontal line drawing is supported, got: {} -> {}", lp, tp);
                    };

                    for (x, y) in rng {
                        println!("set ({}, {}) to rock", x, y);
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
            .zip(2..=5).chain((2..=4).zip(repeat(5)))
            .collect::<HashSet<(i32, i32)>>();

        for x in 2..=4 {
            for y in 2..=5 {
                let c = m.get(x, y);
                assert!(!c.is_err(), "{:?}", c);
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

        let expected = "# ......+...;
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.#";

        let mut m: Map = input.parse().expect("should parse");
        let bounds = m.bounds();
        m.resize(((bounds.0.0, 0), (bounds.1.0, bounds.1.1)));
        // sand falls from (500, 0)
        *m.get_mut(500, 0).unwrap() = Source;

        assert_eq!(expected, &m.to_string(), "maps should equal:\nexpected:\n{}\n\nactual:\n{}\n", expected, m);
    }
}
