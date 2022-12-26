use super::map::{Map, Tile, Tile::*, VisitedMap};

// recursively calls self to find shortest path
pub fn find_shortest_path_brute(m: &Map<Tile>) -> Option<VisitedMap> {
    let mut visited = VisitedMap::new(m.dimensions.0, m.dimensions.1);

    // pull out the start tile with a scan
    let (x, y) = m
        .iter_rows()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, t)| **t == Start)
                .map(|(x, _)| (x, y))
                .collect::<Vec<_>>()
        })
        .next()
        .unwrap();

    // walk results and return the one with the lowest cost
    let mut shortest = None;
    find_shortest_path_brute_inner(m, &mut visited, (x, y), &mut shortest)
        .into_iter()
        .min_by_key(|path| path.score())
}

// visited is x => y => bool
// returns a list of path vecs
fn find_shortest_path_brute_inner(
    m: &Map<Tile>,
    visited: &mut VisitedMap,
    pos: (usize, usize),
    shortest: &mut Option<usize>, // minor optimization. Once a path is found, don't descend into paths which are larger
) -> Vec<VisitedMap> {
    visited.set(pos.0, pos.1).unwrap();
    let current_cost = match m.get(pos.0, pos.1).unwrap() {
        End => {
            // DONE! Clone the visited and return it up so it bubbles to our
            // caller.
            return vec![visited.clone()];
        }
        Start => 255, // coming from start, can go to any tile
        Walkable(c) => *c,
    };

    if let Some(shortest) = shortest {
        if *shortest < visited.score() + 1 {
            visited.unset(pos.0, pos.1).unwrap();
            return vec![]; // don't set, just bail, we can't do better unless this is E (checked above)
        }
    }

    let mut choices = vec![];
    if pos.0 != 0 {
        choices.push((pos.0 - 1, pos.1));
    }
    if pos.0 != m.dimensions.0 - 1 {
        choices.push((pos.0 + 1, pos.1));
    }
    if pos.1 != 0 {
        choices.push((pos.0, pos.1 - 1));
    }
    if pos.1 != m.dimensions.1 - 1 {
        choices.push((pos.0, pos.1 + 1));
    }

    let steps = choices
        .into_iter()
        .filter(|(x, y)| !*visited.get(*x, *y).unwrap())
        .filter(|(x, y)| match m.get(*x, *y).unwrap() {
            Start => unreachable!("evaluation error ({}, {}) => ({}, {}): shouldn't be able to visit start again, is visited", pos.0, pos.1, x, y),
            Walkable(c) => current_cost as i16 - *c as i16 >= -1,
            End => true,
        })
        .collect::<Vec<_>>();

    println!("{:?} => {:?}:\n{}", (pos.0, pos.1), steps, visited);
    let mut results = vec![];
    for step in steps {
        for mut p in find_shortest_path_brute_inner(m, visited, step, shortest) {
            match shortest {
                Some(l) if *l > p.score() => *shortest = Some(p.score()),
                None => *shortest = Some(p.score()),
                _ => (),
            }
            results.push(p);
        }
    }

    // shared with parent. undo modification
    visited.unset(pos.0, pos.1).unwrap();
    results
}
