mod map;
use map::*;

use itertools::Itertools;

use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    let mut m: Map = input.parse()?;
    m.expand_count(1);

    println!("minimum distance sum, doubled: {}", m.min_distance_sum());

    let mut m: Map = input.parse()?;
    m.expand_count(1_000_000 - 1);

    println!("minimum distance sum, 1MMx: {}", m.min_distance_sum());
    Ok(())
}

trait ShortestPath {
    fn shortest_path(&self, from: Coord, to: Coord) -> usize;
}

struct Brute {}

impl ShortestPath for Brute {
    fn shortest_path(&self, mut from: Coord, to: Coord) -> usize {
        let mut points = vec![];

        while from != to {
            let dx = to.x as i64 - from.x as i64;
            let dy = to.y as i64 - from.y as i64;

            let new = if dx.abs() >= dy.abs() {
                if dx > 0 {
                    // go right
                    (from.x + 1, from.y)
                } else {
                    // go left
                    (from.x - 1, from.y)
                }
            } else if dy > 0 {
                // go up
                (from.x, from.y + 1)
            } else {
                (from.x, from.y - 1)
            };
            from = Coord { x: new.0, y: new.1 };
            points.push(from.clone());
        }

        points.len()
    }
}

impl Map {
    // returns galaxies in order from top left to bottom right, row by row.
    fn galaxies(&self) -> impl Iterator<Item = Galaxy> + '_ {
        self.iter().cloned().map(Galaxy).sorted_by(|l, r| {
            let o = l.0.y.cmp(&r.0.y);
            if o == std::cmp::Ordering::Equal {
                l.0.x.cmp(&r.0.x)
            } else {
                o
            }
        })
    }

    fn min_distance_sum(&self) -> usize {
        self.galaxies()
            .combinations(2)
            .map(|mut combos| {
                let rg = combos.pop().unwrap();
                let lg = combos.pop().unwrap();
                (lg.0, rg.0)
            })
            .map(|(lg, rg)| lg.l1_norm(&rg))
            .sum()
    }
}

trait L1Norm {
    fn l1_norm(&self, other: &Self) -> usize;
}

impl L1Norm for Coord {
    fn l1_norm(&self, other: &Self) -> usize {
        (other.x as isize - self.x as isize).unsigned_abs()
            + (other.y as isize - self.y as isize).unsigned_abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE_1: &str = r"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

    #[test]
    fn test_distance() {
        let b = Brute {};

        // 1 -> 2
        assert_eq!(
            6,
            b.shortest_path(Coord { x: 4, y: 11 }, Coord { x: 9, y: 10 })
        );

        // 9 -> 8
        assert_eq!(
            5,
            b.shortest_path(Coord { x: 5, y: 0 }, Coord { x: 0, y: 0 })
        );
    }

    #[test]
    fn test_distance_sum() {
        let mut m: Map = EXAMPLE_1.parse().unwrap();
        m.expand_count(1);

        println!("{}", m.numbered());

        let galaxies = m.galaxies().map(|g| g.0).collect::<Vec<_>>();

        assert_eq!(9, galaxies.len());
        assert_eq!(9, galaxies[4].l1_norm(&galaxies[8]));
        assert_eq!(15, galaxies[0].l1_norm(&galaxies[6]));
        assert_eq!(17, galaxies[2].l1_norm(&galaxies[5]));

        assert_eq!(374, m.min_distance_sum());
    }
}
