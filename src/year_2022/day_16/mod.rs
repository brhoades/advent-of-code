/// This solution is messy. Took a long time to get here and I've not got
/// the energy in me for a refactor. There's tons of extra clones which could be
/// unwound, especially around the last mile changes to VisitedMap to track both
/// player's positions with a joined key.
///
/// Nonetheless: it works and solves the solution pretty fast.
mod graph;

use std::collections::{HashMap, HashSet};
use std::convert::AsRef;
use std::fmt;

use graph::{Graph, Valve};

use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    let g: Graph = input.parse()?;

    let mut solver = Simulation::new(&g, 1);

    println!("pt1: max flow found: {}", solver.solve_dijkstra());

    let mut solver = Simulation::new(&g, 2);
    println!("pt2: max flow found: {}", solver.solve_dijkstra());

    Ok(())
}

#[derive(Clone, Debug)]
pub enum Action {
    Move {
        player: usize,
        from: String,
        to: String,
    },
    Stay {
        player: usize,
        at: String,
    },
    Open {
        player: usize,
        valve: String,
    },
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Action::*;
        match self {
            Move { player, from, to } => {
                write!(f, "Player {} moved from {} to {}", player + 1, from, to)
            }
            Stay { player, at } => {
                write!(f, "Player {} stayed at {}", player + 1, at)
            }
            Open { player, valve } => {
                write!(f, "Player {} opened valve {}", player + 1, valve)
            }
        }
    }
}

impl Action {
    pub fn mv(player: usize, from: String, to: String) -> Self {
        Self::Move { player, from, to }
    }

    pub fn stay(player: usize, at: String) -> Self {
        Self::Stay { player, at }
    }

    pub fn open(player: usize, valve: String) -> Self {
        Self::Open { player, valve }
    }
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

    /// chronological actions indexed by turn
    actions: Vec<Action>,
}

/// solver which consumes itself on call. cloned recursively to avoid
/// annoying Arc semantics.
impl<'a> Simulation<'a> {
    pub fn new(graph: &'a Graph, max_players: u32) -> Self {
        Self {
            graph,
            max_turns: 30,
            max_players: max_players as usize,

            open_valves: Default::default(),
            cum_flow: 0,
            cum_rate: 0,
            turn: 1,
            players: vec![],
            actions: Vec::new(),
        }
    }

    pub fn solve_dijkstra(&mut self) -> u32 {
        self.players
            .push(Player::new("1", self.graph.start().unwrap()));
        let nodes: Vec<&String> = self.graph.valves.values().map(|v| &v.name).collect();
        let mut visited = VisitedMap::new(nodes.as_slice());

        // we always spawn up to max_players ASAP
        match self.max_players {
            // spawn mechanics are different otherwise
            3.. => unimplemented!("only up to 2 players are supported"),
            2 => {
                let pos = self.graph.start().unwrap();
                if self.spawn_player().is_some() {
                    self.players.push(Player::new("2", pos));
                }
            }
            _ => (),
        };

        let result = self.clone().solve_dijkstra_from_node(&mut visited).unwrap();
        for t in &result.actions {
            trace!("{}", t);
        }

        result.cum_flow
    }

