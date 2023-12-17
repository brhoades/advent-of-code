mod map;

use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

use advent_of_code::prelude::*;
use map::{Coords, DistanceMap, Map, Steps, TileKind, TileKindMap};

use crate::year_2023::day_10::map::MainLoopNeighbors;

use self::map::Tile;

pub fn run(input: String) -> Result<()> {
    let m: Map = input.parse()?;
    println!("map:\n{m}");
    let dm = DepthFirstVisited::distance_map(&m);

    println!("depth map:\n{dm}");

    let (coords, n) = DepthFirstVisited::furthest_point(&m);
    println!("furthest: {coords:?} with {n} steps");

    let imap = m.interior_space_map();
    println!("{imap}");
    let interior_nodes = m.interior_spaces();
    println!("interior node count: {interior_nodes}");

    Ok(())
}

trait FurthestPoint {
    fn distance_map(map: &Map) -> DistanceMap;

    fn furthest_point(map: &Map) -> (Coords, Steps) {
        let m = Self::distance_map(map);

        m.iter()
            .max_by(|(_, dl), (_, dr)| dl.cmp(dr))
            .map(|(c, d)| (*c, *d))
            .unwrap()
    }
}

struct DepthFirstVisited;

impl FurthestPoint for DepthFirstVisited {
    fn distance_map(map: &Map) -> DistanceMap {
        let mut visited = DistanceMap {
            height: map.height(),
            width: map.width(),
            ..Default::default()
        };

        Self::distance_map_inner(&mut visited, map, map.start().unwrap(), 0);
        visited
    }
}

impl DepthFirstVisited {
    fn distance_map_inner(
        // coords => minimum distance to reach
        visited: &mut DistanceMap,
        map: &Map,
        current: Coords,
        distance: usize,
    ) {
        if visited.get(&current).is_some_and(|c| *c < distance) {
            return;
        }
        visited.insert(current, distance);

        let node = map.get(current.0, current.1).unwrap().to_owned();
        let neighbors = node.main_loop_neighbors();

        for n in neighbors.iter() {
            let n = n.borrow();

            DepthFirstVisited::distance_map_inner(visited, map, (n.x(), n.y()), distance + 1);
        }
    }
}

trait InteriorSpace {
    // provides a mapping of Coords to the type of tile
    fn interior_space_map(&self) -> TileKindMap;
    fn interior_spaces(&self) -> usize {
        self.interior_space_map()
            .iter()
            .filter(|(_, kind)| matches!(kind, TileKind::Interior))
            .count()
    }
}

impl InteriorSpace for Map {
    // Returns a mapping of non-main loop tiles to whether they are interior (true) or exterior (false).
    fn interior_space_map(&self) -> TileKindMap {
        let points_on_pipe = self.main_loop();
        let mut faces = HashSet::<Coords>::new();
        for v in points_on_pipe {
            // We pretend the ray is slightly above the vertex when it intersects
            // the vertex of a polygon. In effect, the ray JUST misses top corners.
            match self.get(v.0, v.1).unwrap().value() {
                Tile::BottomLeftCorner => (),
                Tile::BottomRightCorner => (),
                Tile::Vertical => (),
                _ => continue,
            };
            faces.insert(v);
        }

        // The start won't be set. When finished, carry last into all
        // None values in the direction map.
        let faces = MainPipeLoopFaces(faces);

        let mut tkmap = DepthFirstVisited::distance_map(self)
            .iter()
            .map(|(coord, _)| {
                (
                    coord.to_owned(),
                    TileKind::MainLoop(*self.get(coord.0, coord.1).unwrap().value()),
                )
            })
            .collect::<TileKindMap>();

        for (coords, _) in self.iter() {
            if tkmap.contains_key(&coords) {
                continue;
            }

            let interior = self.is_point_in_polygon(&faces, coords);
            tkmap.insert(
                coords,
                if interior {
                    TileKind::Interior
                } else {
                    TileKind::Exterior
                },
            );
        }

        tkmap
    }
}

// contains in-order coordinates from walking a map's main loop
type MainPipeLoop = Vec<Coords>;

// the vertices making up the faces" on the main pipe loop, like a polygon.
struct MainPipeLoopFaces(HashSet<Coords>);

