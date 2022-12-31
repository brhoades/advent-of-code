use std::cmp::min;

use anyhow::{anyhow, Result};

#[derive(Debug, Default, Clone)]
pub struct Map<T> {
    data: Vec<Vec<T>>,              // y => x => Tile
    pub dimensions: (usize, usize), // (x, y)
}

impl<T: Default + Clone> Map<T> {
    /// new_dense creates a densely allocated Map with the provided
    /// dimensions available for use.
    pub fn new_dense(width: usize, height: usize) -> Self {
        Self {
            data: (0..height)
                .map(|_| {
                    let mut row = vec![];
                    row.resize(width, Default::default());
                    row
                })
                .collect::<Vec<_>>(),
            dimensions: (width, height),
        }
    }

    /// resize is a destructive resize of self.data. Tiles not covered by the new
    /// dimensions are deleted.
    pub fn resize(&mut self, width: usize, height: usize) {
        // can only copy the smallest of the new dimensions and current dimensions.
        let tocopy = (
            min(width, self.dimensions.0),
            min(height, self.dimensions.1),
        );
        let mut new = Map::<T>::new_dense(width, height);

        for y in 0..tocopy.1 {
            for x in 0..tocopy.0 {
                *new.get_mut(x, y).unwrap() = self.get(x, y).unwrap().clone();
            }
        }

        self.data = new.data;
        self.dimensions = (width, height);
    }
}

impl<T> Map<T> {
    /// from_data assumes input is square
    /// data should be stored in y => x => T form
    pub fn from_data(data: Vec<Vec<T>>) -> Result<Self> {
        let dimensions = (
            data.first()
                .map(|f| f.len())
                .ok_or_else(|| anyhow!("couldn't get first row to determine dimensions"))?,
            data.len(),
        );
        Ok(Self { data, dimensions })
    }

    pub fn get(&self, x: usize, y: usize) -> Result<&T> {
        self.data.get(y).and_then(|row| row.get(x)).ok_or_else(|| {
            anyhow!(
                "map w/ dimens {:?} lacks tile at ({}, {})",
                self.dimensions,
                x,
                y
            )
        })
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut T> {
        self.data
            .get_mut(y)
            .and_then(|row| row.get_mut(x))
            .ok_or_else(|| {
                anyhow!(
                    "map w/ dimens {:?} lacks tile at ({}, {})",
                    self.dimensions,
                    x,
                    y
                )
            })
    }

    pub fn iter_rows(&self) -> std::slice::Iter<Vec<T>> {
        self.data.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt;

    #[test]
    fn test_map_resize() {
        let mut m = Map::<bool>::new_dense(5, 5);
        *m.get_mut(1, 1).unwrap() = true;
        *m.get_mut(1, 4).unwrap() = true;
        *m.get_mut(4, 4).unwrap() = true;

        let mut resized = m.clone();

        // expand max
        resized.resize(10, 10);

        assert_map_subset(&m, &resized, (5, 5));
    }

    fn assert_map_subset<T: Eq + fmt::Debug>(inner: &Map<T>, outer: &Map<T>, b: (usize, usize)) {
        let contextstr = |e: &str, inner: &Map<T>, outer: &Map<T>| {
            format!("{:?}:\ninner:\n{:?}\nnouter:\n{:?}", e, inner, outer)
        };

        for x in 0..b.0 {
            for y in 0..b.1 {
                let old_t = inner
                    .get(x, y)
                    .map_err(|e| contextstr(&e.to_string(), inner, outer))
                    .unwrap();
                let new_t = outer
                    .get(x, y)
                    .map_err(|e| contextstr(&e.to_string(), inner, outer))
                    .unwrap();

                assert_eq!(
                    *old_t,
                    *new_t,
                    "{}",
                    contextstr("tiles are misarranged on old and new maps", inner, outer)
                );
            }
        }
    }
}
