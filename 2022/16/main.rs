mod graph;

use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use graph::{Graph, Valve};

use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    let g: Graph = input.parse()?;

    Ok(())
}

/// SimState captures the state of the simulation in the moment.
#[derive(Clone, Debug)]
pub struct Simulation<'a> {
    /// cumulative flow so far
    pub cum_flow: u32,
    pub cum_rate: u32,
    pub minutes_elapsed: u32,
    pub minutes_max: u32,
    /// valves which are open. not stored in Graph to reduce memory use, maybe probably
    pub open_valves: Vec<String>,
    pub open_valves_lookup: HashSet<String>,
    pub graph: &'a Graph,
}

/// dumb brute force solver with obvious enhancements
impl<'a> Simulation<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self {
            graph,
            open_valves: vec![],
            open_valves_lookup: Default::default(),
            cum_flow: 0,
            cum_rate: 0,
            minutes_elapsed: 0,
            minutes_max: 30,
        }
    }

    // returns maximum flow found
    pub fn solve_brute(&mut self, valve: Weak<RefCell<Valve>>) -> Self {
        let valve = valve.upgrade().unwrap();
        let valve = valve.borrow();
        // tick: we moved
        if self.tick() {
            return self.clone();
        }

        let mut best: Option<Self> = None;
        for wv in &valve.neighbors {
            let rcv: Rc<RefCell<Valve>> = wv.upgrade().unwrap();
            let v: Ref<Valve> = rcv.borrow();

            let mut c = self.clone();
            let new = c.solve_brute(wv.clone());

            if best
                .as_ref()
                .map(|b| b.cum_flow < new.cum_flow)
                .unwrap_or(true)
            {
                best = Some(new);
            }

            if v.rate != 0 && !c.open_valves_lookup.contains(&v.name) {
                // will turn it on, eval, now that we tried not turning it on ^
                let mut c = c.clone();
                let new = if c.tick() {
                    c
                } else {
                    c.cum_rate += v.rate;
                    c.open_valves.push(v.name.clone());
                    c.open_valves_lookup.insert(v.name.clone());
                    c.solve_brute(wv.clone())
                };

                if best
                    .as_ref()
                    .map(|b| b.cum_flow < new.cum_flow)
                    .unwrap_or(true)
                {
                    best = Some(new);
                }
            }
        }

        best.unwrap_or_else(|| self.clone())
    }

    // ticks once and returns true if done
    pub fn tick(&mut self) -> bool {
        self.minutes_elapsed += 1;
        self.cum_flow += self.cum_rate;

        self.minutes_elapsed >= self.minutes_max
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

        let graph: Graph = input.parse().unwrap(); //

        let mut solver = Simulation::new(&graph);
        let st = graph.start().unwrap();
        let sol = solver.solve_brute(st);
        println!("valves open: {:?}", sol.open_valves);

        assert_eq!(1651, sol.cum_rate);
    }
}