impl Deref for MainPipeLoopFaces {
    type Target = HashSet<Coords>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MainPipeLoopFaces {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for MainPipeLoopFaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self
            .iter()
            .map(|(_, y)| y)
            .max_by(|y1, y2| y1.cmp(y2))
            .copied()
            .unwrap_or_default();
        let width = self
            .iter()
            .map(|(x, _)| x)
            .max_by(|x1, x2| x1.cmp(x2))
            .copied()
            .unwrap_or_default();
        for y in (0..=height).rev() {
            for x in 0..=width {
                write!(
                    f,
                    "{}",
                    if self.0.get(&(x, y)).is_some() {
                        "X"
                    } else {
                        "."
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Map {
    // requires a single main continous loop or panics
    fn main_loop(&self) -> MainPipeLoop {
        let mut path = Vec::<Coords>::new();

        let cur = self.start().unwrap();
        let mut cur = self.get(cur.0, cur.1).unwrap().to_owned();
        let mut last = cur.clone();
        loop {
            path.push((cur.x(), cur.y()));

            let mut done = true;
            for n in cur.main_loop_neighbors().iter() {
                let ncoord = (n.borrow().x(), n.borrow().y());
                if ncoord == (last.x(), last.y()) || ncoord == *path.first().unwrap() {
                    continue;
                }
                last = cur;
                cur = n.borrow().to_owned();
                done = false;
                break;
            }

            if done {
                break;
            }
        }

        path
    }

    // Uses a ray tracing trick to determine if the point is interior. It scans in both directions,
    // requiring that both side intersect evenly to fail.
    fn is_point_in_polygon(&self, draw_dir: &MainPipeLoopFaces, point: Coords) -> bool {
        let mut last_sum = None;
        for rng in [0..point.0, (point.0 + 1)..self.width()] {
            let mut intersected = false;
            let mut sum: i32 = 0;
            for x in rng {
                match draw_dir.get(&(x, point.1)) {
                    Some(_) => sum += 1,
                    _ => continue,
                }
                intersected = true;
            }

            if !intersected {
                return false;
            }

            if sum.abs() % 2 == 0 && last_sum.is_some_and(|v| v % 2 == 0) {
                return false;
            }
            last_sum = Some(sum);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE_1: &str = r"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....";

    pub const EXAMPLE_2: &str = r"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";

    // has 2 separate loops
    pub const CUSTOM_1: &str = r"
        F---7
        L7F-J
        .LS-7
        F-J.|
        L---J";

    pub const EXAMPLE_3: &str = r"FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";

    pub const CUSTOM_2: &str = r"
        F-S-7
        |F7.|
        |||.|
        |||.|
        LJL-J";

    pub const CUSTOM_3: &str = r"
        F-S-7
        |F-7|
        ||.||
        ||.||
        LJ.LJ";

    pub const EXAMPLE_4: &str = r"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";

    #[test]
    fn test_furthest_example_1() {
        let m: Map = EXAMPLE_1.parse().unwrap();

        let (furthest, distance) = DepthFirstVisited::furthest_point(&m);
        assert_eq!(4, distance);
        assert_eq!((3, 1), furthest);

        let dm = DepthFirstVisited::distance_map(&m);
        println!("{m}");
        println!("{dm}");

        assert_eq!(
            r".....
.012.
.1.3.
.234.
.....",
            format!("{dm}").trim()
        );
    }

    #[test]
    fn test_furthest_example_2() {
        let m: Map = EXAMPLE_2.parse().unwrap();

        let (furthest, distance) = DepthFirstVisited::furthest_point(&m);
        assert_eq!(8, distance);
        assert_eq!((4, 2), furthest);

        let dm = DepthFirstVisited::distance_map(&m);
        println!("{m}");
        println!("{dm}");

        assert_eq!(
            r"..45.
.236.
01.78
14567
23...",
            format!("{dm}").trim()
        );
    }

    #[test]
    fn test_furthest_custom_1() {
        let m: Map = CUSTOM_1.parse().unwrap();
        println!("{m}");

        let (furthest, distance) = DepthFirstVisited::furthest_point(&m);
        assert_eq!(6, distance);
        assert!(furthest == (2, 0) || furthest == (2, 4));

        let dm = DepthFirstVisited::distance_map(&m);
        println!("{dm}");

        assert_eq!(
            r"45654
32123
.1012
321.3
45654",
            format!("{dm}").trim()
        );
    }

    #[test]
    fn test_interior_spaces_example_3() {
        let m: Map = EXAMPLE_3.parse().unwrap();

        println!("{}", m.interior_space_map());
        assert_eq!(10, m.interior_spaces());
    }

    #[test]
    fn test_interior_spaces_example_4() {
        let m: Map = EXAMPLE_4.parse().unwrap();

        println!("{}", m.interior_space_map());
        assert_eq!(4, m.interior_spaces());
    }

    #[test]
    fn test_interior_spaces_custom() {
        let cases = vec![(3, CUSTOM_2), (0, CUSTOM_3)];

        for (i, (expected, input)) in cases.into_iter().enumerate() {
            let m: Map = input.parse().unwrap();

            println!("{}", m.interior_space_map());
            assert_eq!(
                expected,
                m.interior_spaces(),
                "expected case {i} to have {expected} interior spaces"
            );
        }
    }

    #[test]
    fn test_main_loop_example_1() {
        let m: Map = EXAMPLE_1.parse().unwrap();

        assert_eq!(
            vec![
                (1, 3),
                (2, 3),
                (3, 3),
                (3, 2),
                (3, 1),
                (2, 1),
                (1, 1),
                (1, 2)
            ],
            m.main_loop()
        );
    }
}
