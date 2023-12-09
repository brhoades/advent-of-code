use advent_of_code::prelude::*;
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use Card::*;

        Ok(match s {
            "2" => Two,
            "3" => Three,
            "4" => Four,
            "5" => Five,
            "6" => Six,
            "7" => Seven,
            "8" => Eight,
            "9" => Nine,
            "T" => Ten,
            "J" => Jack,
            "Q" => Queen,
            "K" => King,
            "A" => Ace,
            other => bail!("unknown card '{other}'"),
        })
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Card::*;

        let s = match self {
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Ten => "T",
            Jack => "J",
            Queen => "Q",
            King => "K",
            Ace => "A",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Hand([Card; 5]);

impl std::ops::Deref for Hand {
    type Target = [Card; 5];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Hand(
            s.split("")
                .filter(|c| !c.is_empty())
                .map(FromStr::from_str)
                .collect::<Result<Vec<_>>>()?
                .try_into()
                .map_err(|v: Vec<_>| {
                    anyhow!(
                        "invalid hand size {}? failed to convert to array: {v:?}",
                        v.len()
                    )
                })?,
        ))
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum Rank {
    HighCard(Card),
    OnePair(Card),
    TwoPair(Card, Card), // highest pair first
    ThreeOfAKind(Card),
    FullHouse(Card, Card), // triplet comes first
    FourOfAKind(Card),
    FiveOfAKind(Card),
}

impl Hand {
    // returns counts ordered by usize
    pub fn counts(&self) -> HashMap<Card, usize> {
        self.iter().fold(HashMap::with_capacity(5), |mut acc, c| {
            acc.entry(*c).and_modify(|v| *v = *v + 1).or_insert(1);
            acc
        })
    }
}

pub trait Ranked {
    fn rank(&self) -> Rank;
}

impl Ranked for Hand {
    fn rank(&self) -> Rank {
        use Rank::*;

        let counts = self.counts();

        match counts.len() {
            1 => FiveOfAKind(counts.into_iter().next().unwrap().0),
            2 if counts.iter().any(|(_, cnt)| *cnt == 2) => FullHouse(
                take_card_with_count(&counts, 3).unwrap(),
                take_card_with_count(&counts, 2).unwrap(),
            ),
            2 => FourOfAKind(take_card_with_count(&counts, 4).unwrap()),
            3 if counts.iter().any(|(_, cnt)| *cnt == 3) => {
                ThreeOfAKind(take_card_with_count(&counts, 3).unwrap())
            }
            3 => {
                let mut pairs = counts
                    .into_iter()
                    .filter_map(|(card, cnt)| if cnt == 2 { Some(card) } else { None })
                    .collect::<Vec<_>>();
                // dance a bit to get the highest pair first (for easier scoring)
                pairs.sort();

                TwoPair(pairs.pop().unwrap(), pairs.pop().unwrap())
            }
            // // Two pair: Z, X = 2, Y = 2 OR three of a kind: Z, X, Y = 3
            4 => OnePair(take_card_with_count(&counts, 2).unwrap()),
            5 => HighCard(self.iter().max().cloned().unwrap()),
            6..=usize::MAX => unreachable!("unreachable hand rank {counts:?}"),
            _ => todo!(),
        }
    }
}

fn take_card_with_count(counts: &HashMap<Card, usize>, cnt: usize) -> Option<Card> {
    counts
        .iter()
        .find_map(|(card, c)| if *c == cnt { Some(card) } else { None })
        .copied()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_hand_cards() {
        use Card::*;

        let cases = vec![
            ("AKQJT", [Ace, King, Queen, Jack, Ten]),
            ("98765", [Nine, Eight, Seven, Six, Five]),
            ("432AK", [Four, Three, Two, Ace, King]),
        ];

        for (input, expected) in cases {
            let hand = input.parse();
            assert!(
                hand.is_ok(),
                "failed to parse {input}: {}",
                hand.err().unwrap()
            );

            assert_eq!(Hand(expected), hand.unwrap());
        }
    }

    #[test]
    fn test_rank() {
        use Card::*;
        use Rank::*;

        let cases = vec![
            ("23456", HighCard(Six)),
            ("98765", HighCard(Nine)),
            ("432AK", HighCard(Ace)),
            ("22AKQ", OnePair(Two)),
            ("A2AKQ", OnePair(Ace)),
            ("34QJQ", OnePair(Queen)),
            ("J4QJQ", TwoPair(Queen, Jack)),
            ("242JJ", TwoPair(Jack, Two)),
            ("32JJJ", ThreeOfAKind(Jack)),
            ("2Q2K2", ThreeOfAKind(Two)),
            ("33324", ThreeOfAKind(Three)),
            ("KKKJJ", FullHouse(King, Jack)),
            ("Q2Q2Q", FullHouse(Queen, Two)),
            ("3T3T3", FullHouse(Three, Ten)),
            ("33333", FiveOfAKind(Three)),
            ("TTTTT", FiveOfAKind(Ten)),
        ];

        for (input, expected) in cases {
            let hand: Hand = input.parse().unwrap();

            println!("checking hand {input}");
            assert_eq!(expected, hand.rank());
        }
    }

    #[test]
    fn test_cmp() {
        use Card::*;
        use Rank::*;

        // (l, r, l_wins)
        let cases = vec![
            ("23456", "234QA", false),
            ("23456", "23452", true),
            ("98765", "87654", true),
            // 1 pair
            ("22AKQ", "22A3Q", true),
            ("224K3", "22AQQ", false),
            // 2 pair
            ("J4QJQ", "4QJQJ", true),
            ("J4QJQ", "2233Q", true),
            // mixed
            ("AAQQQ", "AA234", true),
            ("AAQQQ", "AAAAA", true),
            ("22222", "AT234", true),
        ]
        .into_iter()
        .map(|(l, r, e)| -> (Hand, Hand, bool) { (l.parse().unwrap(), r.parse().unwrap(), e) });

        for (l, r, expected) in cases {
            assert_eq!(
                expected,
                l.rank() > r.rank(),
                "expected {expected} in {l:?} > {r:?}"
            );
        }
    }
}
