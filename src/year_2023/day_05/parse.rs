use std::{collections::HashMap, ops::Range, str::FromStr};

use advent_of_code::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Almanac {
    // I'm not sure why this Almanac has seeds, but it's convenient
    pub seeds: Vec<u64>,

    // (seed, soil) -> (1..3, 500..503), etc
    pub lookup: HashMap<MapHeader, Vec<Mapping>>,
    // seed -> soil, soil -> light, etc
    pub headers: HashMap<String, String>,
}

// parser state
enum State {
    Seeds,
    Header,
    Mappings,
}

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut alm = Almanac::default();
        let mut state = State::Seeds;

        let mut last_header = None;

        for line in s.lines() {
            match state {
                State::Seeds => {
                    alm.seeds = line[7..]
                        .split(' ')
                        .map(FromStr::from_str)
                        .collect::<Result<_, _>>()
                        .with_context(|| {
                            format!("failed to parse seeds in '{}'", &line[7..])
                        })?;
                    state = State::Header;
                }
                State::Header if line.is_empty() => (),
                State::Header => {
                    let pieces = line[..(line.len() - 5)].splitn(3, '-').collect::<Vec<_>>();
                    match pieces.as_slice() {
                        &[lhs, _, rhs] => {
                            last_header = Some((lhs.to_string(), rhs.to_string()));
                            alm.headers.insert(lhs.to_string(), rhs.to_string());
                            alm.lookup
                                .insert((lhs.to_string(), rhs.to_string()), vec![]);

                            state = State::Mappings;
                        }
                        _ => bail!("unexpected number of pieces in {line}"),
                    }
                }
                State::Mappings if line.is_empty() => state = State::Header,
                State::Mappings => {
                    let mapping = mapping_line(line)
                        .with_context(|| format!("failed to parse mapping: {line}"))?;
                    alm.lookup
                        .get_mut(last_header.as_ref().unwrap())
                        .expect("header for mapping should have vec prepopulated")
                        .push(mapping);
                }
            }
        }

        Ok(alm)
    }
}

// from a source range to a offset to map to the destination range
pub type Mapping = (Range<u64>, i64);
pub type MapHeader = (String, String);

fn mapping_line(l: &str) -> Result<Mapping> {
    let pieces: Vec<u64> = l
        .splitn(3, ' ')
        .map(|n| n.parse().with_context(|| format!("failed to parse '{n}'")))
        .collect::<Result<_, _>>()?;

    let (dest_start, src_start, width) = match pieces.as_slice() {
        &[d, s, w] => (d, s, w),
        _ => bail!("unknown mapping line arrangement: {:?}", pieces),
    };

    Ok((
        src_start..(src_start + width),
        dest_start as i64 - src_start as i64,
    ))
}

#[cfg(test)]
mod test {
    use super::{super::EXAMPLE_1, *};

    #[test]
    fn test_parse() {
        let a: Almanac = EXAMPLE_1.parse().unwrap();
        assert_eq!(vec![79, 14, 55, 13], a.seeds);
        for (k, v) in vec![
            ("seed", "soil"),
            ("soil", "fertilizer"),
            ("fertilizer", "water"),
            ("water", "light"),
            ("light", "temperature"),
            ("temperature", "humidity"),
            ("humidity", "location"),
        ] {
            assert_eq!(
                v,
                a.headers
                    .get(k)
                    .expect(&format!("failed to get {k} in headers"))
            );
        }
    }
}
