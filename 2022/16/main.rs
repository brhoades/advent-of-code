mod graph;

use std::collections::{HashMap, HashSet};
use std::convert::AsRef;
use std::fmt;

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
    max_turns: u32,
    graph: &'a Graph,
    /// player can spawn up to max_players for time cost = 4
    max_players: usize,

    // live mutated game state below
    cum_flow: u32,
    cum_rate: u32,
    turn: u32,
    /// player positions
    players: Vec<Player<'a>>,
    /// valves which are open
    open_valves: HashSet<&'a String>,
}

/// dumb brute force solver with obvious enhancements
impl<'a> Simulation<'a> {
    pub fn new(graph: &'a Graph, max_players: u32) -> Self {
        Self {
            graph,
            max_turns: 30,
            max_players: max_players as usize,

            open_valves: Default::default(),
            cum_flow: 0,
            cum_rate: 0,
            turn: 0,
            players: vec![],
        }
    }

    pub fn solve_dijkstra(&mut self) -> u32 {
        self.players
            .push(Player::new("1", self.graph.start().unwrap()));
        let nodes: Vec<&String> = self.graph.valves.values().map(|v| &v.name).collect();
        let mut visited = VisitedMap::new(nodes.as_slice());

        // we always spawn up to max_players ASAP
        if self.max_players > 2 {
            // spawn mechanics are different otherwise
            unimplemented!("only up to 2 players are supported");
        } else if self.max_players == 2 {
            let pos = self.graph.start().unwrap();
            if self.spawn_player().is_some() {
                self.players.push(Player::new("2", pos));
            }
        }

        self.solve_dijkstra_from_node(&mut visited);
        visited
            .0
            .values()
            .flat_map(|v| v.values())
            .max()
            .cloned()
            .unwrap_or_default()
    }

    /// Runs dijkstra recursively. There are three outcomes for each player:
    ///   2. open a valve
    ///   3. move to a cell
    ///   4. do nothing
    ///
    ///  Afterwards, the function call recurses in order to evaluate all remaining
    ///  possibilities from the current tile. self is assumed cloned and is not cleaned up
    ///  after.
    ///
    ///  Note: solver does not handle more than 2 max players well. It'll waste all players turns
    ///  when waiting for new players to spawn.
    pub fn solve_dijkstra_from_node(&mut self, visited: &mut VisitedMap) -> Option<()> {
        // tick: we are starting a round and actions happen first
        let done = self.tick();

        let mut change = false;
        // update visited
        for player in &self.players {
            if visited
                .get_or_upsert_if_better(&player.pos.name, &self.turn, &self.cum_flow)
                .is_some()
            {
                debug!(
                    "{}: has new best for {} on {} on turn {}",
                    player, player.pos.name, self.cum_flow, self.turn
                );
                change = true;
            }
        }

        if done {
            return Some(());
        }
        if !change {
            debug!("discarding path");
            return None;
        }

        // We try to open valves first, since that's likely the most effectual in depth-first
        // pathfinding: we move the players. Outcome #2 to do nothing is implicit
        // when we return no "good" next moves.
        for player in &mut self.players {
            let pos = player.pos;
            if player.done || pos.rate == 0 || self.open_valves.contains(&pos.name) {
                continue;
            }

            debug!("{}: open valve {} w/ rate {}", player, pos.name, pos.rate);
            self.cum_rate += pos.rate;
            self.open_valves.insert(&pos.name);
        }

        let mut change = self.players.iter().any(|p| !p.done);
        let next_flow = self.cum_flow + self.cum_rate;
        let mut ret = None; // non-none if found at least one answer
        while change {
            change = false;
            let mut cnext = self.clone();

            for player in &mut cnext.players {
                if player.done {
                    continue;
                }

                let neighbor_valves = player.pos.neighbors().iter().map(|v| &v.name).collect();
                if let Some(next) =
                    visited.get_best_next_node(&neighbor_valves, &(cnext.turn + 1), &next_flow)
                {
                    change = true;
                    debug!(
                        "change loop found: {}: {} -> {}",
                        player, player.pos.name, next
                    );
                    player.pos = cnext.graph.valves.get(&next).unwrap();
                    player.done = true;
                } else {
                    // otherwise: we don't move, no change
                    debug!(
                        "change loop discarded: {}: {} -> {:?}",
                        player, player.pos.name, neighbor_valves
                    );
                }
            }

            if cnext.solve_dijkstra_from_node(visited).is_some() {
                debug!(
                    "solver returned:\n{}\nturn:{}\nflow: {}\nrate: {}\nplayers: {}",
                    cnext.graph,
                    cnext.turn,
                    cnext.cum_flow,
                    cnext.cum_rate,
                    cnext.players.iter().fold("".to_string(), |acc, p| format!(
                        "{}\n  {} @ valve {}",
                        acc, p, p.pos.name
                    ))
                );
                ret = Some(());
            }
        }

        // So finally, try just not moving anywhere (after turning dials) if
        // not been done at least by one player.
        let just_recurse = self.players.iter().any(|player| {
            let neighbor_valves = player.pos.neighbors().iter().map(|v| &v.name).collect();
            visited
                .get_best_next_node(&neighbor_valves, &self.turn, &next_flow)
                .is_some()
        });

        if just_recurse {
            debug!("solver is not moving");
            if self.solve_dijkstra_from_node(visited).is_some() {
                return Some(());
            } else {
                debug!("solver did not move and it failed to bottom out");
                return ret;
            }
        }

        debug!("solver fallthrough with just_recurse false");
        ret
    }

