mod map;
mod pathings;

use anyhow::Result;

use map::{Map, Tile};

pub fn run(input: String) -> Result<()> {
    let map: Map<Tile> = input.as_str().parse()?;
    println!("evaluating map:\n{}", map);
    let path = pathings::find_shortest_path_dijkstra(&map).expect("should have found a path");
    println!("found path: \n{}", path);

    println!("====== part 2 ======");
    let shortest = pathings::find(&map, |t| *t == Tile::Walkable(0)) // a == 0 cost
        .into_iter()
        .filter_map(|(x, y)| pathings::find_shortest_path_dijkstra_from(&map, x, y))
        .min_by_key(|p| p.score());
    println!(
        "shortest path step cost: {}",
        shortest.expect("should have found a path").score()
    );

    Ok(())
}

#[test]
fn map_ex_brute() {
    let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
    let map: Map<Tile> = input.parse().expect("should have parsed the map");

    let path = pathings::find_shortest_path_brute(&map).expect("should have found a path");
    println!("{}", path);
}

#[test]
fn map_ex_dijkstra() {
    let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
    let map: Map<Tile> = input.parse().expect("should have parsed the map");

    let path = pathings::find_shortest_path_dijkstra(&map).expect("should have found a path");
    println!("{}", path);
    assert_eq!(31, path.score());
}
