use std::fmt;
use std::str::FromStr;

use super::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T: FromStr + fmt::Display> FromStr for Coordinate<T>
where
    <T as FromStr>::Err: fmt::Display,
{
    type Err = Error;

    // takes line-by-line representation of wall lines
    // and derives a Map
    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.splitn(2, ',').map(|p| p.trim().parse());

        Ok(Self {
            x: parts
                .next()
                .ok_or_else(|| anyhow!("x missing"))?
                .map_err(|e| anyhow!("failed to parse x in '{}': {}", s, e))?,
            y: parts
                .next()
                .ok_or_else(|| anyhow!("y missing"))?
                .map_err(|e| anyhow!("failed to parse y in '{}': {}", s, e))?,
        })
    }
}

impl<T: fmt::Display> fmt::Display for Coordinate<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
