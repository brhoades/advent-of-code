use super::map::{Map, Tile, Tile::*, VisitedMap};

pub fn find_start(m: &Map<Tile>) -> (usize, usize) {
    find(m, |t| *t == Start).pop().unwrap()
}

pub fn find_end(m: &Map<Tile>) -> (usize, usize) {
    find(m, |t| *t == End).pop().unwrap()
}

pub fn find<F: Fn(&Tile) -> bool>(m: &Map<Tile>, heuristic: F) -> Vec<(usize, usize)> {
    m.iter_rows()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, t)| heuristic(t))
                .map(|(x, _)| (x, y))
                .collect::<Vec<_>>()
        })
        .collect()
}

// recursively calls self to find shortest path
#[allow(dead_code)]
pub fn find_shortest_path_brute(m: &Map<Tile>) -> Option<VisitedMap> {
    let mut visited = VisitedMap::new(m.dimensions.0, m.dimensions.1);

    // pull out the start tile with a scan
    let (x, y) = find_start(m);

    // walk results and return the one with the lowest cost
    let mut shortest = None;
    find_shortest_path_brute_inner(m, &mut visited, (x, y), &mut shortest)
        .into_iter()
        .min_by_key(|path| path.score())
}

// visited is x => y => bool
// returns a list of path vecs
#[allow(dead_code)]
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
            let v = visited.clone();
            visited.unset(pos.0, pos.1).unwrap();
            return vec![v];
        }
        Start => 0, // coming from start, it is 'a' == 0 cost
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
            Start => true, // S == a == 0 cost
            Walkable(c) => (current_cost as i16 - *c as i16) >= -1,
            End => (current_cost as i16 - ('z' as u8 - 'a' as u8) as i16) >= -1,
        })
        .collect::<Vec<_>>();

    let mut results = vec![];
    for step in steps {
        for p in find_shortest_path_brute_inner(m, visited, step, shortest) {
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

pub fn find_shortest_path_dijkstra_from(m: &Map<Tile>, x: usize, y: usize) -> Option<VisitedMap> {
    let mut path = VisitedMap::new(m.dimensions.0, m.dimensions.1);
    let mut visited = Map::<Option<usize>>::new_dense(m.dimensions.0, m.dimensions.1);

    let end = find_end(m);

    // walk paths and return the one with the lowest cost
    find_shortest_path_dijkstra_inner(m, (x, y), end, &mut path, &mut visited)
        .into_iter()
        .min_by_key(|path| path.score())
}

// recursively calls self to find shortest path
pub fn find_shortest_path_dijkstra(m: &Map<Tile>) -> Option<VisitedMap> {
    let st = find_start(m);
    find_shortest_path_dijkstra_from(m, st.0, st.1)
}

// visited is x => y => bool
// returns a list of path vecs
fn find_shortest_path_dijkstra_inner(
    m: &Map<Tile>,
    pos: (usize, usize),
    end: (usize, usize),
    path: &mut VisitedMap,
    visited: &mut Map<Option<usize>>,
) -> Vec<VisitedMap> {
    let (x, y) = pos;

    path.set(x, y).unwrap();

    let current_cost = match m.get(x, y).unwrap() {
        End => {
            // DONE! Clone the visited and return it up so it bubbles to our
            // caller.
            let v = path.clone();
            path.unset(x, y).unwrap();
            return vec![v];
        }
        Start => 0, // coming from start, it is 'a' == 0 cost
        Walkable(c) => *c,
    };

    let mut choices = vec![];
    if x != 0 && !path.get(x - 1, y).unwrap() {
        choices.push((x - 1, y));
    }
    if x != m.dimensions.0 - 1 && !path.get(x + 1, y).unwrap() {
        choices.push((x + 1, y));
    }
    if y != 0 && !path.get(x, y - 1).unwrap() {
        choices.push((x, y - 1));
    }
    if y != m.dimensions.1 - 1 && !path.get(x, y + 1).unwrap() {
        choices.push((x, y + 1));
    }

    let steps = choices
        .into_iter()
        .filter(|(x, y)| !*path.get(*x, *y).unwrap())
        .filter(|(x, y)| match m.get(*x, *y).unwrap() {
            Start => true, // S == a == 0 cost
            Walkable(c) => (current_cost as i16 - *c as i16) >= -1,
            End => (current_cost as i16 - ('z' as u8 - 'a' as u8) as i16) >= -1,
        })
        .collect::<Vec<_>>();

    let mut results = vec![];
    for step in steps {
        match visited.get_mut(step.0, step.1).unwrap() {
            Some(cost) if path.score() + 1 >= *cost => {
                // println!("{} >= {}", path.score() + 1, *cost);
                continue;
            }
            Some(ref mut cost) if path.score() + 1 < *cost => {
                // println!("better cost: {} < {}", path.score() + 1, cost);
                *cost = path.score() + 1;
            }
            other => *other = Some(path.score() + 1),
        }

        results.extend(find_shortest_path_dijkstra_inner(
            m, step, end, path, visited,
        ))
    }

    // shared with parent. undo modification
    path.unset(x, y).unwrap();
    results
}
