use advent_of_code::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub fn run(input: String) -> Result<()> {
    println!("===== part 1 ======");
    println!(
        "calibration value for provided input: {:?}",
        solve_pt1(&input)
    );

    println!("===== part 2 ======");
    println!(
        "calibration value for provided input: {:?}",
        solve_pt2(&input)
    );
    Ok(())
}

// testable entrypoint
fn solve_pt1(input: &str) -> Result<i64> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .try_fold(0, |acc, l| Ok(acc + extract_calibration_part1(l)?))
}

// walk the string backwards and forwards, returning the first digit
fn extract_calibration_part1(line: &str) -> Result<i64> {
    let mut res = line
        .chars()
        .find(char::is_ascii_digit)
        .ok_or_else(|| anyhow!("failed to find digit in input: {line}"))?
        .to_string()
        .parse::<i64>()?;
    res *= 10; // always 2 digits

    res += line
        .chars()
        .rev()
        .find(char::is_ascii_digit)
        .unwrap()
        .to_string()
        .parse::<i64>()?;

    Ok(res)
}

//
// --------------- part 2 ---------------------
//

// testable entrypoint
fn solve_pt2(input: &str) -> Result<i64> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .try_fold(0, |acc, l| Ok(acc + extract_calibration_part_2(l)?))
}
// part 2: searches for number needles in the line, then does the same backwards.
// The result of both is combined and returned.
fn extract_calibration_part_2(line: &str) -> Result<i64> {
    let Some(res) = extract_needle(line, &FORWARD_NEEDLES, 5) else {
        return Ok(0);
    };

    let revline = line.chars().rev().collect::<String>();
    let Some(res_rev) = extract_needle(&revline, &BACKWARD_NEEDLES, 5) else {
        unreachable!("should be at least one number at this point");
    };

    Ok(NEEDLE_VALUE_LOOKUP.get(res).unwrap() * 10 + NEEDLE_VALUE_LOOKUP.get(res_rev).unwrap())
}

// part 2: walks &str with a sliding window of the provided size. Returns
// the first needle found.
//
// If window size is larger than the haystack, the haystack size is used instead.
// Needles which are too large are discarded in this case.
fn extract_needle<'a>(
    haystack: &str,
    needles: &[&'a str],
    mut window_size: usize,
) -> Option<&'a str> {
    if haystack.is_empty() {
        return None;
    }
    let mut needles = needles.to_vec();
    if window_size > haystack.len() {
        window_size = haystack.len();
        needles.retain(|n| n.len() <= window_size);
    }

    // Scanning the beginning of the string for smaller slices removes
    // ordering concerns with our needle search.
    for i in 1..window_size {
        for needle in &needles {
            if haystack[..i].contains(needle) {
                return Some(needle);
            }
        }
    }

    let haystack = haystack.split("").collect::<Vec<_>>();
    let haystack = haystack.as_slice();
    for window in haystack.windows(window_size).map(|w| w.join("")) {
        for needle in &needles {
            // scan the last (newest) window segment for the needle only
            let start = window_size - needle.len();
            if window[start..] == **needle {
                return Some(needle);
            }
        }
    }

    None
}

// index corresponds with the number/digit's value
const STR_DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const STR_NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const STR_NUMBERS_REV: [&str; 10] = [
    "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

static FORWARD_NEEDLES: Lazy<Vec<&str>> = Lazy::new(|| {
    STR_DIGITS
        .iter()
        .cloned()
        .chain(STR_NUMBERS.iter().cloned())
        .collect::<Vec<_>>()
});
static BACKWARD_NEEDLES: Lazy<Vec<&str>> = Lazy::new(|| {
    STR_DIGITS
        .iter()
        .cloned()
        .chain(STR_NUMBERS_REV.iter().cloned())
        .collect::<Vec<_>>()
});

static NEEDLE_VALUE_LOOKUP: Lazy<HashMap<&str, i64>> = Lazy::new(|| {
    // take the enumerations thenf lip the tuples. The three arrays are lined up so
    // the index is the value
    STR_DIGITS
        .iter()
        .enumerate()
        .chain(STR_NUMBERS.iter().enumerate())
        .chain(STR_NUMBERS_REV.iter().enumerate())
        .map(|(i, v)| (*v, i as i64))
        .collect()
});

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_PART_1: &str = r"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    const EXAMPLE_INPUT_PART_2: &str = r"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

    #[test]
    fn test_example_input_sum_part_1() {
        assert_eq!(142, solve_pt1(EXAMPLE_INPUT_PART_1).unwrap());
    }

    #[test]
    fn test_example_input_calibrations_part_1() {
        let expected = vec![12, 38, 15, 77];

        assert_eq!(
            expected,
            EXAMPLE_INPUT_PART_1
                .lines()
                .map(extract_calibration_part1)
                .collect::<Result<Vec<_>>>()
                .unwrap()
        );
    }

    #[test]
    fn test_extract_needle_part_2() {
        let cases = [
            ("onetwo", Some("one")),
            ("oetwo", Some("two")),
            ("two", Some("two")),
            ("two3456", Some("two")),
            ("twtwtw", None),
            ("amsodoawie12w", Some("1")),
            ("", None),
        ];

        for (i, (input, expected)) in cases.into_iter().enumerate() {
            assert_eq!(
                expected,
                extract_needle(input, &FORWARD_NEEDLES, 5),
                "in case {i}: extract_needle({input}, needles, 5) != {expected:?}"
            );
        }
    }

    #[test]
    fn test_extract_needle_part_2_example() {
        let expected_fwd: Vec<_> = EXAMPLE_INPUT_PART_2
            .lines()
            .map(|l| {
                extract_needle(l, &FORWARD_NEEDLES, 5)
                    .unwrap_or_else(|| panic!("expected to find a needle in {l}"))
            })
            .map(|d| NEEDLE_VALUE_LOOKUP.get(d).unwrap())
            .cloned()
            .collect();
        let expected_rev: Vec<_> = EXAMPLE_INPUT_PART_2
            .lines()
            .map(|l| l.chars().rev().collect::<String>())
            .map(|l: String| {
                extract_needle(&l, &BACKWARD_NEEDLES, 5)
                    .unwrap_or_else(|| panic!("expected to find a needle in {l}"))
            })
            .map(|d| NEEDLE_VALUE_LOOKUP.get(d).unwrap())
            .cloned()
            .collect();

        assert_eq!(vec![2, 8, 1, 2, 4, 1, 7], expected_fwd);
        assert_eq!(vec![9, 3, 3, 4, 2, 4, 6], expected_rev);
    }

    #[test]
    fn test_extract_needle_part_2_edges() {
        assert_eq!(33, solve_pt2("nt3").unwrap());
        assert_eq!(0, solve_pt2("nt").unwrap());
        assert_eq!(0, solve_pt2("zero").unwrap());
        assert_eq!(0, solve_pt2("zerozero").unwrap());
        assert_eq!(12, solve_pt2("12").unwrap());
        assert_eq!(11, solve_pt2("1").unwrap());
        assert_eq!(99, solve_pt2("fffffninefffff").unwrap());
        assert_eq!(0, solve_pt2("").unwrap());
    }
}
