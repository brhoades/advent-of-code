use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    let m: Map = input.parse()?;

    println!(
        "part 1, row y=2M has {} squares covered by sensors",
        m.positions_without_beacon(2_000_000)
    );

    println!("===== part 2 =====");
    let (x, y) = m
        .find_distress_signal(0, 0, 4_000_000, 4_000_000)
        .expect("failed to find signal");
    println!(
        "distress beacon @ ({}, {}) with frequency {}",
        x,
        y,
        x * 4000000 + y
    );

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Sensor {
    x: i64,
    y: i64,
    range: i64,

    beacon: (i64, i64),
}

impl Sensor {
    #[inline]
    pub fn distance(&self, x: i64, y: i64) -> i64 {
        (x - self.x).abs() + (y - self.y).abs()
    }

    /// returns if this sensor cover the provided coordinate
    #[inline]
    pub fn cover(&self, x: i64, y: i64) -> bool {
        self.distance(x, y) <= self.range
    }

    /// perimeter_iter returns an interator which walks the perimeter
    /// of the sensor's range at an offset. offset=0 walks the perimeter,
    /// offset=1 is one additional step outside of that, etc
    pub fn perimeter_iter(&self, offset: i64) -> PerimeterIterator {
        PerimeterIterator {
            rem_steps: vec![(1, -1), (1, 1), (-1, 1)],

            cur_step: (-1, -1),
            cur_i: 0,

            offset: self.range + offset,
            source: (self.x, self.y),
        }
    }
}

impl FromStr for Sensor {
    type Err = Error;

    /// takes line-by-line representation of wall lines
    /// and derives a Map
    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split(" ").collect::<Vec<_>>();
        let cleanup = |s: &str| s.trim_matches(|c| ",:x=y".find(c).is_some()).to_string();

        let (sx, sy, bx, by): (i64, i64, i64, i64) =
            if let &[_, _, sx, sy, _, _, _, _, bx, by] = parts.as_slice() {
                (
                    cleanup(sx).parse()?,
                    cleanup(sy).parse()?,
                    cleanup(bx).parse()?,
                    cleanup(by).parse()?,
                )
            } else {
                bail!("unknown format for line: {}", s)
            };

        let range = (bx - sx).abs() + (by - sy).abs();

        Ok(Sensor {
            x: sx,
            y: sy,
            range,
            beacon: (bx, by),
        })
    }
}

struct Map {
    dimensions: (i64, i64),
    left: (i64, i64), // left side of drawn graphs, from S or B
    sensors: Vec<Sensor>,
}

impl Map {
    /// returns the number of positions on a given line (y=#) for which a beacon
    /// cannot be present.
    pub fn positions_without_beacon(&self, y: i64) -> usize {
        let mut cnt = 0;
        let nodes: HashSet<(i64, i64)> = self
            .sensors
            .iter()
            .flat_map(|s| vec![(s.x, s.y), s.beacon.clone()])
            .collect();

        for x in self.left.0..self.dimensions.0 {
            if let Some(_) = nodes.get(&(x, y)) {
                continue;
            }

            if self.sensors.iter().any(|s| s.cover(x, y)) {
                cnt += 1;
            }
        }

        cnt
    }

