use advent_of_code::prelude::*;
use std::cmp::Ordering;
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Card {
    Joker,
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
            "*" => Joker,
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
            Joker => "*",
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deref, DerefMut)]
pub struct Hand([Card; 5]);

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
                        "invalid hand size {}; failed to convert to array: {v:?}",
                        v.len()
                    )
                })?,
        ))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let l = self.rank();
        let r = other.rank();

        let cmp = l.cmp(&r);
        if cmp != Ordering::Equal {
            return cmp;
        }
        self.iter()
            .zip(other.iter())
            .map(|(l, r)| l.cmp(r))
            .find(|c| *c != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    // returns counts of cards for determining hand class. Jokers become the best
    // card possible.
    pub fn counts(&self) -> HashMap<Card, usize> {
        let mut counts = self.iter().fold(HashMap::with_capacity(5), |mut acc, c| {
            acc.entry(*c).and_modify(|v| *v += 1).or_insert(1);
            acc
        });

        // with jokers, our "best" card gets all of them. Otherwise, they become aces.
        if let Some(cnt) = counts.remove(&Card::Joker) {
            let most = counts.values().max();
            if most.is_none() {
                counts.insert(Card::Ace, cnt);
                return counts;
            }

            let most = most.unwrap();
            let best_match = counts
                .iter()
                .filter(|(_, cnt)| *cnt == most)
                .max_by_key(|(card, _)| *card)
                .unwrap();

            counts.insert(*best_match.0, most + cnt);
        }

        counts
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
            1 => FiveOfAKind,
            2 if counts.iter().any(|(_, cnt)| *cnt == 2) => FullHouse,
            2 => FourOfAKind,
            3 if counts.iter().any(|(_, cnt)| *cnt == 3) => ThreeOfAKind,
            3 => TwoPair,
            4 => OnePair,
            5 => HighCard,
            6..=usize::MAX => unreachable!("unreachable hand rank {counts:?}"),
            _ => todo!(),
        }
    }
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
        use Rank::*;

        let cases = vec![
            ("23456", HighCard),
            ("98765", HighCard),
            ("432AK", HighCard),
            ("22AKQ", OnePair),
            ("A2AKQ", OnePair),
            ("34QJQ", OnePair),
            ("J4QJQ", TwoPair),
            ("242JJ", TwoPair),
            ("32JJJ", ThreeOfAKind),
            ("2Q2K2", ThreeOfAKind),
            ("33324", ThreeOfAKind),
            ("KKKJJ", FullHouse),
            ("Q2Q2Q", FullHouse),
            ("3T3T3", FullHouse),
            ("33333", FiveOfAKind),
            ("TTTTT", FiveOfAKind),
        ];

        for (input, expected) in cases {
            let hand: Hand = input.parse().unwrap();

            println!("checking hand {input}");
            assert_eq!(expected, hand.rank());
        }
    }

    #[test]
    fn test_cmp() {
        let cases = vec![
            ("23456", "234QA", Ordering::Less),
            ("AKQ34", "AKQ23", Ordering::Greater),
            ("98765", "87654", Ordering::Greater),
            // 1 pair
            ("22AKQ", "22A3Q", Ordering::Greater),
            ("224K3", "22AQQ", Ordering::Less),
            // 2 pair
            ("J4QJQ", "4QJQJ", Ordering::Greater),
            ("J4QJQ", "2233Q", Ordering::Greater),
            // mixed
            ("AAQQQ", "AA234", Ordering::Greater),
            ("AAQQQ", "AAAAA", Ordering::Less),
            ("22222", "AT234", Ordering::Greater),
            ("A23QJ", "AAJJQ", Ordering::Less),
            ("AAAAA", "AAAAA", Ordering::Equal),
        ]
        .into_iter()
        .map(|(l, r, e)| -> (Hand, Hand, Ordering) { (l.parse().unwrap(), r.parse().unwrap(), e) });

        for (l, r, expected) in cases {
            assert_eq!(
                expected,
                l.cmp(&r),
                "expected {expected:?} in {l:?} <=> {r:?}"
            );
        }
    }

    #[test]
    fn test_cmp_jokers() {
        let cases = vec![
            ("*****", "*****", Ordering::Equal),
            ("K****", "Q****", Ordering::Greater),
            ("224**", "2T***", Ordering::Less),
        ]
        .into_iter()
        .map(|(l, r, e)| -> (Hand, Hand, Ordering) { (l.parse().unwrap(), r.parse().unwrap(), e) });

        for (l, r, expected) in cases {
            assert_eq!(
                expected,
                l.cmp(&r),
                "expected {expected:?} in {l:?} <=> {r:?}"
            );
        }
    }
}
