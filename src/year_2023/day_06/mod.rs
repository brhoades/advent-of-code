use std::collections::HashMap;
use std::str::FromStr;

use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    let r: Records = input.parse()?;

    let product_winning_buttons: u64 = r
        .iter()
        .map(|(record_dur, record_dist)| {
            simulation(*record_dur)
                .filter(|(_, dist)| dist > &&record_dist)
                .count() as u64
        })
        .product();

    println!("product of winning buttons: {product_winning_buttons}");

    let mega: MegaRecord = input.parse()?;
    let product_winning_buttons: u64 = simulation(mega.0)
        .filter(|(_, dist)| dist > &mega.1)
        .count() as u64;

    println!("product of megarecord: {product_winning_buttons}");

    Ok(())
}

type Duration = u64;
type Distance = u64;

#[derive(Debug, Clone)]
struct Records(HashMap<Duration, Distance>);

impl FromStr for Records {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines().filter(|s| !s.trim().is_empty()).map(|l| {
            l.split(' ')
                .filter(|s| !s.is_empty())
                .skip(1)
                .map(FromStr::from_str)
        });

        lines
            .next()
            .unwrap()
            .zip(lines.next().unwrap())
            .map(|(l, r)| Ok((l?, r?)))
            .collect::<Result<HashMap<_, _>>>()
            .map(Records)
    }
}

#[derive(Debug)]
struct MegaRecord(Duration, Distance);

impl FromStr for MegaRecord {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines().filter(|s| !s.trim().is_empty()).map(|l| {
            l.split(' ')
                .filter(|s| !s.is_empty())
                .skip(1)
                .collect::<String>()
                .chars()
                .filter(|c| !c.is_ascii_whitespace())
                .collect::<String>()
                .parse()
        });

        Ok(Self(lines.next().unwrap()?, lines.next().unwrap()?))
    }
}

impl Records {
    fn iter(&self) -> impl Iterator<Item = (&u64, &u64)> {
        self.0.iter()
    }
}

// returns iterator over simulation results by duration the button held
// and total distance it ran
fn simulation(total_duration: Duration) -> impl Iterator<Item = (Duration, Distance)> {
    (0..=total_duration).map(move |button_held_dur| {
        (
            button_held_dur,
            simulate_run(button_held_dur, total_duration),
        )
    })
}

// returns the distance covered during a race of duration if the button is held
fn simulate_run(button_held_duration: Duration, race_duration: Duration) -> Distance {
    let remaining_time = race_duration - button_held_duration;
    remaining_time * button_held_duration
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
    Time:      7  15   30
    Distance:  9  40  200";

    #[test]
    fn test_parse() {
        let records: Records = EXAMPLE_INPUT.parse().unwrap();

        assert_eq!(
            [(7, 9), (15, 40), (30, 200)]
                .into_iter()
                .collect::<HashMap<_, _>>(),
            records.0
        );
    }

    #[test]
    fn test_iter() {
        let records: Records = EXAMPLE_INPUT.parse().unwrap();
        let mut values = records.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>();
        values.sort_by_key(|(k, _)| *k);

        assert_eq!(vec![(7, 9), (15, 40), (30, 200)], values);
    }

    #[test]
    fn test_simulation_results() {
        let records: Records = EXAMPLE_INPUT.parse().unwrap();

        assert_eq!(
            vec![
                (0, 0),
                (1, 6),
                (2, 10),
                (3, 12),
                (4, 12),
                (5, 10),
                (6, 6),
                (7, 0)
            ],
            simulation(7).collect::<Vec<_>>()
        );
    }
}
