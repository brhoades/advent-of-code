use advent_of_code::prelude::*;
use std::{collections::HashSet, str::FromStr};

pub fn run(input: String) -> Result<()> {
    let pile: Pile = input.parse()?;

    println!("total score for pile: {}", pile.score());

    println!("total cards for pile: {}", pile.total_scorecards());

    Ok(())
}

#[derive(Debug)]
struct Pile(Vec<ScratchCard>);

impl Pile {
    // for the part 1 scoring mechanics
    fn score(&self) -> u64 {
        self.0.iter().map(ScratchCard::points).sum()
    }

    // Following logic in part 2, returns how many total scratchcards
    // are won. An intelligent implementation would walk these backwards
    // and cache each result over time.
    fn total_scorecards(&self) -> usize {
        let mut total_scorecards = 0;
        let mut next_scorecards: Vec<_> = self.0.iter().map(|s| s.number).collect();

        while let Some(idx) = next_scorecards.pop() {
            let sc = self.0.get(idx - 1).unwrap();
            total_scorecards += 1;
            let cnt = sc.matches();
            if cnt == 0 {
                continue;
            }
            for off in 1..=cnt {
                next_scorecards.push(sc.number + off);
            }
        }

        total_scorecards
    }
}

impl FromStr for Pile {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(
            s.lines()
                .map(FromStr::from_str)
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ScratchCard {
    number: usize,
    winning_numbers: Vec<u64>,
    scratched_numbers: Vec<u64>,

    winning_numbers_lookup: HashSet<u64>,
}

impl ScratchCard {
    fn matches(&self) -> usize {
        self.scratched_numbers
            .iter()
            .filter(|n| self.winning_numbers_lookup.contains(n))
            .count()
    }

    fn points(&self) -> u64 {
        let cnt = self.matches() as u32;
        if cnt == 0 {
            0
        } else {
            2_u64.pow(cnt - 1)
        }
    }
}

impl FromStr for ScratchCard {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split(":").collect::<Vec<_>>();

        let &[number, scorecard] = parts.as_slice() else {
            bail!("scratchcard didn't appear split by colon")
        };

        let &[winning_numbers, scratched_numbers] =
            scorecard.trim().split(" | ").collect::<Vec<_>>().as_slice()
        else {
            bail!("scratchcard didn't appear split by a pipe");
        };

        let winning_numbers: Vec<u64> = winning_numbers
            .trim()
            .split_terminator(" ")
            .filter(|n| !n.is_empty())
            .map(|n| Ok(n.parse()?))
            .collect::<Result<_>>()?;
        let scratched_numbers: Vec<u64> = scratched_numbers
            .trim()
            .split_terminator(" ")
            .filter(|n| !n.is_empty())
            .map(|n| Ok(n.parse()?))
            .collect::<Result<_>>()?;

        Ok(Self {
            number: number[4..] // "Card"
                .trim()
                .parse()
                .with_context(|| format!("parsing card number: {number}"))?,
            winning_numbers_lookup: winning_numbers.iter().cloned().collect(),
            winning_numbers,
            scratched_numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example_line() {
        let card: ScratchCard = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .parse()
            .unwrap();

        assert_eq!(6, card.number);
        assert_eq!(vec![31, 18, 13, 56, 72], card.winning_numbers);
        assert_eq!(vec![74, 77, 10, 23, 35, 67, 36, 11], card.scratched_numbers);
    }

    #[test]
    fn test_score_examples() {
        let pile: Pile = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .parse()
            .unwrap();

        assert_eq!(13, pile.score());
    }

    #[test]
    fn test_total_card_example() {
        let pile: Pile = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .parse()
            .unwrap();

        assert_eq!(30, pile.total_scorecards());
    }
}
