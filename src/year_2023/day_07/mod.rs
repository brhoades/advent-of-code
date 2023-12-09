mod cards;
use cards::*;

use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    let bids = parse_input(&input)?;

    let total = total_winnings(bids);
    println!("total winnings: {total}");

    let bids = parse_input_jacks_wild(&input)?;
    let total = total_winnings(bids);
    println!("total winnings with jacks wild: {total}");

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<(Hand, u64)>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut pieces = l.trim().splitn(2, ' ');
            let hand = pieces.next().context("incorrect line")?.parse()?;
            let bid = pieces.next().context("incorrect line")?.parse()?;

            Ok((hand, bid))
        })
        .collect()
}

fn parse_input_jacks_wild(input: &str) -> Result<Vec<(Hand, u64)>> {
    parse_input(&input.replace('J', "*"))
}

fn total_winnings(mut bids: Vec<(Hand, u64)>) -> u64 {
    bids.sort_by(|(l, _), (r, _)| l.cmp(r));
    bids.into_iter()
        .enumerate()
        .map(|(rank, play)| ((rank + 1) as u64, play))
        .map(|(rank, (_, bid))| rank * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let bids = parse_input(
            r"32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483",
        )
        .unwrap();

        assert_eq!(6440, total_winnings(bids));
    }

    #[test]
    fn test_example_input_wild_jacks() {
        let bids = parse_input_jacks_wild(
            r"32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483",
        )
        .unwrap();

        assert_eq!(5905, total_winnings(bids));
    }
}
