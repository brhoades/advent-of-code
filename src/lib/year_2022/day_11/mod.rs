use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};
use indicatif::{ProgressBar, ProgressStyle};

pub fn run(input: String) -> Result<()> {
    let mut monkies = parse_monkies(&input)?;

    for _ in 0..20 {
        monkey_a_round(&mut monkies, Some(3), None);
    }

    println!(
        "{}",
        monkies
            .iter()
            .map(|m| format!("{}", m))
            .fold("".to_string(), |acc, s| acc + "\n" + &s)
    );

    println!("\n");

    println!(
        "{}",
        monkies
            .iter()
            .map(|m| format!("Monkey {} inspected items {} times", m.index, m.inspections))
            .fold("".to_string(), |acc, s| acc + "\n" + &s),
    );

    let mut inspections = monkies.iter().map(|m| m.inspections).collect::<Vec<_>>();
    inspections.sort();

    println!("\n");

    println!(
        "Monkey business level: {}",
        inspections.iter().rev().take(2).product::<usize>(),
    );

    println!("=================\nPart 2\n=================");
    let mut monkies = parse_monkies(&input)?;
    let worry_modulus = monkies.iter().map(|m| m.test_divisor).product::<u32>();
    let pb = ProgressBar::new(10000);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>5}/{len:5} {msg}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    for i in 0..10000 {
        pb.set_position(i);
        monkey_a_round(&mut monkies, None, Some(worry_modulus));
    }
    pb.finish();

    let mut inspections = monkies.iter().map(|m| m.inspections).collect::<Vec<_>>();
    inspections.sort();
    println!(
        "Monkey business level: {}",
        inspections.iter().rev().take(2).product::<usize>(),
    );

    Ok(())
}

