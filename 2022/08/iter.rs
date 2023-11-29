use std::iter::repeat;

#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Coordinate {
    fn from(c: (usize, usize)) -> Self {
        Self { x: c.0, y: c.1 }
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from(c: (i32, i32)) -> Self {
        Self {
            x: c.0 as usize,
            y: c.1 as usize,
        }
    }
}

// rays_from_point provides an iterator which yields coords from cardinal directions
// at the provied point.
//
// The provided point is excluded in iterators; they start from the 'next' point.
// Iterators return in order: x+ x- y+ y-
pub fn rays_from_point<D: std::borrow::Borrow<(usize, usize)>>(
    dimens: D,
    point: Coordinate,
) -> DirectionalGridIterator {
    let dimens = dimens.borrow();

    // 1 2 3
    // 4 3 2
    //   x-->
    // 1 5 4
    let posx_iter = ((point.x + 1)..dimens.0)
        .zip(repeat(point.y))
        .collect::<Vec<_>>();

    // 1 2 3
    // 4 3 2
    // <-- x
    // 1 5 4
    let negx_iter = (0..point.x).rev().zip(repeat(point.y)).collect::<Vec<_>>();

    // 1 2 3
    // 4 x 2
    //   |
    //   V
    // 1 5 4
    let posy_iter = repeat(point.x)
        .zip((point.y + 1)..dimens.1)
        .collect::<Vec<_>>();

    // 1 2 3
    // 4 3 2
    //   ^
    //   |
    // 1 x 4
    let negy_iter = repeat(point.x).zip((0..point.y).rev()).collect::<Vec<_>>();

    let iters = vec![posx_iter, negx_iter, posy_iter, negy_iter]
        .into_iter()
        .rev() // they're popped
        .map(RayIterator::from)
        .collect();

    DirectionalGridIterator { iters }
}

pub struct DirectionalGridIterator {
    iters: Vec<RayIterator>,
}

impl Iterator for DirectionalGridIterator {
    type Item = RayIterator;

    fn next(&mut self) -> Option<Self::Item> {
        self.iters.pop()
    }
}

// RayIterator iterates over trees in a single direction.
pub struct RayIterator {
    // stored in reverse so we just pop
    items: Vec<(usize, usize)>,
}

impl RayIterator {
    fn from(mut v: Vec<(usize, usize)>) -> Self {
        v.reverse();
        RayIterator { items: v }
    }

    #[allow(dead_code)]
    fn from_iter<I: Iterator<Item = (usize, usize)>>(i: I) -> Self {
        Self::from(i.collect())
    }
}

impl Iterator for RayIterator {
    // ((absolute coords), tree_height)
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }
}

#[test]
fn test_basic_walk() {
    let st = (4_usize, 4_usize);
    let (sx, sy) = st;
    let mut dirs = rays_from_point((10, 10), st.into());
    for (i, (x, y)) in dirs.next().expect("failed to get x+").enumerate() {
        assert_eq!(
            (sx + i + 1, sy),
            (x, y),
            "x+ quality check failed at i={}",
            i
        );
    }
    for (i, (x, y)) in dirs.next().expect("failed to get x-").enumerate() {
        assert_eq!(
            (sx - i - 1, sy),
            (x, y),
            "x- equality check failed at i={}",
            i
        );
    }
    for (i, (x, y)) in dirs.next().expect("failed to get y+").enumerate() {
        assert_eq!(
            (sx, sy + i + 1),
            (x, y),
            "y+ equality check failed at i={}",
            i
        );
    }
    for (i, (x, y)) in dirs.next().expect("failed to get y-").enumerate() {
        assert_eq!(
            (sx, sy - i - 1),
            (x, y),
            "y- equality check failed at i={}",
            i
        );
    }
}

// literal edge cases
#[test]
fn test_edge_cases() {
    let st = (3_usize, 3_usize);
    let (sx, sy) = st;
    let mut dirs = rays_from_point((4, 4), st.into());
    for (x, y) in dirs.next().expect("failed to get x+") {
        panic!("should not have gotten point ({}, {}) in x+", x, y);
    }
    for (i, (x, y)) in dirs.next().expect("failed to get x-").enumerate() {
        assert_eq!(
            (sx - i - 1, sy),
            (x, y),
            "x- equality check failed at i={}",
            i
        );
    }
    for (x, y) in dirs.next().expect("failed to get y+") {
        panic!("should not have gotten point ({}, {}) in y+", x, y);
    }
    for (i, (x, y)) in dirs.next().expect("failed to get y-").enumerate() {
        assert_eq!(
            (sx, sy - i - 1),
            (x, y),
            "y- equality check failed at i={}",
            i
        );
    }
}
