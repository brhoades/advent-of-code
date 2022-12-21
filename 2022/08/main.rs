use anyhow::{anyhow, Result};

pub fn run(input: String) -> Result<()> {
    let vis_map = get_vismap(&input)?;

    /*
    println!("visibility map:");
    for x in 0..vis_map.len() {
        for y in 0..vis_map.len() {
            if *vis_map.get(x).unwrap().get(y).unwrap() {
                print!("t")
            } else {
                print!("f")
            }
        }
        println!("")
    }
    */

    println!("number of visible trees: {}", count_vismap(&vis_map));

    let scenic_map = scenic_score_map(&input)?;

    Ok(())
}

// calculate how many trees are visible from a position
fn scenic_score_map(input: &String) -> Result<Vec<Vec<u32>>> {
    Ok(vec![])
}

fn parse_trees(input: &str) -> Result<Vec<Vec<u8>>> {
    input
        .split("\n")
        .filter(|row| *row != "")
        .map(|row| {
            row.split("")
                .filter(|col| *col != "")
                .map(|v| {
                    v.parse()
                        .map_err(|e| anyhow!("failed to parse '{}': {}", v, e))
                })
                .collect::<Result<Vec<u8>>>()
        })
        .collect::<Result<Vec<_>>>()
}

fn get_vismap(input: &str) -> Result<Vec<Vec<bool>>> {
    let grid = parse_trees(input)?;
    // array of bits for whether a tree is visible
    let mut vis_map: Vec<Vec<bool>> = Vec::with_capacity(grid.len());
    let width = grid.iter().map(|x| x.len()).max().unwrap();
    for _ in 0..grid.len() {
        let mut row = vec![];
        row.resize(width, false);
        vis_map.push(row);
    }

    for dir in iter_dirs(&grid, None) {
        let mut last: Option<u8> = None;
        for ((x, y), tree) in dir {
            let vm_row = vis_map.get_mut(y).unwrap();
            match last {
                Some(l) if l >= tree => continue,
                _ => (),
            }

            *vm_row
                .get_mut(x)
                .expect(&format!("could not fetch ({},{}) from vismap", x, y)) = true;
            last = Some(tree);
        }
    }

    Ok(vis_map)
}

// iter_dirs returns a directional iterator which walks the 2-d grid
// from a point in all four directions, row-by-row and column-by-column.
// It does not include the source point in iteration.
// If None is provided for src, iteration is done for all rows and columns from the edge.
fn iter_dirs(grid: &Vec<Vec<u8>>, src: Option<(usize, usize)>) -> DirectionalGridIterator {
    let width = grid.get(0).unwrap().len();
    println!("dimens: [{}, {}]", width, grid.len());
    let (sx, sy) = match src {
        Some((x, y)) => (x, y),
        None => (0, 0),
    };

    // 1 2 3 4 |
    // 4 3 2 1 v
    // -->
    let pos_xy_iter = grid
        .iter()
        .enumerate()
        .filter(|(y, _)| src.is_none() || y > &sy)
        .map(|(y, row)| {
            let iter = row
                .iter()
                .enumerate()
                .filter(|(x, _)| src.is_none() || x > &sx)
                .map(|(x, cell)| ((x, y), cell.clone()))
                .collect::<Vec<_>>();
            iter.into_iter()
        })
        .map(RayIterator::from_iter);
    // 1 2 3 4  |
    // 4 3 2 1  v
    //    <---
    let neg_xy_iter = grid
        .iter()
        .enumerate() //
        .rev()
        .filter(|(y, _)| src.is_none() || y > &sy)
        .map(|(y, row)| {
            let iter = row
                .iter()
                .enumerate()
                .rev()
                .filter(|(x, _)| src.is_none() || x < &sx)
                .map(|(x, cell)| ((x, y.clone()), cell.clone()))
                .collect::<Vec<_>>();
            iter.into_iter()
        })
        .map(RayIterator::from_iter);

    let mut pos_yx_iter = Vec::with_capacity(grid.len());
    let mut neg_yx_iter = Vec::with_capacity(grid.len());
    for x in 0..width {
        // 1 2 3 4 |
        // 4 3 2 1 v
        // <----

        let sy = src.map(|coords| coords.1).unwrap_or(width);
        if src.is_none() || x < sx {
            pos_yx_iter.push(RayIterator::from_iter((0..sy).into_iter().map(|y| {
                (
                    (x, y),
                    grid.get(y)
                        .and_then(|row| row.get(x))
                        .expect(&format!(
                            "failed to get row/col in pos yx iter: ({}, {})",
                            x, y
                        ))
                        .clone(),
                )
            })));
        }

        if src.is_none() || x > sx {
            // 1 2 3 4 ^
            // 4 3 2 1 |
            // ---->

            let sy = src.map(|coords| coords.1 + 1).unwrap_or(0);
            neg_yx_iter.push(RayIterator::from_iter(
                (sy..grid.len()).into_iter().rev().map(|y| {
                    (
                        (x, y),
                        grid.get(y)
                            .and_then(|row| row.get(x))
                            .expect(&format!(
                                "failed to get row/col in neg yx iter: ({}, {})",
                                x, y
                            ))
                            .clone(),
                    )
                }),
            ));
        }
    }

    let mut iters: Vec<RayIterator> = pos_xy_iter.chain(neg_xy_iter).collect();
    iters.extend(pos_yx_iter);
    iters.extend(neg_yx_iter);

    DirectionalGridIterator { iters: iters }
}

struct DirectionalGridIterator {
    iters: Vec<RayIterator>,
}

impl Iterator for DirectionalGridIterator {
    type Item = RayIterator;

    fn next(&mut self) -> Option<Self::Item> {
        self.iters.pop()
    }
}

// RayIterator iterates over trees in a single direction.
struct RayIterator {
    // stored in reverse so we just pop
    items: Vec<((usize, usize), u8)>,
}

impl RayIterator {
    fn from(mut v: Vec<((usize, usize), u8)>) -> Self {
        v.reverse();
        RayIterator { items: v }
    }

    fn from_iter<I: Iterator<Item = ((usize, usize), u8)>>(i: I) -> Self {
        Self::from(i.collect())
    }
}

impl Iterator for RayIterator {
    // ((absolute coords), tree_height)
    type Item = ((usize, usize), u8);

    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }
}

fn count_vismap<T: std::borrow::Borrow<Vec<Vec<bool>>>>(vismap: T) -> usize {
    vismap
        .borrow()
        .iter()
        .flat_map(|vec| vec.iter().filter(|cell| **cell))
        .count()
}

#[test]
fn test_basic_vis() {
    let test = r#"44444
41114
42224
43334
44444"#;

    assert_eq!(16, get_vismap(test).map(count_vismap).unwrap());
}

// tests a case where some taller trees are in the way in some
// directions
#[test]
fn test_taller_vis() {
    let test = r#"44444
46514
42564
46534
44444"#;

    assert_eq!(22, get_vismap(test).map(count_vismap).unwrap());
}

// tests a case where trees sawtooth in size and can't be seen in other dirs
#[test]
fn test_sawtooth_vis() {
    let test = r#"99999
96514
92564
96534
99999"#;

    assert_eq!(21, get_vismap(test).map(count_vismap).unwrap());
}

// test border trees are always visible
#[test]
fn test_border_varying_vis() {
    let test = r#"87654
90004
80003
70002
61231"#;

    assert_eq!(13, get_vismap(test).map(count_vismap).unwrap());
}
