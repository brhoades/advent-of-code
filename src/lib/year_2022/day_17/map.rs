#![allow(dead_code)]
use crate::prelude::*;

/// Map is grid of configurable maximum width with growing height.
/// Map stores whether a block is set or not.
#[derive(Debug, Clone)]
pub struct Map {
    // Y -> X -> is_set
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize) -> Self {
        Self {
            grid: vec![],
            height: 0,
            width,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.grid
            .get(y)
            .and_then(|row| row.get(x))
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut bool {
        if y >= self.grid.len() {
            self.grid.resize(y + 1, vec![false; self.width]);
        }

        self.grid.get_mut(y).unwrap().get_mut(x).unwrap()
    }

    pub fn set(&mut self, x: usize, y: usize) -> Result<()> {
        if x >= self.width {
            bail!(
                "({x}, {y}) is out of bounds in Map with width {}",
                self.width
            );
        }

        *self.get_mut(x, y) = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_get_set_resize() {
        let mut map = Map::new(7);
        assert!(!map.get(0, 0));
        assert!(!map.get(10, 0));
        assert!(!map.get(10, 10));
        assert!(!map.get(0, 10));

        map.set(0, 0).unwrap();
        map.set(0, 1).unwrap();
        map.set(5, 50).unwrap();
        assert!(map.get(0, 0));
        assert!(map.get(0, 1));
        assert!(map.get(5, 50));
        assert!(!map.get(0, 3));
        assert!(!map.get(4, 5));

        assert!(map.set(7, 5).is_err());
        assert!(map.set(10, 5).is_err());
        assert!(!map.get(5, 7));
        assert!(!map.get(5, 10));
    }
}