    /// Runs dijkstra recursively. There are three outcomes for each player:
    ///   1. open a valve
    ///   2. move to a cell
    ///   3. do nothing
    ///
    ///  After making one decision, the function recurses to finish the remaining moves.
    ///  If all players are done, a tick occurs and it repeats.
    ///  self is assumed cloned and is not cleaned up after.
    ///
    /// If there are more than one players, visited are their positions joined together.
    pub fn solve_dijkstra_from_node(mut self, visited: &mut VisitedMap) -> Option<Self> {
        trace!(
            "recurse: actions: {}\tturns: {}\tplayers not done: {:?}",
            self.actions.len(),
            self.turn,
            self.players
                .iter()
                .filter(|p| !p.done)
                .map(|p| format!("{}", p))
                .collect::<Vec<_>>()
        );
        if self.players.iter().all(|p| p.done) {
            trace!("turn {}: done, ticking", self.turn);
            // tick: we are starting a round and actions happen first
            let done = self.tick();

            let mut change = false;

            if visited
                .get_or_upsert_if_better(&self.key(), &self.turn, &self.cum_flow)
                .is_some()
            {
                debug!(
                    "{}: new best with flow={} on turn {}",
                    self.key(),
                    self.cum_flow,
                    self.turn
                );
                change = true;
            }

            if done {
                debug!("done!");
                return Some(self);
            }
            if !change {
                debug!("discarding path");
                return None;
            }
        }

        if self.turn == self.max_turns {
            trace!("max turns hit");
            return Some(self);
        }

        let mut results = vec![];

        // We try to open valves first, since that's likely the most impactful in depth-first
        // pathfinding: we move the players.
        //
        // We evaluate the possibility of opening valves and not by recursing after each
        // action and continuing top-level.
        for (i, player) in self.players.iter().enumerate() {
            let pos = player.pos;
            if player.done || pos.rate == 0 || self.open_valves.contains(&pos.name) {
                continue;
            }

            let mut c = self.clone();

            debug!("{}: open valve {} w/ rate {}", player, pos.name, pos.rate);
            c.open(i);
            if let Some(res) = c.solve_dijkstra_from_node(visited) {
                results.push(res);
            }
        }

        let players = self
            .players
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if !p.done {
                    Some((i, &p.name, p.pos))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let next_flow = self.cum_flow + self.cum_rate;

        for (i, name, pos) in &players {
            let neighbor_keys: Vec<String> = if self.max_players == 1 {
                pos.neighbors().iter().map(|v| v.name.clone()).collect()
            } else if *i == 0 {
                pos.neighbors()
                    .iter()
                    .map(|v| v.name.clone() + &self.players.get(1).unwrap().pos.name)
                    .collect()
            } else {
                pos.neighbors()
                    .iter()
                    .map(|v| self.players.get(1).unwrap().pos.name.clone() + &v.name)
                    .collect()
            };

            let neighbor_keys = neighbor_keys.iter().collect();

            let next_nodes =
                visited.get_next_best_nodes(&neighbor_keys, &(self.turn + 1), &next_flow);
            let next_nodes: Vec<String> = next_nodes
                .into_iter()
                .map(|n| {
                    if self.max_players != 1 {
                        if *i == 0 {
                            n[..2].to_string()
                        } else {
                            n[2..].to_string()
                        }
                    } else {
                        n
                    }
                })
                .collect();

            // wallk all possible next nodes and call ourselves again to do the same.
            for next in next_nodes {
                let mut c = self.clone();

                debug!(
                    "change loop for turn {}: player {}: {} -> {}",
                    self.turn, name, pos.name, next
                );
                c.mv(*i, &next);

                if let Some(res) = c.solve_dijkstra_from_node(visited) {
                    results.push(res);
                }
            }
        }

        // So finally, try just not moving anywhere (after turning dials) if
        // a player isn't done and hasn't moved.
        let players = self
            .players
            .iter()
            .enumerate()
            .filter_map(|(i, p)| if !p.done { Some((i, p.pos)) } else { None })
            .collect::<Vec<_>>();

        if !players.is_empty() && self.turn <= self.max_turns {
            for (i, pos) in &players {
                let player_pos = if self.max_players == 1 {
                    vec![pos.name.clone()]
                } else if *i == 0 {
                    vec![pos.name.clone() + &self.players.get(1).unwrap().pos.name]
                } else {
                    vec![self.players.get(1).unwrap().pos.name.clone() + &pos.name]
                };
                let player_pos = player_pos.iter().collect();
                if visited
                    .get_best_next_node(&player_pos, &(self.turn + 1), &next_flow)
                    .is_none()
                {
                    trace!("player {} cannot stay @ {}", i + 1, pos.name);
                    continue;
                }

                let mut c = self.clone();
                c.stay(*i);

                if let Some(res) = c.solve_dijkstra_from_node(visited) {
                    results.push(res);
                }
            }
        }

        results.into_iter().max_by_key(|r| r.cum_flow)
    }

    fn key(&self) -> String {
        self.players
            .iter()
            .fold("".to_string(), |acc, p| acc + &p.pos.name)
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

        self.turn >= self.max_turns
    }

    pub fn mv(&mut self, player: usize, to: &String) {
        let p = self.players.get_mut(player).unwrap();
        let action = Action::mv(player, p.pos.name.clone(), to.clone());

        trace!("{}", &action);
        self.actions.push(action);
        p.pos = self
            .graph
            .get(to)
            .unwrap_or_else(|| panic!("failed to retrieve destination: {}", to));
        p.done = true;
    }

    pub fn stay(&mut self, player: usize) {
        let p = self.players.get_mut(player).unwrap();
        let action = Action::stay(player, p.pos.name.clone());

        trace!("{}", &action);
        self.actions.push(action);
        p.done = true;
    }

    pub fn open(&mut self, player: usize) {
        let p = self.players.get_mut(player).unwrap();
        let action = Action::open(player, p.pos.name.clone());

        trace!("{}", &action);
        self.actions.push(action);

        self.cum_rate += p.pos.rate;
        self.open_valves.insert(&p.pos.name);
        p.done = true;
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
                .map(|n| (n, (0..turns).map(|t| (t, 0)).collect()))
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
            [(*turn, *score)].into_iter().collect()
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

    fn get_next_best_nodes_inner(
        &mut self,
        #[allow(clippy::ptr_arg)] valves: &Vec<&String>,
        turn: &u32,
        score: &u32,
    ) -> Vec<(String, u32)> {
        // turn valves into (name, score_delta) and filter only those where our score
        // is greater
        valves
            .iter()
            .map(|valve| (valve, self.0.get(valve.as_str()).and_then(|t| t.get(turn))))
            .filter_map(|(valve, last_best)| {
                match last_best {
                    None => Some((*valve, *score)), // never moved there before
                    Some(b) if score > b => Some((valve, score - b)),
                    Some(_) => None, // no moves where we are an improvement
                }
            })
            .map(|(n, score)| (n.clone(), score))
            .collect::<Vec<_>>()
    }

    pub fn get_next_best_nodes(
        &mut self,
        valves: &Vec<&String>,
        turn: &u32,
        score: &u32,
    ) -> Vec<String> {
        // turn valves into (name, score_delta) and filter only those where our score
        // is greater
        let mut valves = self.get_next_best_nodes_inner(valves, turn, score);

        valves.sort_by_key(|v| v.1);
        valves.into_iter().map(|v| v.0).collect()
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
        self.get_next_best_nodes_inner(valves, turn, score)
            .into_iter()
            .max_by_key(|t| t.1)
            .map(|(v, _)| v)
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

    #[test]
    fn test_solve_pt2_ex() {
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
        let mut solver = Simulation::new(&graph, 2);

        println!("{}", graph);

        assert_eq!(1707, solver.solve_dijkstra());
    }
}
