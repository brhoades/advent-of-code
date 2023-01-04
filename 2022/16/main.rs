mod graph;

use std::collections::{HashMap, HashSet};
use std::convert::AsRef;

use graph::{Graph, Valve};

use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    let g: Graph = input.parse()?;

    let mut solver = Simulation::new(&g, 1);

    println!("max flow found: {}", solver.solve_dijkstra());

    Ok(())
}

/// SimState captures the state of the simulation in the moment.
#[derive(Clone, Debug)]
pub struct Simulation<'a> {
    /// cumulative flow so far
    pub cum_flow: u32,
    pub cum_rate: u32,
    pub turn: u32,
    pub max_turns: u32,
    /// player can spawn up to max_players for time cost = 4
    pub max_players: usize,
    /// valves which are open. not stored in Graph to reduce memory use, maybe probably
    pub open_valves: HashSet<&'a String>,
    pub graph: &'a Graph,
}

/// dumb brute force solver with obvious enhancements
impl<'a> Simulation<'a> {
    pub fn new(graph: &'a Graph, max_players: u32) -> Self {
        Self {
            graph,
            open_valves: Default::default(),
            cum_flow: 0,
            cum_rate: 0,
            turn: 0,
            max_turns: 30,
            max_players: max_players as usize,
        }
    }

    pub fn solve_dijkstra(&mut self) -> u32 {
        let st = self.graph.start().unwrap();
        let nodes: Vec<&String> = self.graph.valves.values().map(|v| &v.name).collect();
        let mut visited = VisitedMap::new_dense(nodes.as_slice(), self.max_turns);

        self.solve_dijkstra_from_node(&mut visited, vec![st]);
        visited
            .0
            .values()
            .flat_map(|v| v.values())
            .max()
            .cloned()
            .unwrap_or_default()
    }

    /// Runs dijkstra recursively. There are three outcomes for each player:
    ///   1. spawn new player
    ///   2. open a valve
    ///   3. move to a cell
    ///   4. do nothing
    ///
    ///  Afterwards, the function call recurses in order to evaluate all remaining
    ///  possibilities from the current tile.
    ///
    ///  Note: solver does not handle more than 2 max players well. It'll waste all players turns
    ///  when waiting for new players to spawn.
    pub fn solve_dijkstra_from_node(
        &mut self,
        visited: &mut VisitedMap,
        mut player_pos: Vec<&Valve>,
    ) -> Option<()> {
        // tick: we are doing a round and actions are late
        if self.tick() {
            return Some(());
        }

        // update visited
        for pos in &player_pos {
            visited.get_or_upsert_if_better(&pos.name, &self.turn, &self.cum_flow);
        }

        // Always try to spawn players if we can. Recurse on the possiblity of doing that
        // and later on the possibility of not. This function which did the spawning,
        // (w/ self, not c) will continue without.
        if self.max_turns - self.turn > 5 && player_pos.len() < self.max_players {
            let mut c = self.clone();
            for _ in 0..4 {
                if c.tick() {
                    return Some(());
                }
            }

            let pos = player_pos.first().unwrap();
            return c.solve_dijkstra_from_node(visited, vec![pos, pos]);
        }

        let mut c = self.clone();
        // We try to open valves first, since that's likely the most effectual in depth-first
        // pathfinding: we move the players. Outcome #2 to do nothing is implicit
        // when we return no "good" next moves.
        let mut rem_pos = vec![];
        for pos in player_pos {
            if pos.rate == 0 || self.open_valves.contains(&pos.name) {
                rem_pos.push(pos);
                continue;
            }

            // println!("third check: {}\t{} > {:?}", v.name, self.cum_flow, vamount);
            c.cum_rate += pos.rate;
            c.open_valves.insert(&pos.name);
        }
        player_pos = rem_pos;

        let mut change = true;
        let next_flow = self.cum_flow + self.cum_rate;
        let mut ret = None; // non-none if found at least one answer
        while change && player_pos.len() > 0 {
            change = false;
            let mut next_pos = vec![];

            for pos in &player_pos {
                if let Some(next) = visited.get_best_next_node(
                    &pos.neighbors().iter().map(|v| &v.name).collect(),
                    &self.turn,
                    &next_flow,
                ) {
                    change = true;
                    next_pos.push(self.graph.valves.get(&next).unwrap());
                } else {
                    next_pos.push(pos);
                }
            }

            if c.clone()
                .solve_dijkstra_from_node(visited, next_pos)
                .is_some()
            {
                ret = Some(());
            }
        }

        // so finally, try just not moving anywhere (after turning dials) if it's OK
        if player_pos.iter().all(|p| {
            visited
                .get_best_next_node(
                    &p.neighbors().iter().map(|n| &n.name).collect(),
                    &self.turn,
                    &next_flow,
                )
                .is_some()
        }) {
            return c.solve_dijkstra_from_node(visited, player_pos);
        }
        ret
    }

    // ticks once and returns true if done
    pub fn tick(&mut self) -> bool {
        self.turn += 1;
        self.cum_flow += self.cum_rate;

        self.turn > self.max_turns
    }
}

/// specialized HashMap for Simulation. Maps a neighbor node
/// to a by-turn "best" score tracker.
#[derive(Debug, Default)]
pub struct VisitedMap(HashMap<String, HashMap<u32, u32>>);

impl VisitedMap {
    /// new returns a sparsely allocated map.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }

    /// new returns a densely allocated map with neighbors plugged in.
    pub fn new_dense<S: AsRef<str>>(neighbors: &[S], turns: u32) -> Self {
        Self(
            neighbors
                .iter()
                .map(|s| s.as_ref().to_string())
                .map(|n| (n, (0..turns).map(|t| (t as u32, 0)).collect()))
                .collect(),
        )
    }

    /// gets or upserts the neighbor's value on this turn. If the neighbor has not been visited
    /// on this turn or it has and our score is better, it's replaced and the old value is returned.
    pub fn get_or_upsert_if_better(
        &mut self,
        neighbor: &String,
        turn: &u32,
        score: &u32,
    ) -> Option<u32> {
        let mut ret = None;
        let turns = self.0.entry(neighbor.to_string()).or_insert_with(|| {
            ret = Some(*score);
            [(*turn, score.clone())].into_iter().collect()
        });

        let v = turns.entry(*turn).or_insert_with(|| {
            ret = Some(*score);
            *turn
        });

        if ret.is_some() {
            return ret;
        }

        if *v > *score {
            None
        } else {
            ret = Some(*v);
            *v = *score;
            ret
        }
    }

    /// Provides the next best node from the provided list of nodes on a given turn
    /// evaluated against a score. Among equal options, the "first" wins.
    pub fn get_best_next_node(
        &mut self,
        valves: &Vec<&String>,
        turn: &u32,
        score: &u32,
    ) -> Option<String> {
        // turn valves into (name, score_delta) and filter only those where our score
        // is greater
        valves
            .iter()
            .map(|valve| {
                (
                    valve.clone(),
                    self.0.get(valve.as_str()).and_then(|t| t.get(turn)),
                )
            })
            .filter_map(|(valve, last_best)| {
                match last_best {
                    None => Some((valve, *score)), // never moved there before
                    Some(b) if score > b => Some((valve, score - b)),
                    Some(_) => None, // no moves where we are an improvement
                }
            })
            .max_by_key(|t| t.1)
            .map(|(v, _)| v.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_pt1_ex() {
        let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

        let graph: Graph = input.parse().unwrap();
        let mut solver = Simulation::new(&graph, 1);

        println!("{}", graph);

        assert_eq!(1651, solver.solve_dijkstra());
    }
}
