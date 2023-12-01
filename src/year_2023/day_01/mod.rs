use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    println!("===== part 1 ======");
    println!("calibration value for provided input: {}", solve(&input)?);
    Ok(())
}

// testable entrypoint
fn solve(input: &str) -> Result<i64> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .fold(Ok(0), |acc, l| Ok(acc? + extract_calibration(l)?))
}

// walk the string backwards and forwards, returning the first digit
fn extract_calibration(line: &str) -> Result<i64> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_inputs() {}

    const EXAMPLE_INPUT: &str = r"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    #[test]
    fn test_example_input_sum() {
        assert_eq!(142, solve(EXAMPLE_INPUT).unwrap());
    }

    #[test]
    fn test_example_input_calibrations() {
        let expected = vec![12, 38, 15, 77];

        assert_eq!(
            expected,
            EXAMPLE_INPUT
                .lines()
                .map(extract_calibration)
                .collect::<Result<Vec<_>>>()
                .unwrap()
        );
    }
}
