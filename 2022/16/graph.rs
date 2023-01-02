use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use advent_of_code::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Graph {
    pub valves: Vec<Rc<RefCell<Valve>>>,
}

#[derive(Clone, Debug, Default)]
pub struct Valve {
    pub name: String,
    pub neighbors: Vec<Weak<RefCell<Valve>>>,
    pub rate: u32,
}

impl Graph {
    pub fn start(&self) -> Option<Weak<RefCell<Valve>>> {
        self.valves
            .iter()
            .find(|v| v.borrow().name == "AA")
            .map(|v| Rc::downgrade(v))
    }
}

impl FromStr for Graph {
    type Err = Error;

    /// takes line-by-line representation of a cyclic undirected graph network of valves
    /// and returns a Graph incorporating them.
    fn from_str(s: &str) -> Result<Self> {
        let mut valves: HashMap<&str, Rc<RefCell<Valve>>> = Default::default();
        let mut neighbors: HashMap<&str, Vec<&str>> = Default::default();

        for line in s.lines() {
            let parts = line.split(" ").filter(|l| *l != "").collect::<Vec<_>>();

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
                name,
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
            v.borrow_mut().neighbors = neighbors.remove(name).unwrap();
        }

        Ok(Graph {
            valves: valves.into_values().collect(),
        })
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
