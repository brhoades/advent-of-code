use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Start,
    Walkable(u8),
    End,
}
use Tile::*;

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.chars().collect::<Vec<_>>()[..] {
            ['S'] => Ok(Start),
            ['E'] => Ok(End),
            [c @ 'a'..='z'] => Ok(Walkable(c as u8 - 'a' as u8)),
            _ => bail!("invalid input to parse for Tile: {:?}", s),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Start => write!(f, "S"),
            End => write!(f, "E"),
            Walkable(c) => write!(f, "{}", (*c + 'a' as u8) as char),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Map<T> {
    tiles: Vec<Vec<T>>,             // y => x => Tile
    pub dimensions: (usize, usize), // (x, y)
}

impl FromStr for Map<Tile> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut m = Self {
            tiles: s
                .split("\n")
                .filter(|l| *l != "")
                .map(|row| {
                    row.split("")
                        .filter(|c| *c != "")
                        .map(FromStr::from_str)
                        .map(|c| c.map_err(|e| anyhow!("error '{}' on parsing row: {}", e, row)))
                        .collect::<Result<Vec<_>>>()
                })
                .collect::<Result<_>>()?,
            dimensions: (0, 0),
        };
        m.dimensions = (m.tiles.get(0).unwrap().len(), m.tiles.len());

        if m.tiles.iter().any(|row| row.len() != m.dimensions.0) {
            bail!("all rows must be the same width");
        }

        Ok(m)
    }
}

impl<T: Default + Clone> Map<T> {
    pub fn new_dense(width: usize, height: usize) -> Self {
        Self {
            tiles: (0..height)
                .map(|_| {
                    let mut row = vec![];
                    row.resize(width, Default::default());
                    row
                })
                .collect::<Vec<_>>(),
            dimensions: (width, height),
        }
    }
}

impl<T> Map<T> {
    pub fn get(&self, x: usize, y: usize) -> Result<&T> {
        self.tiles.get(y).and_then(|row| row.get(x)).ok_or_else(|| {
            anyhow!(
                "map w/ dimens {:?} lacks tile at ({}, {})",
                self.dimensions,
                x,
                y
            )
        })
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut T> {
        self.tiles
            .get_mut(y)
            .and_then(|row| row.get_mut(x))
            .ok_or_else(|| {
                anyhow!(
                    "map w/ dimens {:?} lacks tile at ({}, {})",
                    self.dimensions,
                    x,
                    y
                )
            })
    }

    pub fn iter_rows(&self) -> std::slice::Iter<Vec<T>> {
        self.tiles.iter()
    }
}

impl fmt::Display for Map<bool> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.iter_rows() {
            for cell in row {
                if *cell {
                    write!(f, "T")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl fmt::Display for Map<Tile> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.iter_rows() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

// Specialization of Map<bool> which caches its score
// until mutated
#[derive(Debug, Clone)]
pub struct VisitedMap {
    map: Map<bool>,
    score: usize, // keeps track of # of tiles set
}

impl VisitedMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            map: Map::<bool>::new_dense(width, height),
            score: 0,
        }
    }

    pub fn score(&self) -> usize {
        self.score - 1 // don't count start
    }

    pub fn get(&self, x: usize, y: usize) -> Result<&bool> {
        self.map.get(x, y)
    }

    pub fn set(&mut self, x: usize, y: usize) -> Result<()> {
        *self.map.get_mut(x, y)? = true;
        self.score += 1;
        Ok(())
    }

    pub fn unset(&mut self, x: usize, y: usize) -> Result<()> {
        *self.map.get_mut(x, y)? = false;
        self.score -= 1;
        Ok(())
    }
}

impl fmt::Display for VisitedMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.map)
    }
}

// writes a map to stdout, clearning the screen, and flushes
// only once all output is ready to avoid flashes.
pub fn batch_print<T: fmt::Display>(m: &T) {
    let mut handle = io::stdout().lock();
    let s = format!("{}", m);
    handle.write_all(b"\x1B[2J\x1B[1;1H").unwrap();
    handle.write_all(s.as_bytes()).unwrap();
    handle.flush().unwrap();
}
