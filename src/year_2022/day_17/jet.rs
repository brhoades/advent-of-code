use advent_of_code::prelude::*;
use std::{fmt, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
pub enum JetDirection {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
pub struct JetPattern(Vec<JetDirection>);

impl IntoIterator for JetPattern {
    type Item = JetDirection;

    type IntoIter = std::vec::IntoIter<JetDirection>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromStr for JetPattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        s.split("")
            .filter(|s| !s.is_empty())
            .map(FromStr::from_str)
            .collect()
    }
}

impl FromIterator<JetDirection> for JetPattern {
    fn from_iter<T: IntoIterator<Item = JetDirection>>(iter: T) -> Self {
        JetPattern(iter.into_iter().collect())
    }
}

impl fmt::Display for JetDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JetDirection::Left => write!(f, "<"),
            JetDirection::Right => write!(f, ">"),
        }
    }
}

impl FromStr for JetDirection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Self::Right),
            "<" => Ok(Self::Left),
            other => bail!("unknown jet direction '{other}'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_parse_wind() -> Result<()> {
        use JetDirection::*;
        let wind: JetPattern = EXAMPLE_ONE.parse().unwrap();

        assert_eq!(
            vec![
                Right, Right, Right, Left, Left, Right, Left, Right, Right, Left, Left, Left,
                Right, Right, Left, Right, Right, Right, Left, Left, Left, Right, Right, Right,
                Left, Left, Left, Right, Left, Left, Left, Right, Right, Left, Right, Right, Left,
                Left, Right, Right
            ],
            wind.into_iter().collect::<Vec<_>>()
        );

        Ok(())
    }
}
