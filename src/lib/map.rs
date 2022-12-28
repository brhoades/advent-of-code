use anyhow::{anyhow, Result};

#[derive(Debug, Default, Clone)]
pub struct Map<T> {
    data: Vec<Vec<T>>,              // y => x => Tile
    pub dimensions: (usize, usize), // (x, y)
}

impl<T: Default + Clone> Map<T> {
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
