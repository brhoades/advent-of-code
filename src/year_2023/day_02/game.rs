use advent_of_code::prelude::*;
use std::{collections::HashMap, str::FromStr};

// a set of games. can be parsed from a new line delimited string.
#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut)]
pub struct Games(Vec<Game>);

impl FromStr for Games {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.lines()
            .map(FromStr::from_str)
            .collect::<Result<_>>()
            .map(Self)
    }
}

// Single game instance with many rounds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub id: usize,
    pub rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, game) = parse::game(s).map_err(|e| anyhow!("failed to parse game: {e}"))?;
        Ok(game)
    }
}

impl Game {
    // returns cubes in this game's rounds
    pub fn cubes(&self) -> impl Iterator<Item = (&CubeKind, &usize)> {
        self.rounds.iter().flat_map(|r| r.cubes())
    }

    // provides the minimum number of each kind of cube required to play this game.
    pub fn minimum_cubes(&self) -> HashMap<&CubeKind, usize> {
        let mut ret = HashMap::<&CubeKind, usize>::new();
        for (k, cnt) in self.cubes() {
            ret.entry(k)
                .and_modify(|old| {
                    *old = std::cmp::max(*cnt, *old);
                })
                .or_insert(*cnt);
        }

        ret
    }

    // the product of the minimum cubes
    pub fn power(&self) -> usize {
        self.minimum_cubes()
            .values()
            .cloned()
            .reduce(std::ops::Mul::mul)
            .unwrap_or_default()
    }
}

pub type CubeKind = String;

// An arrangement of various cubes in the bag
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Round {
    cubes: HashMap<CubeKind, usize>,
}

impl Round {
    pub fn cubes(&self) -> &HashMap<CubeKind, usize> {
        &self.cubes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example_line() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game: Games = input.parse().unwrap();
        assert_eq!(
            Games(vec![Game {
                id: 1,
                rounds: vec![
                    Round {
                        cubes: [("blue", 3), ("red", 4)]
                            .into_iter()
                            .map(|(k, v)| (k.to_string(), v))
                            .collect(),
                    },
                    Round {
                        cubes: [("red", 1), ("green", 2), ("blue", 6)]
                            .into_iter()
                            .map(|(k, v)| (k.to_string(), v))
                            .collect(),
                    },
                    Round {
                        cubes: [("green", 2)]
                            .into_iter()
                            .map(|(k, v)| (k.to_string(), v))
                            .collect(),
                    },
                ],
            }]),
            game,
            "incorrect result when parsing for: {input}"
        );
    }

    #[test]
    fn test_parse_edge_cases() {
        let game: Games = "Game 24: 1 a".parse().unwrap();

        assert_eq!(
            Games(vec![Game {
                id: 24,
                rounds: vec![Round {
                    cubes: [("a", 1)]
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v))
                        .collect(),
                },],
            }]),
            game,
        );
    }

    #[test]
    fn test_min_cubes() {
        let game: Game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse()
            .unwrap();
        let expected: HashMap<CubeKind, usize> = [("red", 4), ("blue", 6), ("green", 2)]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        let expected: HashMap<&CubeKind, usize> = expected.iter().map(|(k, v)| (k, *v)).collect();

        assert_eq!(expected, game.minimum_cubes());
    }
}

mod parse {
    use super::{CubeKind, Game, Round};
    use nom::{
        bytes::complete::{tag, take_while1},
        character::complete as chars,
        IResult,
    };

    pub fn game(input: &str) -> IResult<&str, Game> {
        let (input, id) = game_header(input)?;
        let (input, _) = tag(": ")(input)?;

        let (_, rounds) = rounds(input)?;
        Ok((input, Game { id, rounds }))
    }

    // parses out the initial game statement and returns the ID
    fn game_header(input: &str) -> IResult<&str, usize> {
        let (input, _) = tag("Game ")(input)?;
        let (input, game_id) = chars::u64(input)?;

        Ok((input, game_id as usize))
    }

    // parses all rounds following the game header
    fn rounds(mut input: &str) -> IResult<&str, Vec<Round>> {
        let mut ret = vec![];
        while !input.is_empty() {
            let (new_input, rstr) =
                take_while1(|c: char| c == ' ' || c == ',' || c.is_ascii_alphanumeric())(input)?;
            input = new_input;

            if !input.is_empty() {
                (input, _) = tag("; ")(input)?;
            }

            let (_, round) = round(rstr)?;
            ret.push(round);
        }
        Ok((input, ret))
    }

    // parses a list of comma-separated dice rolls terminated by a semicolon
    fn round(mut input: &str) -> IResult<&str, Round> {
        let mut ret = vec![];
        while !input.is_empty() {
            let (new_input, dicestr) =
                take_while1(|c: char| c == ' ' || c.is_ascii_alphanumeric())(input)?;
            input = new_input;

            if !input.is_empty() {
                (input, _) = tag(", ")(input)?;
            }

            let (_, dice) = dice(dicestr)?;
            ret.push(dice);
        }

        Ok((
            input,
            Round {
                cubes: ret.into_iter().collect(),
            },
        ))
    }

    fn dice(input: &str) -> IResult<&str, (CubeKind, usize)> {
        let (input, count) = chars::u64(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, kind) = take_while1(|c: char| c.is_ascii_alphanumeric())(input)?;
        Ok((input, (kind.to_string(), count as usize)))
    }
}
