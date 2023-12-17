use advent_of_code::{
    neighbor_map::{Map as NMap, Neighbors, NodeData},
    prelude::*,
};
use std::{
    collections::HashMap,
    fmt,
    ops::{Deref, DerefMut},
};

pub type Steps = usize;
pub type Coords = (usize, usize);

#[derive(Debug, Clone)]
pub struct Map(NMap<Tile>);

impl Deref for Map {
    type Target = NMap<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Map {
    pub fn start(&self) -> Option<Coords> {
        self.0
            .iter()
            .find(|(_, t)| *t.value() == Tile::Start)
            .map(|tup| tup.0)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Tile {
    #[default]
    Empty,
    Vertical,
    Horizontal,
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
    Start,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Tile::*;
        // ╠
        // ╣
        // ╦
        // ╩
        // ╬
        let c = match self {
            Empty => ".",
            TopLeftCorner => "╔",
            TopRightCorner => "╗",
            BottomLeftCorner => "╚",
            BottomRightCorner => "╝",
            Vertical => "║",
            Horizontal => "═",
            Start => "S",
        };
        write!(f, "{c}")
    }
}

impl FromStr for Tile {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        use Tile::*;
        Ok(match s {
            "." => Empty,
            "F" => TopLeftCorner,
            "7" => TopRightCorner,
            "|" => Vertical,
            "-" => Horizontal,
            "J" => BottomRightCorner,
            "L" => BottomLeftCorner,
            "S" => Start,
            other => bail!("unknown character '{other}'"),
        })
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let data = s
            .trim()
            .lines()
            .map(|l| {
                l.trim()
                    .split("")
                    .filter(|s| !s.is_empty())
                    .map(FromStr::from_str)
                    .collect::<Result<Vec<Tile>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        let m = NMap::<Tile>::new(data.get(0).unwrap().len(), data.len());

        // populate m by zipping it together with ourself
        let data_source = data
            .into_iter()
            .rev()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(x, cell)| ((x, y), cell))
            });
        for (((xd, yd), mut dest), ((xs, ys), src)) in m.iter_mut().zip(data_source) {
            if xd != xs {
                bail!("when iterating the source data, expected x={xs} to be x={xd}; is the dataset square?");
            } else if ys != yd {
                bail!("when iterating the source data, expected x={ys} to be x={yd}; is the dataset square?");
            }

            dest.set(src);
        }

        // finally, clear neighbors and map them over to traversible tiles
        for (_, mut tile) in m.iter_mut() {
            let n = tile.neighbors_mut();
            // diagonals are not possible
            n.upleft = None;
            n.upright = None;
            n.downleft = None;
            n.downright = None;
        }

        Ok(Self(m))
    }
}

pub trait MainLoopNeighbors {
    // returns neighbors which are a valid part of the main loop
    fn main_loop_neighbors(&self) -> Neighbors<Tile>;
}

impl MainLoopNeighbors for NodeData<Tile> {
    fn main_loop_neighbors(&self) -> Neighbors<Tile> {
        let mut n = self.neighbors().to_owned();
        use Tile::*;
        match self.value() {
            Empty => {
                n.down = None;
                n.right = None;
            }
            TopLeftCorner => {
                n.up = None;
                n.left = None;
            }
            TopRightCorner => {
                n.up = None;
                n.right = None;
            }
            BottomLeftCorner => {
                n.down = None;
                n.left = None;
            }
            BottomRightCorner => {
                n.down = None;
                n.right = None;
            }
            Vertical => {
                n.left = None;
                n.right = None;
            }
            Horizontal => {
                n.up = None;
                n.down = None;
            }
            Start => {
                // depends on the destination tile's type and its relative
                // direction to the start.
                match n.left.as_ref().map(|nt| *nt.borrow().value()) {
                    Some(BottomLeftCorner | TopRightCorner | Horizontal) => (),
                    _ => {
                        n.left = None;
                    }
                }
                match n.right.as_ref().map(|nt| *nt.borrow().value()) {
                    Some(BottomRightCorner | TopRightCorner | Horizontal) => (),
                    _ => {
                        n.right = None;
                    }
                }
                match n.up.as_ref().map(|nt| *nt.borrow().value()) {
                    Some(
                        BottomRightCorner | BottomLeftCorner | TopLeftCorner | TopRightCorner
                        | Vertical,
                    ) => (),
                    _ => {
                        n.up = None;
                    }
                }
                match n.down.as_ref().map(|nt| *nt.borrow().value()) {
                    Some(
                        BottomRightCorner | BottomLeftCorner | TopLeftCorner | TopRightCorner
                        | Vertical,
                    ) => (),
                    _ => {
                        n.down = None;
                    }
                }
            }
        };

        n
    }
}

