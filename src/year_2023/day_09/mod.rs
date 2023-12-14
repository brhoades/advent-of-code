use advent_of_code::prelude::*;

pub fn run(input: String) -> Result<()> {
    let lines = parse(&input)?;
    let sum = extrapolate_all_sum(&lines);
    println!("total input extrapolated sum: {sum}");

    let sum = extrapolate_backwards_sum(&lines);
    println!("total input extrapolated backwards sum: {sum}");

    Ok(())
}

fn parse(input: &str) -> Result<Vec<Vec<i64>>> {
    input
        .lines()
        .map(|l| {
            Ok(l.trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()?)
        })
        .collect()
}

fn extrapolate_all_sum(lines: &Vec<Vec<i64>>) -> i64 {
    let mut sum: i64 = 0;

    for line in lines {
        let mut line = line.to_owned();
        loop {
            sum += line.last().unwrap();

            if line.is_empty() || !line.iter().any(|v| *v != 0) {
                break;
            }

            line = extrapolate_next(&line).collect();
        }
    }

    sum
}

// sums the first extrapolated number for each line
fn extrapolate_backwards_sum(lines: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;

    for line in lines {
        // first build the full pyramid
        let mut extrapolated_lines = Vec::with_capacity(line.len() - 1);
        extrapolated_lines.push(line.clone());

        loop {
            let line = extrapolated_lines.last().unwrap();
            if line.is_empty() || !line.iter().any(|v| *v != 0) {
                break;
            }

            extrapolated_lines.push(extrapolate_next(line).collect());
        }

        // now walk it backwards, summing and retaining the last value interpolated
        let mut rewrite_lines = vec![];
        let mut last_interpolated = 0; // we always interpolate a zero first
        for line in extrapolated_lines.iter().rev().skip(1) {
            last_interpolated = line.first().unwrap() - last_interpolated;
            rewrite_lines.push(format!(
                "{last_interpolated} {}",
                line.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        sum += last_interpolated;

        for (i, l) in rewrite_lines.into_iter().rev().enumerate() {
            println!("{}{l}", (0..=i).map(|_| " ").collect::<Vec<_>>().join(""));
        }
    }

    sum
}

fn extrapolate_next(nums: &[i64]) -> impl Iterator<Item = i64> + '_ {
    nums.iter()
        .zip(nums.iter().skip(1))
        .map(|(cur, next)| next - cur)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

    #[test]
    fn test_sample() {
        let lines = parse(SAMPLE_INPUT).unwrap();
        assert_eq!(114, extrapolate_all_sum(&lines));

        assert_eq!(2, extrapolate_backwards_sum(&lines));
    }
}
