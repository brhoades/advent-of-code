use std::cmp::{max, min};
use std::collections::HashSet;
use std::iter::repeat;
use std::str::FromStr;

use anyhow::{bail, Error, Result};

pub fn run(input: String) -> Result<()> {
    let orders = parse_orders(&input)?;
    let mut m = Map::new();

    // println!("{}", m);
    for o in orders {
        m.execute(o);
        // println!("{}", m);
    }

    println!("visited: {}", m.visited.len());
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn as_tuple(&self) -> (&i32, &i32) {
        (&self.x, &self.y)
    }

    fn offset(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

struct Map {
    head: Coordinate,
    tail: Coordinate,
    visited: HashSet<Coordinate>,
}

impl Map {
    fn new() -> Self {
        Self {
            head: Coordinate::new(0, 0),
            tail: Coordinate::new(0, 0),
            visited: HashSet::new(),
        }
    }

    /// execute moves the head according to order.
    /// the tail and visited are updated accordingly
    fn execute(&mut self, o: Order) {
        let (dx, dy) = match o {
            Up => (0, 1),
            Right => (1, 0),
            Down => (0, -1),
            Left => (-1, 0),
        };
        self.head.offset(dx, dy);

        self.update_tail();
    }

    fn update_tail(&mut self) {
        match (self.head.x - self.tail.x, self.head.y - self.tail.y) {
            // touching, OK!
            (dx, dy) if dx.abs() <= 1 && dy.abs() <= 1 => return,
            (0, 2) => self.tail.offset(0, 1),
            (0, -2) => self.tail.offset(0, -1),
            (2, 0) => self.tail.offset(1, 0),
            (-2, 0) => self.tail.offset(-1, 0),
            (-2, -2) => self.tail.offset(-1, -1),
            (-2, 2) => self.tail.offset(-1, 1),
            (2, -2) => self.tail.offset(1, -1),
            (2, 2) => self.tail.offset(1, 1),
            (1, 2) | (2, 1) => self.tail.offset(1, 1),
            (-2, 1) | (-1, 2) => self.tail.offset(-1, 1),
            (2, -1) | (1, -2) => self.tail.offset(1, -1),
            (-2, -1) | (-1, -2) => self.tail.offset(-1, -1),
            (dx, dy) => panic!(
                "unknown case ({}, {}): {:?} => {:?}",
                dx, dy, self.head, self.tail
            ),
        }

        self.visited.insert(self.tail.clone());
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let px = max(max(self.head.x, self.tail.x), 10) + 1;
        let py = max(max(self.head.y, self.tail.y), 10) + 1;
        let nx = min(min(self.head.x, self.tail.x), 0) - 1;
        let ny = min(min(self.head.y, self.tail.y), 0) - 1;

        println!("{:?} and {:?}", ny..py, nx..px);
        // render in quadrant 1, flip bounds
        for y in (ny..py).rev() {
            for x in nx..px {
                if self.head.x == x && self.head.y == y {
                    write!(f, "H")?;
                } else if self.tail.x == x && self.tail.y == y {
                    write!(f, "T")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        std::fmt::Result::Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Order {
    Up,
    Right,
    Down,
    Left,
}
use Order::*;

impl FromStr for Order {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "U" => Ok(Up),
            "R" => Ok(Right),
            "D" => Ok(Down),
            "L" => Ok(Left),
            order => bail!("unknown order: {}", order),
        }
    }
}

/// parse_orders takes a string of newline-separated orders
/// and parses them out into the enum above.
fn parse_orders(input: &str) -> Result<Vec<Order>> {
    input
        .split("\n")
        .filter(|l| *l != "")
        .map(|l| match l.split(" ").collect::<Vec<_>>().as_slice() {
            &[dir, count] => Ok((dir.parse::<Order>()?, count.parse::<u8>()?)),
            _ => bail!("unknown line format: {}", l),
        })
        .collect::<Result<Vec<_>>>()
        .map(|orders| {
            orders
                .into_iter()
                .flat_map(|(o, cnt)| repeat(o.clone()).take(cnt as usize))
                .collect()
        })
}

#[test]
fn test_example() {
    let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    let orders = parse_orders(input).unwrap();
    let mut m = Map::new();

    for o in orders {
        m.execute(o);
    }
}
