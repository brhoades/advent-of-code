mod parse;

use crate::prelude::*;
#[cfg(not(test))]
use indicatif::{ProgressBar, ProgressStyle};
use parse::Almanac;
use std::ops::Range;
#[cfg(not(test))]
use std::sync::Arc;
use tokio::{runtime::Runtime, task::JoinSet};

pub fn run(input: String) -> Result<()> {
    let alm: Almanac = input.parse()?;

    println!(
        "minimum location for starter seeds: {}",
        alm.lowest_location()
    );

    let sums = alm.total_seed_range_width();
    println!("total range width: {sums}");

    println!(
        "minimum seed using started seed ranges: {}",
        alm.lowest_location_seed_range()
    );

    Ok(())
}

impl Almanac {
    // repeatedly maps to retrieve the header value for a seed
    fn header_mapping_for_seed(&self, target_header: &str, mut seed: u64) -> u64 {
        let mut header = "seed";
        while header != target_header {
            let target = self.headers.get(header).expect("unknown header");
            let mapping_key = (header.to_string(), target.to_string());
            let ranges = self.lookup.get(&mapping_key).expect("unknown mapping key");

            for (src, offset) in ranges {
                if src.contains(&seed) {
                    let old_seed = seed;
                    seed = (seed as i64 + offset) as u64;
                    if seed == 0 {
                        println!("{old_seed} became zero when mapping over src range {src:?} {header}-to-{target} w/ offset {offset}");
                    }
                    break;
                }
            }

            header = target;
        }

        seed
    }

    fn lowest_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|s| self.header_mapping_for_seed("location", *s))
            .min()
            .unwrap()
    }

    // Lowest location for seed range... evolved.
    //
    // I originally had a bug where I calculated the seed ranges as start/end pairs instead of start/len
    // pairs. it doubled the input set, which took over an hour to calculate.
    //
    // I fixed that later but I added parallelization before that. I kept the parallelization,
    // which is great since it's still needed. It takes the runtime down from 30m to a few minutes.
    //
    // Divide and conquer: the seed ranges are split into ranges of size 250MM, then processed in parallel
    // to find a minimum.
    fn lowest_location_seed_range(&self) -> u64 {
        #[cfg(not(test))]
        let prog = Arc::new(
            ProgressBar::new(self.total_seed_range_width()).with_style(
                ProgressStyle::with_template(
                    "[{elapsed}] {wide_bar:00.cyan/blue} {human_pos:>7}/{human_len:7} [ETA {eta}]",
                )
                .unwrap(),
            ),
        );

        let rt = Runtime::new().unwrap();
        let mut joins = JoinSet::new();

        let ranges = self
            .seed_ranges()
            .flat_map(|rng| split_range_until_size(rng, 250_000_000))
            .collect::<Vec<Range<u64>>>();

        for rng in ranges {
            #[cfg(not(test))]
            let prog = prog.clone();
            let alm = self.clone();

            joins.spawn_blocking_on(
                move || {
                    let mut min = u64::MAX;
                    for seed in rng {
                        let loc = alm.header_mapping_for_seed("location", seed);
                        if min > loc {
                            min = loc;
                        }

                        #[cfg(not(test))]
                        prog.inc(1);
                    }
                    min
                },
                rt.handle(),
            );
        }

        let mut min = u64::MAX;
        rt.block_on(async {
            while let Some(Ok(n)) = joins.join_next().await {
                println!("task exited");
                if min > n {
                    min = n;
                }
            }
        });
        min
    }

    // Returns pt2 seed ranges for easy iteration.
    fn seed_ranges(&self) -> impl Iterator<Item = Range<u64>> {
        let mut ranges = vec![];
        for i in (0..self.seeds.len()).step_by(2) {
            let start = *self.seeds.get(i).unwrap();
            ranges.push(start..(start + *self.seeds.get(i + 1).unwrap()));
        }

        ranges.into_iter()
    }

    // For displaying the progress bar we calculate the total number of seeds
    // to evlauate.
    fn total_seed_range_width(&self) -> u64 {
        self.seed_ranges().map(|rng| rng.end - rng.start).sum()
    }
}

// Splits a range into subranges that are at most size. It does this by trimming
// off the end into pieces of size until the original rnage is at most size.
fn split_range_until_size(mut rng: Range<u64>, size: usize) -> Vec<Range<u64>> {
    if rng.end - rng.start < size as u64 {
        return vec![rng];
    }

    let mut pieces = Vec::with_capacity((rng.end - rng.start) as usize / size);

    while rng.end - rng.start > size as u64 {
        let newend = rng.end - size as u64;
        pieces.push(newend..rng.end);
        rng = rng.start..newend;
    }

    pieces.push(rng);

    pieces
}

#[allow(dead_code)]
const EXAMPLE_1: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_example_mapping() {
        let a: Almanac = EXAMPLE_1.parse().unwrap();

        assert_eq!(35, a.lowest_location());
    }

    #[test]
    fn test_example_range_seeds() {
        let a: Almanac = EXAMPLE_1.parse().unwrap();

        assert_eq!(46, a.lowest_location_seed_range());
    }

    #[test]
    fn test_split_range() {
        let testrange = 0..25;
        let tofind = testrange.clone().collect::<HashSet<_>>();

        for size in 1..25 {
            let mut found = tofind.clone();
            let ranges = split_range_until_size(testrange.clone(), size);
            for rng in ranges {
                for i in rng {
                    assert!(found.remove(&i));
                }
            }

            assert!(found.is_empty(), "with size={size}, found had: {:?}", found);
        }
    }
}
