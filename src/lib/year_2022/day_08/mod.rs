mod iter;

use anyhow::{anyhow, Result};

use iter::rays_from_point;
use std::iter::repeat;

pub fn run(input: String) -> Result<()> {
    let vis_map = get_vismap(&input)?;

    println!("visibility map:");
    for x in 0..vis_map.len() {
        for y in 0..vis_map.len() {
            if *vis_map.get(x).unwrap().get(y).unwrap() {
                print!("t")
            } else {
                print!("f")
            }
        }
        println!()
    }

    println!("number of visible trees: {}", count_vismap(&vis_map));

    let scenic_map = scenic_score_map(&input)?;
    let max: ((usize, usize), u32) = scenic_map
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(move |(y, score)| ((x, y), *score))
        })
        .max_by_key(|(_, s)| *s)
        .unwrap();

    println!("scenic score map:");
    for x in 0..vis_map.len() {
        for y in 0..vis_map.len() {
            print!("({:>3})", scenic_map.get(x).unwrap().get(y).unwrap());
        }
        println!()
    }

    println!(
        "tree with highest scenic score has {} at ({}, {})",
        max.1, max.0 .0, max.0 .1
    );

    Ok(())
}

fn parse_trees(input: &str) -> Result<Vec<Vec<u8>>> {
    input
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.split("")
                .filter(|col| !col.is_empty())
                .map(|v| {
                    v.parse()
                        .map_err(|e| anyhow!("failed to parse '{}': {}", v, e))
                })
                .collect::<Result<Vec<u8>>>()
        })
        .collect::<Result<Vec<_>>>()
}

// get vismap returns a truth map of what trees are visible from
// the edge by !! (y => x => true)
fn get_vismap(input: &str) -> Result<Vec<Vec<bool>>> {
    let grid = parse_trees(input)?;

    // array of bits for whether a tree is visible
    let mut vis_map: Vec<Vec<bool>> = Vec::with_capacity(grid.len());
    let width = grid.iter().map(|x| x.len()).max().unwrap();
    let dimens = (width as usize, grid.len() as usize);

    for _ in 0..grid.len() {
        let mut row = vec![];
        row.resize(width, false);
        vis_map.push(row);
    }

    let edgepoints = (0..grid.len())
        .zip(repeat(0))
        .chain(repeat(0).zip(1..width))
        .chain(repeat(width - 1).zip(1..grid.len()))
        .chain((1..width).zip(repeat(grid.len() - 1)))
        .map(Into::into)
        .collect::<Vec<iter::Coordinate>>();

    for src in edgepoints {
        let start = grid.get(src.y).unwrap().get(src.x).unwrap();
        *vis_map.get_mut(src.y).unwrap().get_mut(src.x).unwrap() = true;

        // walk the edges and then ray out.
        for dir in rays_from_point(dimens, src) {
            let mut last = Some(start);

            for (x, y) in dir {
                let tree = grid
                    .get(y)
                    .unwrap_or_else(|| panic!("failed to get y={}", y))
                    .get(x)
                    .unwrap_or_else(|| panic!("failed to get x @ ({}, {})", x, y));

                // println!(
                //     "({}, {}) ==> ({}, {}): {} >= {:?}",
                //     src.x, src.y, x, y, tree, last
                // );
                match last {
                    Some(l) if l >= tree => continue,
                    _ => (),
                }

                let vm_row = vis_map.get_mut(y).unwrap();
                *vm_row
                    .get_mut(x)
                    .unwrap_or_else(|| panic!("could not fetch ({},{}) from vismap", x, y)) = true;
                last = Some(tree);
            }
        }

        println!("===");
    }

    /*
    println!("=================================");
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
    */

    Ok(vis_map)
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
    let answer = r#"ttttt
tttft
tfttt
tttft
ttttt"#;

    assert_trees_eq(answer, get_vismap(test).unwrap());
}

// tests a tiny case where the middle tree is hidden
#[test]
fn test_tiny_hidden() {
    let test = r#"999
919
999"#;
    let answer = r#"ttt
tft
ttt"#;

    assert_trees_eq(answer, get_vismap(test).unwrap());
}

