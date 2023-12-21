use std::collections::HashMap;

use crate::prelude::*;

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

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Map(HashMap<String, Fork>);

#[derive(Deref, DerefMut)]
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
