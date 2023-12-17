#![allow(dead_code)]
use crate::prelude::*;
use std::{
    cell::{self as cell, RefCell},
    fmt,
    rc::Rc,
};

// wraps a map point, providing helpers to interact with sibling nodes
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Node<T>(Rc<RefCell<NodeData<T>>>);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NodeData<T> {
    x: usize,
    y: usize,
    inner: T,
    neighbors: Neighbors<T>,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Neighbors<T> {
    pub up: Option<Node<T>>,
    pub right: Option<Node<T>>,
    pub down: Option<Node<T>>,
    pub left: Option<Node<T>>,

    pub upright: Option<Node<T>>,
    pub downright: Option<Node<T>>,
    pub downleft: Option<Node<T>>,
    pub upleft: Option<Node<T>>,
}

impl<T> Neighbors<T> {
    // returns an iteterator over present neighbors
    pub fn iter(&self) -> impl Iterator<Item = &Node<T>> {
        vec![
            &self.up,
            &self.right,
            &self.down,
            &self.left,
            &self.upright,
            &self.downright,
            &self.downleft,
            &self.upleft,
        ]
        .into_iter()
        .filter_map(|n| n.as_ref())
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }
}

impl<T> Node<T> {
    pub fn borrow(&self) -> cell::Ref<'_, NodeData<T>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> cell::RefMut<'_, NodeData<T>> {
        self.0.borrow_mut()
    }
}

// custom impl to avoid recursively debugging every node in the map
impl<T: fmt::Debug> fmt::Debug for Neighbors<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt_node = |n: &Option<Node<T>>| {
            n.as_ref()
                .map(|n| format!("({}, {})", n.0.borrow().x, n.0.borrow().y))
        };
        f.debug_struct("Neighbors")
            .field("up", &fmt_node(&self.up))
            .field("right", &fmt_node(&self.right))
            .field("down", &fmt_node(&self.down))
            .field("left", &fmt_node(&self.left))
            .field("upright", &fmt_node(&self.upright))
            .field("downright", &fmt_node(&self.downright))
            .field("downleft", &fmt_node(&self.downleft))
            .field("upleft", &fmt_node(&self.upleft))
            .finish()
    }
}

impl<T> NodeData<T> {
    pub fn neighbors(&self) -> &Neighbors<T> {
        &self.neighbors
    }

    pub fn neighbors_mut(&mut self) -> &mut Neighbors<T> {
        &mut self.neighbors
    }

    // sets the interior value, returning the previous one
    pub fn set(&mut self, new: T) -> T {
        std::mem::replace(&mut self.inner, new)
    }

    // gets a reference to the interior value
    pub fn value(&self) -> &T {
        &self.inner
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

// Map is a safe, densely allocated grid of nodes.
// Nodes contain references to sibling nodes in the grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map<T> {
    // y --> x --> T
    tiles: Vec<Vec<Node<T>>>,
    width: usize,
    height: usize,
}

impl<T: Default + Clone> Map<T> {
    // new initializes the datastore then goes back to populate neighbor
    // values since they're only present after initialization
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = vec![vec![]; height];

        for y in 0..height {
            let row = tiles.get_mut(y).unwrap();
            for x in 0..width {
                row.push(Node(Rc::new(RefCell::new(NodeData {
                    x,
                    y,
                    ..Default::default()
                }))));
            }
        }

        let ret = Self {
            tiles,
            width,
            height,
        };

        for y in 0..height {
            for x in 0..width {
                let mut node = ret.get_mut(x, y).unwrap();

                node.x = x;
                node.y = y;
                if x > 0 {
                    // left
                    node.neighbors.left = ret.get_node(x - 1, y).ok().cloned();
                }
                if y > 0 {
                    // down
                    node.neighbors.down = ret.get_node(x, y - 1).ok().cloned();
                }
                // right
                node.neighbors.right = ret.get_node(x + 1, y).ok().cloned();
                // up
                node.neighbors.up = ret.get_node(x, y + 1).ok().cloned();

                node.neighbors.upright = ret.get_node(x + 1, y + 1).ok().cloned();
                if y > 0 {
                    node.neighbors.downright = ret.get_node(x + 1, y - 1).ok().cloned();
                    if x > 0 {
                        node.neighbors.downleft = ret.get_node(x - 1, y - 1).ok().cloned();
                    }
                }
                if x > 0 {
                    node.neighbors.upleft = ret.get_node(x - 1, y + 1).ok().cloned();
                }
            }
        }

