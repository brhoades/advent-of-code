use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use advent_of_code::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Graph {
    pub valves: HashMap<String, Rc<RefCell<Valve>>>,
}

#[derive(Clone, Debug, Default)]
pub struct Valve {
    pub name: String,
    pub neighbors: Vec<Weak<RefCell<Valve>>>,
    pub rate: u32,
}

impl Valve {
    pub fn neighbors_ref(&self) -> Vec<Rc<RefCell<Valve>>> {
        self.neighbors
            .iter()
            .map(|n| n.upgrade().unwrap())
            .collect()
    }
}

impl Graph {
    #[allow(dead_code)]
    pub fn start(&self) -> Option<&Rc<RefCell<Valve>>> {
        self.valves.get("AA")
    }
}

impl FromStr for Graph {
    type Err = Error;

    /// takes line-by-line representation of a cyclic undirected graph network of valves
    /// and returns a Graph incorporating them.
    fn from_str(s: &str) -> Result<Self> {
        let mut valves: HashMap<String, Rc<RefCell<Valve>>> = Default::default();
        let mut neighbors: HashMap<&str, Vec<&str>> = Default::default();

        for line in s.lines() {
            let parts = line
                .split(" ")
                .filter(|l| l.trim() != "")
                .collect::<Vec<_>>();

            let (name, rate, neighbor_valves): (&str, u32, Vec<&str>) =
                if let [_valve, name, _, _, rate] = parts[..5] {
                    (
                        name,
                        rate.strip_prefix("rate=")
                            .and_then(|r| r.strip_suffix(";"))
                            .ok_or_else(|| anyhow!("failed to parse rate: {}", rate))
                            .and_then(|r| r.parse().map_err(|e| anyhow!("{}", e)))?,
                        parts[9..]
                            .into_iter()
                            .map(|s| s.trim_end_matches(","))
                            .collect(),
                    )
                } else {
                    bail!("unknown format for line: {}", s)
                };

            valves.insert(
                name.to_string(),
                Rc::new(RefCell::new(Valve {
                    name: name.to_string(),
                    neighbors: vec![],
                    rate,
                })),
            );
            neighbors.insert(name, neighbor_valves);
        }

        // now we've got a full valves map, plug in edges
        let mut neighbors: HashMap<&str, Vec<_>> = neighbors
            .into_iter()
            .map(|(name, ns)| {
                Ok((
                    name,
                    ns.into_iter()
                        .map(|n| {
                            Ok(Rc::downgrade(valves.get(n).ok_or_else(|| {
                                anyhow!("dangling valve neighbor edge: {}", n)
                            })?))
                        })
                        .collect::<Result<_>>()?,
                ))
            })
            .collect::<Result<_>>()?;
        for (name, v) in &valves {
            v.borrow_mut().neighbors = neighbors.remove(name.as_str()).unwrap();
        }

        Ok(Graph { valves })
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Graph with {} valves:\n", self.valves.len())?;
        let mut m: Vec<(String, String)> = self
            .valves
            .values()
            .map(|v| {
                let v = v.borrow();
                (
                    format!("{} (r={})", v.name, v.rate),
                    v.neighbors
                        .iter()
                        .map(|n| n.upgrade().unwrap())
                        .map(|n| n.borrow().name.clone())
                        .collect::<Vec<_>>()
                        .as_slice()
                        .join(", "),
                )
            })
            .collect();
        m.sort_by_key(|(v, _)| v.clone());

        for (v, neighbors) in m {
            write!(f, "  {} => {}\n", v, neighbors)?;
        }

        Result::Ok(())
    }
}

impl Graph {
    pub fn get(&self, name: &str) -> Option<Rc<RefCell<Valve>>> {
        self.valves.get(name).cloned()
    }

    pub fn get_neighbors_ref(&self, name: &str) -> Vec<Rc<RefCell<Valve>>> {
        self.valves
            .get(name)
            .map(|n| n.borrow().neighbors.clone())
            .unwrap_or_else(|| vec![])
            .iter()
            .map(|v| v.upgrade().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_pt1_ex() {
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
    }
}