#[derive(Debug, Default, Clone)]
pub struct DistanceMap {
    pub data: HashMap<Coords, usize>,
    pub width: usize,
    pub height: usize,
}

impl Deref for DistanceMap {
    type Target = HashMap<Coords, usize>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for DistanceMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl fmt::Display for DistanceMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                if let Some(c) = self.data.get(&(x, y)) {
                    write!(f, "{c}")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Interior,
    Exterior,
    MainLoop(Tile),
}

impl fmt::Display for TileKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileKind::Interior => write!(f, "I")?,
            TileKind::Exterior => write!(f, "O")?,
            TileKind::MainLoop(t) => write!(f, "{t}")?,
        }

        Ok(())
    }
}

pub struct TileKindMap(HashMap<Coords, TileKind>);

impl Deref for TileKindMap {
    type Target = HashMap<Coords, TileKind>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TileKindMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<(Coords, TileKind)> for TileKindMap {
    fn from_iter<T: IntoIterator<Item = (Coords, TileKind)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl fmt::Display for TileKindMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height = self
            .keys()
            .map(|(_, y)| y)
            .max_by(|y1, y2| y1.cmp(y2))
            .copied()
            .unwrap_or_default();
        let width = self
            .keys()
            .map(|(x, _)| x)
            .max_by(|x1, x2| x1.cmp(x2))
            .copied()
            .unwrap_or_default();
        for y in (0..=height).rev() {
            for x in 0..=width {
                if let Some(c) = self.0.get(&(x, y)) {
                    write!(f, "{c}")?;
                } else {
                    unreachable!("incomplete tile map");
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{super::tests::*, *};

    #[test]
    fn test_parse_example_1() {
        let m: Map = EXAMPLE_1.parse().unwrap();
        println!("{m}");

        assert_eq!(Tile::Empty, *m.get(0, 0).unwrap().value());
        assert_eq!(Tile::BottomLeftCorner, *m.get(1, 1).unwrap().value());
        assert_eq!(Tile::Empty, *m.get(2, 2).unwrap().value());
        assert_eq!(Tile::Empty, *m.get(0, 4).unwrap().value());
        assert_eq!(Tile::Empty, *m.get(4, 0).unwrap().value());
        assert_eq!(Tile::TopRightCorner, *m.get(3, 3).unwrap().value());
        assert_eq!(Tile::Empty, *m.get(4, 4).unwrap().value());
    }

    #[test]
    fn test_parse_example_2() {
        let m: Map = EXAMPLE_2.parse().unwrap();
        println!("{m}");

        assert_eq!(Tile::BottomLeftCorner, *m.get(0, 0).unwrap().value());
        assert_eq!(Tile::TopLeftCorner, *m.get(1, 1).unwrap().value());
        assert_eq!(Tile::Empty, *m.get(2, 2).unwrap().value());
        assert_eq!(Tile::Empty, *m.get(0, 4).unwrap().value());
        assert_eq!(Tile::Empty, *m.get(4, 0).unwrap().value());
        assert_eq!(Tile::Vertical, *m.get(3, 3).unwrap().value());
        assert_eq!(Tile::Empty, *m.get(4, 4).unwrap().value());
    }

    #[test]
    fn test_start() {
        let m1: Map = EXAMPLE_1.parse().unwrap();
        let m2: Map = EXAMPLE_2.parse().unwrap();

        assert_eq!((1, 3), m1.start().unwrap());
        assert_eq!((0, 2), m2.start().unwrap());
    }
}
