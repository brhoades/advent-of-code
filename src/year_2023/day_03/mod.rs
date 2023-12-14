use std::collections::{HashMap, HashSet};

use advent_of_code::{neighbor_map::*, prelude::*};

pub fn run(input: String) -> Result<()> {
    let map = parse_map(&input)?;
    println!("sum of part numbers: {}", map_part_number_sum(&map)?);
    println!("sum of gear ratios: {}", map_gear_ratio_sum(&map)?);
    Ok(())
}

fn map_part_number_sum(m: &Map<Tile>) -> Result<u64> {
    m.part_numbers().map(|parts| parts.into_iter().sum())
}

fn map_gear_ratio_sum(m: &Map<Tile>) -> Result<u64> {
    let stars = m.iter().filter_map(|(_, nd)| match nd.value() {
        Tile::Symbol('*') => Some(nd),
        _ => None,
    });

    let mut sum = 0;
    for n in stars {
        // pull out all the numbers and jam them into a hashmap
        // keyed on their start position to their value. This ensures
        // uniqueness.
        let numbers = n
            .neighbors()
            .iter()
            .map(|n| n.borrow())
            .filter_map(|nd| match nd.value() {
                Tile::Number { start, number, .. } => Some((*start, *number)),
                _ => None,
            })
            .collect::<HashMap<_, _>>();

        if numbers.len() == 2 {
            sum += numbers.into_values().product::<u64>();
        }
    }

    Ok(sum)
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub enum Tile {
    #[default]
    Empty,
    Symbol(char),
    Number {
        number: u64,           // the whole number when parsed with adjacent tiles
        digit: usize,          // the specific digit displayed on this title
        start: (usize, usize), // where the number starts, for uniqueness checks
    },
}

impl Tile {
    fn is_symbol(&self) -> bool {
        matches!(self, Tile::Symbol(_))
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Tile::Empty => write!(f, "."),
            Tile::Symbol(c) => write!(f, "{c}"),
            Tile::Number { number, digit, .. } => {
                write!(f, "{}", number.to_string().chars().nth(*digit).unwrap())
            }
        }
    }
}

// assumes a uniform input with the same number of chars
// on every line. Whitespace in input is removed.
fn parse_map(input: &str) -> Result<Map<Tile>> {
    let input: String = input.chars().filter(|c| *c != ' ' && *c != '\t').collect();
    let m = Map::new(input.lines().next().unwrap().len(), input.lines().count());

    let mut lines = vec![];
    for (y, line) in input.lines().rev().enumerate() {
        let mut row = vec![];
        let mut num_stack = vec![];
        for c in line.chars() {
            // accumulate numbers in our stack as we see them.
            // we will later parse the stack and backfill tiles for each number
            match c {
                '0'..='9' => num_stack.push(c),
                '.' => {
                    collapse_stack(&mut row, &mut num_stack, y)?;
                    row.push(Tile::Empty);
                }
                c if !c.is_ascii_alphanumeric() => {
                    collapse_stack(&mut row, &mut num_stack, y)?;
                    row.push(Tile::Symbol(c));
                }
                c => bail!("unexpected character in input '{c}'"),
            }
        }

        collapse_stack(&mut row, &mut num_stack, y)?;
        lines.push(row);
    }

    // now we can set map contents
    for (y, row) in lines.into_iter().enumerate() {
        for (x, tile) in row.into_iter().enumerate() {
            let mut t = m.get_mut(x, y).unwrap();
            t.set(tile);
        }
    }

    Ok(m)
}

trait PartNumbers {
    fn part_numbers(&self) -> Result<Vec<u64>>;
}

impl PartNumbers for Map<Tile> {
    // Returns a vec of all part numbers, defined by having an adjacent
    // symbol. Part numbers are only reported once, even if there are multiple
    // adjacent symbols.
    fn part_numbers(&self) -> Result<Vec<u64>> {
        let mut seen = HashSet::new();
        let mut result = vec![];

        for (_, t) in self.iter() {
            let Tile::Number { number, start, .. } = t.value() else {
                continue;
            };

            if seen.contains(start) {
                continue;
            }

            if t.neighbors().iter().any(|n| n.borrow().value().is_symbol()) {
                seen.insert(*start);
                result.push(*number);
            }
        }

        Ok(result)
    }
}

// for parsing, takes a stack of digits, converts them to a number,
// and builds tiles based on the length. Clears the stack.
//
// tolerates an empty stack for easier guarding.
//
// finally, y is provided to track where numbers begin--- allowing easier uniqueness checks
fn collapse_stack(row: &mut Vec<Tile>, stack: &mut Vec<char>, y: usize) -> Result<()> {
    if stack.is_empty() {
        return Ok(());
    }

    let number = stack
        .iter()
        .collect::<String>()
        .parse()
        .context("when parsing a number stack")?;
    let start = (row.len(), y);
    for digit in 0..stack.len() {
        row.push(Tile::Number {
            number,
            digit,
            start,
        });
    }
    stack.clear();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = r"467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    // successful map parsing should roundtrip its display output
    #[test]
    fn test_map_parse() {
        let map = parse_map(EXAMPLE_INPUT_1).unwrap();
        assert_eq!(
            &EXAMPLE_INPUT_1
                .chars()
                .filter(|c| *c != ' ')
                .collect::<String>(),
            &format!("{map}").trim()
        );
    }

    #[test]
    fn test_example_1_sum() {
        let map = parse_map(EXAMPLE_INPUT_1).unwrap();
        assert_eq!(4361, map_part_number_sum(&map).unwrap());
    }
}