#[cfg(test)]
fn assert_trees_eq<T: std::borrow::Borrow<Vec<Vec<bool>>>>(expected: &str, actual: T) {
    let res = actual
        .borrow()
        .iter()
        .map(|row| {
            row.iter()
                .map(|t| if *t { "t" } else { "f" })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("expected:\n{}\n\nactual:\n{}", expected, res);
    assert_eq!(expected, res);
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
    let expected = r#"ttttt
tffft
tffft
tffft
ttttt"#;
    let res = get_vismap(test).unwrap();
    assert_trees_eq(expected, res);
    assert_eq!(16, get_vismap(test).map(count_vismap).unwrap());
}

// Pt 2 follows. it bad.

// calculate how many trees are visible from a position
fn scenic_score_map(input: &str) -> Result<Vec<Vec<u32>>> {
    let grid = parse_trees(input)?;

    let mut scenic_map: Vec<Vec<u32>> = Vec::with_capacity(grid.len());
    let width = grid.iter().map(|x| x.len()).max().unwrap();
    for _ in 0..grid.len() {
        scenic_map.push(vec![0; width]);
    }
    let dimens = (width, grid.len());

    // O(N^2) yikes. optimizing this after writing a dumb pt 1 feels even worse :D
    for y in 0..grid.len() {
        for x in 0..width {
            let mut vis = 1;
            let our_size = grid.get(y).unwrap().get(x).unwrap();

            for dir in rays_from_point(dimens, (x, y).into()) {
                // product of all directions, sum this line
                let mut line = 0;
                for (tx, ty) in dir {
                    let t = grid.get(ty).unwrap().get(tx).unwrap();

                    line += 1;
                    if t >= our_size {
                        break;
                    }
                }

                if line != 0 {
                    vis *= line;
                }
            }

            let row = scenic_map.get_mut(y).unwrap();
            *row.get_mut(x).unwrap() = vis;
        }
    }

    Ok(scenic_map)
}

#[test]
fn minimal_scenic_score_lines() {
    let test = r#"12
34"#;
    let expected = vec![vec![1, 1], vec![1, 1]];

    assert_scenic_tree_eq(expected, scenic_score_map(test).unwrap());
}

#[test]
fn fourxfour_blind_score_lines() {
    let test = r#"1234
1234
1234
1234"#;
    let expected = vec![
        vec![1, 1, 2, 3],
        vec![1, 1, 2, 3],
        vec![1, 1, 2, 3],
        vec![1, 1, 2, 3],
    ];

    assert_scenic_tree_eq(expected, scenic_score_map(test).unwrap());
}

#[test]
fn fourxfour_big_trees() {
    let test = r#"1243
9234
1234
9284"#;
    let expected = vec![
        vec![1, 1, 6, 1],
        vec![6, 1, 2, 3],
        vec![1, 1, 2, 3],
        vec![6, 1, 6, 1],
    ];

    assert_scenic_tree_eq(expected, scenic_score_map(test).unwrap());
}

#[test]
fn test_examples_scenic() {
    let test = r#"30373
25512
65332
33549
35390"#;

    let scores = scenic_score_map(test).unwrap();
    assert_eq!(4, *scores.get(1).unwrap().get(2).unwrap());

    let test = r#"30373
25512
65332
33549
35390"#;
    let scores = scenic_score_map(test).unwrap();
    assert_eq!(8, *scores.get(3).unwrap().get(2).unwrap());
}

#[cfg(test)]
fn assert_scenic_tree_eq(expected: Vec<Vec<u32>>, actual: Vec<Vec<u32>>) {
    let zip_rows = expected.iter().enumerate().zip(actual.iter());
    let mut failed: Option<(usize, usize, u32, u32)> = None;
    for ((ey, erow), arow) in zip_rows {
        if failed.is_some() {
            break;
        }
        for ((ex, esc), asc) in erow.iter().enumerate().zip(arow.iter()) {
            if esc != asc {
                failed = Some((ex, ey, *esc, *asc));
                break;
            }
        }
    }

    if let Some((fx, fy, esc, asc)) = failed {
        println!("expected:");
        for erow in expected {
            for score in erow {
                print!("({:>3})", score);
            }
            println!()
        }

        println!("\nactual:");
        for arow in actual {
            for score in arow {
                print!("({:>3})", score);
            }
            println!()
        }

        println!(
            "score at ({}, {}) failed equality: {} != {}",
            fx, fy, esc, asc
        );
    }

    assert_eq!(None, failed);
}
