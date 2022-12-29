use std::cmp::{max, min};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use anyhow::{bail, Error, Result};

pub use advent_of_code::{coord::Coordinate as BaseCoordinate, map::Map as BaseMap};

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
        self.data.get(
            (x as i32 + self.offset.0) as usize,
            (y as i32 + self.offset.1) as usize,
        )
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Result<&mut Tile> {
        self.data.get_mut(
            (x as i32 + self.offset.0) as usize,
            (y as i32 + self.offset.1) as usize,
        )
    }

    pub fn iter_rows(&self) -> std::slice::Iter<Vec<Tile>> {
        self.data.iter_rows()
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

impl FromStr for Map {
    type Err = Error;

    // takes line-by-line representation of wall lines
    // and derives a Map
    fn from_str(s: &str) -> Result<Self> {
        let lines: Vec<Vec<Coordinate>> = s.lines().map(parse_line).collect::<Result<_>>()?;
        let mut bounds = (0, 0);
        let upper = lines.iter().flatten().fold((0, 0), |last, curr| {
            (max(last.0, curr.x), max(last.1, curr.y))
        });
        let lower = lines.iter().flatten().fold((0, 0), |last, curr| {
            (min(last.0, curr.x), min(last.1, curr.y))
        });

        let dimensions = ((upper.0 - lower.0) as usize, (upper.1 - lower.1) as usize);

        let mut m = Self {
            data: BaseMap::new_dense(dimensions.0, dimensions.1),
            offset: (0 as i32 - lower.0 as i32, 0 as i32 - lower.1 as i32),
            bounds: ((lower.0, upper.0), (lower.1, upper.1)),
            dimensions,
        };

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

#[test]
fn test_parse_map() {
    let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    let m: Map = input.parse().expect("should parse");
}
