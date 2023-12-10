use std::collections::HashMap;

use advent_of_code::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Fork {
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone)]
pub struct Map(HashMap<String, Fork>);

pub struct Directions(Box<dyn Iterator<Item = Dir>>);

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => bail!("unknown Dir {s}"),
        })
    }
}

impl std::ops::Deref for Directions {
    type Target = Box<dyn Iterator<Item = Dir>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Directions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Directions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(Box::new(
            s.split("")
                .filter(|s| *s != "\n" && *s != " " && !s.is_empty())
                .map(FromStr::from_str)
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .cycle(),
        )))
    }
}

impl std::ops::Deref for Map {
    type Target = HashMap<String, Fork>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(
            s.lines()
                .map(|l| {
                    let mut pieces = l.splitn(2, " = ").collect::<Vec<_>>();
                    ensure!(pieces.len() == 2, "line is too small: '{l}'");
                    let to = pieces.pop().unwrap();
                    let from = pieces.pop().unwrap();

                    let to: Vec<_> = to
                        .trim_matches(|c| c == '(' || c == ')')
                        .splitn(2, ", ")
                        .map(str::to_string)
                        .collect();
                    ensure!(to.len() == 2, "line is too small: '{l}'");
                    let mut to = to.into_iter();

                    Ok((
                        from.to_string(),
                        Fork {
                            left: to.next().unwrap(),
                            right: to.next().unwrap(),
                        },
                    ))
                })
                .collect::<Result<_>>()?,
        ))
    }
}
