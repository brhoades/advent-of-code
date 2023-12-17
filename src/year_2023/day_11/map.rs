use advent_of_code::{coord::Coordinate, prelude::*};
use std::{collections::HashSet, fmt};

pub type Coord = Coordinate<usize>;

#[derive(Clone, Debug)]
pub struct Galaxy(pub Coord);

pub struct Map {
    // y => x => Tile
    data: HashSet<Coord>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.data.contains(&Coord { x, y })
    }

    // expands the map, adding adding the number of empty rows provided
    // 1 doubles every empty row/col, 2 triples, etc
    pub fn expand_count(&mut self, count: usize) {
        // INSERT ROWS
        // Insert rows in reverse so we don't need to update x in galaxies_x.
        // we go.
        // While we walk rows, if there are no galaxies we grab all galaxies with >y and shift
        // them by cnt.
        for y in (0..self.height).rev() {
            if self.iter().any(|g| g.y == y) {
                continue;
            }

            // this can be inefficient and still be blazing fast since we're
            // sparesly allocated
            //
            // snapshot
            let galaxies = self.galaxies().map(|g| g.0).collect::<Vec<_>>();
            for g in galaxies.iter().filter(|g| g.y > y) {
                if !self.remove(&g) {
                    unreachable!("failed to get tile at {g:?}:\n{self}");
                }

                self.insert(Coord {
                    x: g.x,
                    y: g.y + count,
                });
            }
            self.height += count;
        }

        // INSERT COLUMNS
        // repeat for x, same strat.
        for x in (0..self.width).rev() {
            if self.iter().any(|g| g.x == x) {
                continue;
            }

            // this can be inefficient and still be blazing fast since we're
            // sparesly allocated
            //
            // snapshot
            let galaxies = self.galaxies().map(|g| g.0).collect::<Vec<_>>();
            for g in galaxies.iter().filter(|g| g.x > x) {
                if !self.remove(&g) {
                    unreachable!("failed to get tile at {g:?}:\n{self}");
                }
                self.insert(Coord {
                    x: g.x + count,
                    y: g.y,
                });
            }
            self.width += count;
        }
    }

    #[cfg(test)]
    // returns a Map which dislays with the galaxies numbered
    pub fn numbered(&self) -> NumberedMap<'_> {
        NumberedMap(&self)
    }
}

pub struct NumberedMap<'a>(&'a Map);

impl fmt::Display for NumberedMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cnt = 1;
        for y in 0..self.0.height {
            for x in 0..self.0.width {
                if self.0.get(x, y) {
                    write!(f, "{cnt}")?;
                    cnt += 1;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl std::ops::Deref for Map {
    type Target = HashSet<Coord>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// assumes retangular input
impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        let width = s
            .lines()
            .filter(|l| !l.is_empty())
            .next()
            .map(|l| l.trim().split("").filter(|c| !c.is_empty()).count())
            .unwrap_or_default();
        let height = s.lines().filter(|l| !l.is_empty()).count();
        let data = s
            .trim()
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.trim()
                    .split("")
                    .filter(|v| !v.is_empty())
                    .enumerate()
                    .filter(|(_, v)| *v == "#")
                    .map(move |(x, _)| Coord { x, y })
            })
            .flatten()
            .collect();

        Ok(Self {
            width,
            height,
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;
    use super::*;

    #[test]
    fn test_parse() {
        let _: Map = EXAMPLE_1.parse().unwrap();
    }

    #[test]
    fn test_expand() {
        let mut m: Map = EXAMPLE_1.parse().unwrap();
        println!("{m}");
        m.expand_count(1);

        println!("got:\n{m}");
        let expected = r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
        println!("expected:\n{expected}");

        assert_eq!(expected, format!("{m}").trim());
    }
}
