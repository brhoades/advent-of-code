use std::cmp::{max, min};
use std::collections::HashSet;
use std::iter::repeat;
use std::str::FromStr;

use anyhow::{bail, Error, Result};

pub fn run(input: String) -> Result<()> {
    let orders = parse_orders(&input)?;
    let mut m = Map::new(1);

    // println!("{}", m);
    for o in &orders {
        m.execute(o.clone());
        // println!("{}", m);
    }

    println!("visited: {}", m.visited.len());

    let mut m = Map::new(9);

    // println!("{}", m);
    for o in orders {
        m.execute(o);
        // println!("{}", m);
    }

    println!("visited, pt 2: {}", m.visited.len());
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

    fn offset(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

struct Map {
    knots: Vec<Coordinate>,
    visited: HashSet<Coordinate>,
}

impl Map {
    fn new(knots: usize) -> Self {
        if knots < 1 {
            panic!("needs more knots");
        }

        Self {
            knots: repeat(Coordinate::new(0, 0)).take(knots + 1).collect(),
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
        // the head knot is first, we then update
        // all remaining knots to follow.
        self.knots.get_mut(0).unwrap().offset(dx, dy);

        for (headi, taili) in (0..self.knots.len()).zip(1..self.knots.len()) {
            let head = self.knots.get(headi).unwrap();
            let tail = self.knots.get(taili).unwrap();

            let newtail = self.update_pair(head, tail.clone());
            *self.knots.get_mut(taili).unwrap() = newtail;
        }

        self.visited.insert(self.knots.last().unwrap().clone());
    }

    // modifies the psat tail given a moved head
    fn update_pair(&self, head: &Coordinate, mut tail: Coordinate) -> Coordinate {
        match (head.x - tail.x, head.y - tail.y) {
            // touching, OK!
            (dx, dy) if dx.abs() <= 1 && dy.abs() <= 1 => return tail,
            (0, 2) => tail.offset(0, 1),
            (0, -2) => tail.offset(0, -1),
            (2, 0) => tail.offset(1, 0),
            (-2, 0) => tail.offset(-1, 0),
            (-2, -2) => tail.offset(-1, -1),
            (-2, 2) => tail.offset(-1, 1),
            (2, -2) => tail.offset(1, -1),
            (2, 2) => tail.offset(1, 1),
            (1, 2) | (2, 1) => tail.offset(1, 1),
            (-2, 1) | (-1, 2) => tail.offset(-1, 1),
            (2, -1) | (1, -2) => tail.offset(1, -1),
            (-2, -1) | (-1, -2) => tail.offset(-1, -1),
            (dx, dy) => panic!("unknown case ({}, {}): {:?} => {:?}", dx, dy, head, tail),
        }

        tail
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut px = 10;
        let mut py = 10;
        let mut nx = 0;
        let mut ny = 0;

        for knot in &self.knots {
            px = max(knot.x, px) + 1;
            py = max(knot.y, py) + 1;
            nx = min(knot.x, nx) - 1;
            ny = min(knot.y, ny) - 1;
        }

        let mut output = vec![];
        output.resize((px - nx) as usize, vec![]);
        for row in &mut output {
            row.resize((py - ny) as usize, ".".to_string());
        }

        // render in quadrant 1, flip bounds
        for y in (ny..py).rev() {
            for x in nx..px {
                for (i, knot) in self.knots.iter().enumerate() {
                    let c = output
                        .get_mut((x - nx) as usize)
                        .unwrap()
                        .get_mut((y - ny) as usize)
                        .unwrap();

                    if knot.x == x && knot.y == y {
                        *c = i.to_string();
                    }
                }
            }
        }

        for row in output {
            for c in row {
                write!(f, "{}", c)?;
            }
            write!(f, "{}", "\n")?;
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
    let mut m = Map::new(1);

    for o in orders {
        m.execute(o);
    }

    assert_eq!(13, m.visited.len());
}

#[test]
fn test_example_two() {
    let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    let orders = parse_orders(input).unwrap();
    let mut m = Map::new(9);

    println!("{}", m);
    for o in orders {
        m.execute(o);
        println!("{}", m);
    }

    assert_eq!(36, m.visited.len());
}
