#![allow(dead_code)]
use std::fmt;
use std::str::FromStr;

use super::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: FromStr + fmt::Display> FromStr for Coordinate<T>
where
    <T as FromStr>::Err: fmt::Display,
{
    type Err = Error;

    // takes line-by-line representation of wall lines
    // and derives a Map
    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.splitn(3, ",").map(|p| p.trim().parse());

        Ok(Self {
            x: parts
                .next()
                .ok_or_else(|| anyhow!("x missing"))?
                .map_err(|e| anyhow!("failed to parse x in '{}': {}", s, e))?,
            y: parts
                .next()
                .ok_or_else(|| anyhow!("y missing"))?
                .map_err(|e| anyhow!("failed to parse y in '{}': {}", s, e))?,
            z: parts
                .next()
                .ok_or_else(|| anyhow!("z missing"))?
                .map_err(|e| anyhow!("failed to parse z in '{}': {}", s, e))?,
        })
    }
}

impl<T: fmt::Display> fmt::Display for Coordinate<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T> Coordinate<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

impl<T: Copy + std::ops::Add<i32, Output = T> + std::ops::Sub<i32, Output = T>> Coordinate<T> {
    fn directions(&self) -> impl Iterator<Item=Coordinate<T>> {
        let (x, y, z) = (self.x, self.y, self.z);
        let v = vec![
            (x - 1, y, z),
            (x, y - 1, z),
            (x, y, z - 1),
            (x + 1, y , z),
            (x, y + 1, z),
            (x, y, z + 1),
        ].into_iter()
            .map(|c| c.into())
            .collect::<Vec<Coordinate<T>>>();

        v.into_iter()
    }
}

impl<T> From<(T, T, T)> for Coordinate<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Coordinate::new(x, y, z)
    }
}