    /// find_distress_signal looks at all the points on the edge of sensors perimeter in the range
    /// and finds their intersection since there's just one.
    pub fn find_distress_signal(&self, lx: i64, ly: i64, mx: i64, my: i64) -> Option<(i64, i64)> {
        None
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // pad base on number of digits and add two for negatives with some minor padding from data
        let ypad = max(
            (self.left.1 as f64).log10() as usize,
            (self.dimensions.1 as f64).log10() as usize,
        ) + 2;
        let xpad = max(
            (self.left.0 as f64).log10() as usize,
            (self.dimensions.0 as f64).log10() as usize,
        ) + 2;

        // render X position labels
        for y in self.left.1..self.dimensions.1 {
            if y == self.left.1 {
                for i in 0..=xpad {
                    for _ in 0..=ypad {
                        write!(f, " ")?;
                    }

                    if i >= xpad {
                        write!(f, " ")?;
                    } else {
                        for x in self.left.0..self.dimensions.0 {
                            if x % 5 == 0 || x == self.dimensions.0 - 1 || x == self.left.0 {
                                write!(
                                    f,
                                    "{}",
                                    format!("{: >width$} ", x, width = xpad).get(i..=i).unwrap()
                                )?;
                            } else {
                                write!(f, " ")?;
                            }
                        }
                    }

                    write!(f, "\n")?;
                }
            }

            // render Y position labels
            write!(f, "{: >width$} ", y, width = ypad)?;
            for x in self.left.0..self.dimensions.0 {
                let mut drawn = false;
                for s in &self.sensors {
                    if s.beacon.0 == x && s.beacon.1 == y {
                        write!(f, "B")?;
                        drawn = true;
                        break;
                    }
                    if s.x == x && s.y == y {
                        write!(f, "S")?;
                        drawn = true;
                        break;
                    }

                    if s.cover(x, y) {
                        write!(f, "#")?;
                        drawn = true;
                        break;
                    }
                }

                if !drawn {
                    write!(f, ".")?;
                }
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
        let sensors = s
            .lines()
            .map(FromStr::from_str)
            .collect::<Result<Vec<Sensor>>>()?;
        let dimensions = (
            sensors
                .iter()
                .map(|s| max(s.x + s.range + 1, s.beacon.0))
                .max()
                .unwrap(),
            sensors
                .iter()
                .map(|s| max(s.y + s.range + 1, s.beacon.1))
                .max()
                .unwrap(),
        );
        let left = (
            sensors
                .iter()
                .map(|s| min(s.x - s.range, s.beacon.0))
                .min()
                .unwrap(),
            sensors
                .iter()
                .map(|s| min(s.y - s.range, s.beacon.1))
                .min()
                .unwrap(),
        );

        Ok(Map {
            sensors,
            dimensions,
            left,
        })
    }
}

#[derive(Debug)]
struct PerimeterIterator {
    // the steps remaining to walk
    rem_steps: Vec<(i64, i64)>,

    cur_step: (i64, i64),
    cur_i: i64,

    offset: i64, // how far out we are iterating from the source
    source: (i64, i64),
}

impl PerimeterIterator {
    // returns the start point to step from given the current step direction
    fn start_by_offset(&self) -> (i64, i64) {
        match self.cur_step {
            (-1, -1) => (self.offset + self.source.0, self.source.1),
            (-1, 1) => (self.source.0, self.source.1 - self.offset),
            (1, 1) => (self.source.0 - self.offset, self.source.1),
            (1, -1) => (self.source.0, self.source.1 + self.offset),
            other => unimplemented!("should not have step size of {:?}", other),
        }
    }
}

impl Iterator for PerimeterIterator {
    type Item = (i64, i64);

    // next iterates over cur_step, incrementing cur_i
    // moves on to the next step when cur_i * cur_step
    //
    // next returns the offset + source vertices first on initial
    // use of next. Iteration for a step finishes when the next
    // vertex is hit.
    fn next(&mut self) -> Option<Self::Item> {
        let edgex = self.offset + self.source.0;
        let edgey = self.offset + self.source.1;
        let (srcx, srcy) = self.start_by_offset();

        let x = self.cur_step.0 * self.cur_i + srcx;
        let y = self.cur_step.1 * self.cur_i + srcy;
        if self.cur_i != 0 && (x == self.source.0 || y == self.source.1) {
            // we're on a beacon vertex: use next step
            if self.rem_steps.len() == 0 {
                return None;
            }

            self.cur_step = self.rem_steps.pop().unwrap();
            self.cur_i = 0;
            return self.next();
        } else {
            self.cur_i += 1;
        }

        Some((x, y))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_pt1() {
        let inputs = "Sensor at x=5, y=5: closest beacon is at x=3, y=3";
        let m: Map = inputs.parse().unwrap();
        println!("{}", m);

        for x in 0..=10 {
            for y in 0..=10 {
                assert_eq!(
                    (x - 5 as i64).abs() + (y - 5 as i64).abs() <= 4,
                    m.sensors.first().unwrap().cover(x, y),
                );
            }
        }
    }

    #[test]
    fn test_ex_pt1() {
        let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
        let sensors: Map = input.parse().unwrap();

        println!("{}", sensors);
        assert_eq!(26, sensors.positions_without_beacon(10));
    }

    #[test]
    fn test_perimeter_iter_diamond_zero_offset() {
        for size in 2..=10 {
            let s = Sensor {
                x: size,
                y: size,

                range: size,

                beacon: (0, size),
            };
            let mut visited = HashSet::new();
            let mut i = 0;

            for (x, y) in s.perimeter_iter(0) {
                println!("({}, {})", x, y);
                assert!(!visited.contains(&(x, y)));
                visited.insert((x, y));
                assert_eq!(s.range, s.distance(x, y));

                i += 1;
                if i > 2 * s.range * 4 {
                    panic!();
                }
            }

            assert_eq!(s.range * 4, visited.len() as i64);
        }
    }

    #[test]
    fn test_perimeter_iter_square_zero_offset() {
        for size in 2..=10 {
            let s = Sensor {
                x: size,
                y: size,

                range: size * 2,

                beacon: (0, 0),
            };
            let mut visited = HashSet::new();
            let mut i = 0;

            for (x, y) in s.perimeter_iter(0) {
                println!("({}, {})", x, y);
                assert!(!visited.contains(&(x, y)));
                visited.insert((x, y));
                assert_eq!(s.range, s.distance(x, y));

                i += 1;
                if i > s.range * 4 {
                    panic!();
                }
            }

            assert_eq!(s.range * 4, visited.len() as i64);
        }
    }

    #[test]
    fn test_perimeter_iter_diamond_increasing_offset() {
        let size = 5;

        for offset in 0..=10 {
            let s = Sensor {
                x: size,
                y: size,

                range: size,

                beacon: (0, size),
            };
            let side_len = s.range + offset;
            let mut visited = HashSet::new();
            let mut i = 0;

            for (x, y) in s.perimeter_iter(offset) {
                println!("({}, {})", x, y);
                assert!(!visited.contains(&(x, y)));
                visited.insert((x, y));
                assert_eq!(side_len, s.distance(x, y));

                i += 1;
                if i > side_len * 4 {
                    panic!();
                }
            }

            assert_eq!(side_len * 4, visited.len() as i64);
            println!("===");
        }
    }
}

/*
0   .....#.....
1   ....###....
2   ...#####...
3   ..#######..
4   .#########.
5   B####S#####
6   .#########.
7   ..#######..
8   ...#####...
9   ....###....
10  .....#.....


*/
