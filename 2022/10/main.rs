use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};

const MEASURED_POINTS: [usize; 6] = [20, 60, 100, 140, 180, 220];

pub fn run(input: String) -> Result<()> {
    let ops = parse_ops(input).unwrap();
    let c = process_ops(ops);

    println!(
        "signal strength sum: {}",
        MEASURED_POINTS
            .iter()
            .map(|cycle| c.get_value_at_cycle(*cycle).map(|v| v * *cycle as i32))
            .try_fold(0, |acc, c| c.map(|c| c + acc))?,
    );

    for row in c.display {
        for c in row {
            if c {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    Ok(())
}

#[derive(Debug, Clone)]
enum Op {
    Addx(i32),
    Noop,
}

use Op::*;

impl Op {
    fn cycles(&self) -> u32 {
        match self {
            Addx(_) => 2,
            Noop => 1,
        }
    }
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split(' ').collect::<Vec<_>>().as_slice() {
            ["addx", num] => Ok(Addx(num.parse()?)),
            ["noop"] => Ok(Noop),
            s => bail!("unknown op line: {:?}", s),
        }
    }
}

fn parse_ops<S: std::borrow::Borrow<str>>(input: S) -> Result<Vec<Op>> {
    input
        .borrow()
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(FromStr::from_str)
        .collect()
}

fn process_ops(ops: Vec<Op>) -> Computer {
    let mut c = Computer {
        x: 1,
        c: 0,
        history: HashMap::new(),
        display: vec![],
    };
    for op in ops {
        for _ in 0..op.cycles() {
            c.tick();
        }

        match op {
            Addx(x) => {
                c.x += x;
            }
            Noop => (),
        }
    }

    c
}

#[derive(Debug)]
struct Computer {
    x: i32,
    c: usize,
    // pixels drawn are true
    display: Vec<[bool; 40]>,
    history: HashMap<usize, i32>,
}

impl Computer {
    fn get_value_at_cycle(&self, cycle: usize) -> Result<&i32> {
        self.history
            .get(&cycle)
            .ok_or_else(|| anyhow!("untracked cycle: {}", cycle))
    }

    fn tick(&mut self) {
        // pos of center of current pixel being drawn
        let x = self.c % 40;
        if (x as i32 - self.x).abs() <= 1 {
            let row = self.c / 40;
            if row >= self.display.len() {
                self.display.resize(row + 1, [false; 40]);
            }
            self.display
                .get_mut(row)
                .ok_or_else(|| anyhow!("failed to get row {}", row))
                .unwrap()[x] = true;
        }

        self.c += 1;

        if should_sample(&self.c) {
            self.history.insert(self.c, self.x);
        }
    }
}

fn should_sample(c: &usize) -> bool {
    *c >= 20 && *c <= 220 && (*c - 20) % 40 == 0
}

#[test]
fn test_example_1() {
    let input = r#"addx 15;addx -11;addx 6;addx -3;addx 5;addx -1;addx -8;addx 13;addx 4;noop;addx -1;addx 5;addx -1;addx 5;addx -1;addx 5;addx -1;addx 5;addx -1;addx -35;addx 1;addx 24;addx -19;addx 1;addx 16;addx -11;noop;noop;addx 21;addx -15;noop;noop;addx -3;addx 9;addx 1;addx -3;addx 8;addx 1;addx 5;noop;noop;noop;noop;noop;addx -36;noop;addx 1;addx 7;noop;noop;noop;addx 2;addx 6;noop;noop;noop;noop;noop;addx 1;noop;noop;addx 7;addx 1;noop;addx -13;addx 13;addx 7;noop;addx 1;addx -33;noop;noop;noop;addx 2;noop;noop;noop;addx 8;noop;addx -1;addx 2;addx 1;noop;addx 17;addx -9;addx 1;addx 1;addx -3;addx 11;noop;noop;addx 1;noop;addx 1;noop;noop;addx -13;addx -19;addx 1;addx 3;addx 26;addx -30;addx 12;addx -1;addx 3;addx 1;noop;noop;noop;addx -9;addx 18;addx 1;addx 2;noop;noop;addx 9;noop;noop;noop;addx -1;addx 2;addx -37;addx 1;addx 3;noop;addx 15;addx -21;addx 22;addx -6;addx 1;noop;addx 2;addx 1;noop;addx -10;noop;noop;addx 20;addx 1;addx 2;addx 2;addx -6;addx -11;noop;noop;noop"#.split(';').fold("".to_string(), |acc, l| acc.to_string() + "\n" + l);
    let pts = vec![
        (20, 21),
        (60, 19),
        (100, 18),
        (140, 21),
        (180, 16),
        (220, 18),
    ];

    let ops = parse_ops(input).unwrap();
    let c = process_ops(ops);

    for (cycle, res) in pts {
        assert_eq!(&res, c.get_value_at_cycle(cycle).unwrap(), "at {}", cycle);
    }

    println!(
        "{:?}\t{:?}",
        vec![20, 60, 100, 140, 180, 220]
            .into_iter()
            .map(|cycle| c.get_value_at_cycle(cycle))
            .collect::<Result<Vec<_>>>()
            .unwrap(),
        vec![20, 60, 100, 140, 180, 220]
            .into_iter()
            .map(|cycle| cycle as i32 * c.get_value_at_cycle(cycle).unwrap())
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        13140,
        vec![20, 60, 100, 140, 180, 220]
            .into_iter()
            .map(|cycle| cycle as i32 * c.get_value_at_cycle(cycle).unwrap())
            .sum::<i32>()
    );
}