    /// ticks once:
    ///   * tracking turns
    ///   * updating running flow
    ///   * resetting player moved status
    ///   * returning true when done
    pub fn tick(&mut self) -> bool {
        self.turn += 1;
        self.cum_flow += self.cum_rate;
        for p in &mut self.players {
            p.done = false;
        }

        self.turn > self.max_turns
    }

    /// spawn a player on the passed valve. Automatically ticks.
    /// If time runs out before spawn, reutrns None.
    fn spawn_player(&mut self) -> Option<()> {
        for _ in 0..4 {
            if self.tick() {
                return None;
            }
        }

        Some(())
    }
}

/// specialized HashMap for Simulation. Maps a neighbor node
/// to a by-turn "best" score tracker.
#[derive(Debug, Default)]
pub struct VisitedMap(HashMap<String, HashMap<u32, u32>>);

impl VisitedMap {
    /// new returns a sparsely allocated map.
    #[allow(dead_code)]
    pub fn new<S: AsRef<str>>(neighbors: &[S]) -> Self {
        Self(
            neighbors
                .iter()
                .map(|s| s.as_ref().to_string())
                .map(|n| (n, Default::default()))
                .collect(),
        )
    }

    /// new returns a densely allocated map with neighbors plugged in.
    #[allow(dead_code)]
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
            .map(|valve| (valve, self.0.get(valve.as_str()).and_then(|t| t.get(turn))))
            .filter_map(|(valve, last_best)| {
                match last_best {
                    None => Some((valve, *score)), // never moved there before
                    Some(b) if score > b => Some((valve, score - b)),
                    Some(_) => None, // no moves where we are an improvement
                }
            })
            .max_by_key(|t| t.1)
            .map(|(v, _)| v.clone().clone())
    }
}

#[derive(Debug, Clone)]
struct Player<'a> {
    pub pos: &'a Valve,
    pub done: bool,
    pub name: String,
}

impl<'a> Player<'a> {
    pub fn new(name: &str, pos: &'a Valve) -> Self {
        Self {
            pos,
            done: false,
            name: name.to_string(),
        }
    }
}

impl<'a> fmt::Display for Player<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "player {}", self.name)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn init() {
        pretty_env_logger::init();
    }

    #[test]
    fn test_solve_pt1_ex() {
        init();
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