        ret
    }
}

impl<T: fmt::Display> fmt::Display for Map<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // we're stored with y=0 first, walk numbers instead of using iter
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y).unwrap().inner)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum MapError {
    #[error("specified coordinate ({x}, {y}) is out of bounds: width={width}, height={height}")]
    OutOfBounds {
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    },
    #[error("requested node at ({0}, {1}) is already borrowed exclusively")]
    DoubleBorrow(usize, usize),
}

impl<T> Map<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    // iter walks each item in each row, bottom to top, left to right, borrowing the node data
    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), cell::Ref<'_, NodeData<T>>)> {
        self.tiles.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, node)| ((x, y), node.borrow()))
        })
    }

    // iter_mut is like iter, but walks a mutable reference of the inner node data.
    pub fn iter_mut(
        &self,
    ) -> impl Iterator<Item = ((usize, usize), cell::RefMut<'_, NodeData<T>>)> {
        self.tiles.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, node)| ((x, y), node.borrow_mut()))
        })
    }

    // iterates over the nodes in
    pub fn iter_nodes(&self) -> impl Iterator<Item = ((usize, usize), &Node<T>)> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, node)| ((x, y), node)))
    }

    // return the interior refcell borrowed for easier API use. Node is transparent
    // unless explicitly requested.
    //
    // panics if the node is borrowed mutably.
    pub fn get(&self, x: usize, y: usize) -> Result<cell::Ref<'_, NodeData<T>>, MapError> {
        self.get_node(x, y)?
            .0
            .try_borrow()
            .map_err(|_| MapError::DoubleBorrow(x, y))
    }

    // return the interior refcell borrowed for easier API use. Node is transparent
    // unless explicitly requested.
    //
    // panics if the node is borrowed mutably.
    pub fn get_node(&self, x: usize, y: usize) -> Result<&Node<T>, MapError> {
        if y >= self.height || x >= self.width {
            return Err(MapError::OutOfBounds {
                x,
                y,
                width: self.width,
                height: self.height,
            });
        }

        Ok(self.tiles.get(y).and_then(|row| row.get(x)).unwrap())
    }

    // return the interior refcell borrowed mutably for easier API use. If we returned the Node,
    // we'd have to lock the entire row exclusively.
    pub fn get_mut(&self, x: usize, y: usize) -> Result<cell::RefMut<'_, NodeData<T>>, MapError> {
        self.get_node(x, y)?
            .0
            .try_borrow_mut()
            .map_err(|_| MapError::DoubleBorrow(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // checks a few nodes to see if neighbors are properly populated
    #[test]
    fn test_neighbors() {
        let m = Map::<()>::new(5, 5);

        let n = m.get(0, 0).unwrap();
        assert_eq!((0, 0), (n.x, n.y));

        let n = n.neighbors();
        assert_eq!(None, n.down);
        assert_eq!(None, n.left);
        let right = n.right.as_ref().unwrap().borrow();
        assert_eq!((1, 0), (right.x, right.y));
        let up = n.up.as_ref().unwrap().borrow();
        assert_eq!((0, 1), (up.x, up.y));

        let n = m.get(3, 3).unwrap();
        assert_eq!((3, 3), (n.x, n.y));

        let n = n.neighbors();
        let right = n.right.as_ref().unwrap().borrow();
        assert_eq!((4, 3), (right.x, right.y));
        let down = n.down.as_ref().unwrap().borrow();
        assert_eq!((3, 2), (down.x, down.y));
        let left = n.left.as_ref().unwrap().borrow();
        assert_eq!((2, 3), (left.x, left.y));
        let up = n.up.as_ref().unwrap().borrow();
        assert_eq!((3, 4), (up.x, up.y));
    }
}