/// do a round of item calculations with passed monkies
fn monkey_a_round(
    monkies: &mut Vec<Monkey>,
    worry_divisor: Option<u32>,
    worry_modulus: Option<u32>,
) {
    for i in 0..monkies.len() {
        let (dec, test, op, items) = {
            let m = monkies.get_mut(i).unwrap();
            m.inspections += m.items.len();
            (
                m.test_decision,
                m.test_divisor,
                m.op.clone(),
                m.items.drain(..).collect::<Vec<_>>(), // avoid double mutable borrow
            )
        };

        for item in items {
            let mut item = op.process(item);
            if let Some(div) = worry_divisor {
                item /= div as u128
            } else if let Some(modulus) = worry_modulus {
                item %= modulus as u128
            }
            let dest = if item % test as u128 == 0 {
                dec.0
            } else {
                dec.1
            };

            monkies
                .get_mut(dest)
                .ok_or_else(|| anyhow!("monkey {} out of range", dest))
                .unwrap()
                .items
                .push(item);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operand {
    Num(u32),
    Old,
}

impl FromStr for Operand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "old" => Ok(Operand::Old),
            s => Ok(Operand::Num(s.parse()?)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Op {
    Add(Operand, Operand),
    Subtract(Operand, Operand),
    Multiply(Operand, Operand),
}

impl Op {
    fn process(&self, old: u128) -> u128 {
        use Op::*;
        use Operand::*;

        match self {
            Add(Old, Num(n)) | Add(Num(n), Old) => old + *n as u128, //
            Subtract(Old, Num(n)) => old - *n as u128,
            Subtract(Num(n), Old) => *n as u128 - old,
            Multiply(Old, Num(n)) | Multiply(Num(n), Old) => old * *n as u128,
            Multiply(Old, Old) => old.pow(2),
            other => unimplemented!("{:?}", other),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Monkey {
    items: Vec<u128>,
    op: Op,
    test_divisor: u32,
    // if w % test_divisor == 0 then monkey_(test_decision.0) else monkey_(test_decision.1)
    test_decision: (usize, usize),
    index: usize,
    inspections: usize,
}

fn parse_monkey(index: usize, monkey: &str) -> Result<Monkey> {
    let lines = monkey
        .lines()
        .map(|l| l.split(' ').filter(|w| !w.is_empty()).collect::<Vec<_>>())
        .skip(1)
        .collect::<Vec<_>>();

    if lines.len() != 5 {
        bail!(
            "expected exactly 6 lines to parse a monkey, got {}",
            lines.len()
        );
    }

    let mut lines = lines.into_iter();

    let items = match lines.next().unwrap().as_slice() {
        ["Starting", "items:", rem @ ..] => {
            let rem = rem.join("");
            rem.split(',')
                .map(|i| i.parse::<u128>())
                .collect::<std::result::Result<Vec<_>, _>>()
        }
        line => bail!("cannot parse monkey line. expected items, got: {:?}", line),
    }?;

    let op = match lines.next().unwrap().as_slice() {
        ["Operation:", "new", "=", oper1, op, oper2] => {
            let oper1 = oper1
                .parse()
                .map_err(|e| anyhow!("failed to parse left operand '{}': {}", oper1, e))?;
            let oper2 = oper2
                .parse()
                .map_err(|e| anyhow!("failed to parse right operand '{}': {}", oper2, e))?;

            match *op {
                "+" => Op::Add(oper1, oper2),
                "-" => Op::Subtract(oper1, oper2),
                "*" => Op::Multiply(oper1, oper2),
                op => bail!("unkown op: {}", op),
            }
        }
        line => bail!(
            "cannot parse monkey line. expected operation, got: {:?}",
            line
        ),
    };

    let test_divisor = match lines.next().unwrap().as_slice() {
        ["Test:", "divisible", "by", divisor] => divisor.parse()?,
        line => bail!(
            "cannot parse monkey line. expected operation, got: {:?}",
            line
        ),
    };

    let mut test_decision = vec![];
    for l in lines {
        match l.as_slice() {
            ["If", "true:", "throw", "to", "monkey", num] => test_decision.push(num.parse()?),
            ["If", "false:", "throw", "to", "monkey", num] => test_decision.push(num.parse()?),
            line => bail!(
                "cannot parse monkey line. expected conditional, got: {:?}",
                line
            ),
        }
    }

    let last = test_decision.pop().unwrap();
    Ok(Monkey {
        items,
        op,
        test_divisor,
        test_decision: (test_decision.pop().unwrap(), last),
        index,
        inspections: 0,
    })
}

fn parse_monkies(input: &str) -> Result<Vec<Monkey>> {
    input
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, m)| parse_monkey(i, m).map_err(|e| anyhow!("on monkey {}: {}", i, e)))
        .collect::<Result<Vec<_>>>()
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Monkey {}: {}",
            self.index,
            self.items
                .iter()
                .fold("".to_string(), |acc, s| if acc.is_empty() {
                    s.to_string()
                } else {
                    acc + ", " + &s.to_string()
                })
        )
    }
}

#[test]
fn test_monkey_example() {
    let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    let mut monkies = parse_monkies(input).unwrap();

    let m = monkies.get(0).unwrap();
    assert_eq!(vec![79, 98], m.items);
    assert_eq!(Op::Multiply(Operand::Old, Operand::Num(19)), m.op);
    assert_eq!(23, m.test_divisor);
    assert_eq!(2, m.test_decision.0);
    assert_eq!(3, m.test_decision.1);

    monkey_a_round(&mut monkies, Some(3), None);
    assert_monkey_items_eq(
        vec![
            vec![20, 23, 27, 26],
            vec![2080, 25, 167, 207, 401, 1046],
            vec![],
            vec![],
        ],
        &monkies,
    );

    for _ in 0..19 {
        monkey_a_round(&mut monkies, Some(3), None);
    }

    assert_monkey_items_eq(
        vec![
            vec![10, 12, 14, 26, 34],
            vec![245, 93, 53, 199, 115],
            vec![],
            vec![],
        ],
        &monkies,
    );

    assert_eq!(
        vec![101, 95, 7, 105],
        monkies.iter().map(|m| m.inspections).collect::<Vec<_>>()
    );

    let mut monkies = parse_monkies(input).unwrap();
    let worry_modulus = monkies.iter().map(|m| m.test_divisor).product::<u32>();

    for _ in 0..10000 {
        monkey_a_round(&mut monkies, None, Some(worry_modulus));
    }

    assert_eq!(
        vec![52166, 47830, 1938, 52013],
        monkies.iter().map(|m| m.inspections).collect::<Vec<_>>()
    );
}

#[cfg(test)]
fn assert_monkey_items_eq(expected: Vec<Vec<u32>>, monkies: &Vec<Monkey>) {
    assert_eq!(expected.len(), monkies.len());
    for (monkey, expected_items) in monkies.iter().zip(expected.iter()) {
        assert_eq!(
            *expected_items,
            monkey.items.iter().map(|i| *i as u32).collect::<Vec<_>>(),
            "monkey {} had different items than expected\nAll monkies:\n{}",
            monkey.index,
            monkies
                .iter()
                .map(|m| format!("{}", m))
                .fold("".to_string(), |acc, s| acc + "\n" + &s),
        );
    }
}
