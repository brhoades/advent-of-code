mod graph;

use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::convert::AsRef;
use std::rc::{Rc, Weak};

use graph::{Graph, Valve};

use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    let g: Graph = input.parse()?;

    let mut solver = Simulation::new(&g, 1);

    println!("max flow found: {}", solver.solve_dijkstra(),);

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
    pub players: u32,
    pub max_players: u32,
    /// valves which are open. not stored in Graph to reduce memory use, maybe probably
    pub open_valves: Vec<String>,
    pub open_valves_lookup: HashSet<String>,
    pub graph: &'a Graph,
}

/// dumb brute force solver with obvious enhancements
impl<'a> Simulation<'a> {
    pub fn new(graph: &'a Graph, max_players: u32) -> Self {
        Self {
            graph,
            open_valves: vec![],
            open_valves_lookup: Default::default(),
            cum_flow: 0,
            cum_rate: 0,
            turn: 0,
            max_turns: 30,
            players: 1,
            max_players,
        }
    }

    pub fn solve_dijkstra(&mut self) -> u32 {
        let st = self.graph.start().unwrap();
        let nodes: Vec<String> = self
            .graph
            .valves
            .iter()
            .map(|v| v.borrow().name.clone())
            .collect();
        let mut visited = VisitedMap::new_dense(nodes.as_slice(), self.max_turns);

        self.solve_dijkstra_from_node(&mut visited, st);
        visited
            .0
            .values()
            .flat_map(|v| v.values())
            .max()
            .cloned()
            .unwrap_or_default()
    }

    // returns maximum flow found
    pub fn solve_dijkstra_from_node(
        &mut self,
        visited: &mut VisitedMap,
        valve: Weak<RefCell<Valve>>,
    ) {
        let valve = valve.upgrade().unwrap();
        let valve = valve.borrow();
        // tick: we moved
        if self.tick() {
            return;
        }
        let vamount = visited.get_or_upsert_if_better(&valve.name, &self.turn, &self.cum_flow);
        // println!(
        // "first check: {}\t\t{} > {:?}",
        // valve.name, self.cum_flow, vamount
        // );
        if vamount.is_none() {
            return;
        }

        for wv in &valve.neighbors {
            let rcv: Rc<RefCell<Valve>> = wv.upgrade().unwrap();
            let v: Ref<Valve> = rcv.borrow();

            let vamount = visited.get_or_upsert_if_better(&v.name, &self.turn, &self.cum_flow);
            // println!(
            // "second check: {}\t{} > {:?}",
            // v.name, self.cum_flow, vamount,
            // );

            if vamount.is_some() {
                let mut c = self.clone();
                c.solve_dijkstra_from_node(visited, wv.clone());
            }

            if v.rate != 0 && !self.open_valves_lookup.contains(&v.name) {
                // will turn it on, eval, now that we tried not turning it on ^
                let mut c = self.clone();
                let done = c.tick();

                let vamount = visited.get_or_upsert_if_better(&v.name, &self.turn, &self.cum_flow);
                // println!("third check: {}\t{} > {:?}", v.name, self.cum_flow, vamount);
                if !done && vamount.is_some() {
                    c.cum_rate += v.rate;
                    c.open_valves.push(v.name.clone());
                    c.open_valves_lookup.insert(v.name.clone());
                    c.solve_dijkstra_from_node(visited, wv.clone());
                }
            }
        }
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
