use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Graph {
    pub valves: HashMap<String, Valve>,
}

#[derive(Clone, Debug, Default)]
pub struct Valve {
    pub name: String,
    neighbors: Vec<Valve>,
    pub rate: u32,
}

impl Valve {
    pub fn neighbors(&self) -> &Vec<Valve> {
        &self.neighbors
    }
}

impl Graph {
    #[allow(dead_code)]
    pub fn start(&self) -> Option<&Valve> {
        self.valves.get("AA")
    }
}

impl FromStr for Graph {
    type Err = Error;

    /// takes line-by-line representation of a cyclic undirected graph network of valves
    /// and returns a Graph incorporating them.
    fn from_str(s: &str) -> Result<Self> {
        let mut valves: HashMap<String, Valve> = Default::default();
        let mut neighbors: HashMap<&str, Vec<&str>> = Default::default();

        for line in s.lines() {
            let parts = line
                .split(' ')
                .filter(|l| l.trim() != "")
                .collect::<Vec<_>>();

            let (name, rate, neighbor_valves): (&str, u32, Vec<&str>) =
                if let [_valve, name, _, _, rate] = parts[..5] {
                    (
                        name,
                        rate.strip_prefix("rate=")
                            .and_then(|r| r.strip_suffix(';'))
                            .ok_or_else(|| anyhow!("failed to parse rate: {}", rate))
                            .and_then(|r| r.parse().map_err(|e| anyhow!("{}", e)))?,
                        parts[9..].iter().map(|s| s.trim_end_matches(',')).collect(),
                    )
                } else {
                    bail!("unknown format for line: {}", s)
                };

            valves.insert(
                name.to_string(),
                Valve {
                    name: name.to_string(),
                    neighbors: vec![],
                    rate,
                },
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
                            Ok(valves
                                .get(n)
                                .ok_or_else(|| anyhow!("dangling valve neighbor edge: {}", n))?
                                .clone())
                        })
                        .collect::<Result<_>>()?,
                ))
            })
            .collect::<Result<_>>()?;
        for (name, v) in &mut valves {
            v.neighbors = neighbors.remove(name.as_str()).unwrap();
        }

        Ok(Graph { valves })
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Graph with {} valves:", self.valves.len())?;
        let mut m: Vec<String> = self
            .valves
            .values()
            .map(|v| {
                let n = v
                    .neighbors
                    .iter()
                    .map(|v| v.name.as_str())
                    .collect::<Vec<_>>();
                format!("{} (r={}) => {}", v.name, v.rate, n.as_slice().join(", "))
            })
            .collect();
        m.sort();

        for line in m {
            writeln!(f, "  {}", line)?;
        }

        Result::Ok(())
    }
}

impl Graph {
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&Valve> {
        self.valves.get(name)
    }

    #[allow(dead_code)]
    pub fn neighbors(&self, name: &str) -> Option<&Vec<Valve>> {
        self.valves.get(name).map(|v| &v.neighbors)
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

        let _graph: Graph = input.parse().unwrap();
    }
}
