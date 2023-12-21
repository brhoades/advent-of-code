mod equality;
mod parse;
mod value;

use anyhow::Result;
use std::fmt;

use value::{Value, Value::*};

pub fn run(input: String) -> Result<()> {
    let mut lines = parse_input(&input)?;
    println!("sum of sorted indices: {}", indices_sum(&lines)?);

    println!("\n======= part 2 =========\n");

    let two = parse::line("[[2]]")?;
    let six = parse::line("[[6]]")?;
    lines.push(two.clone());
    lines.push(six.clone());
    lines.sort();

    println!(
        "decoder key: {}",
        lines
            .iter()
            .enumerate()
            .filter(|(_, p)| **p == two || **p == six)
            .fold(1, |acc, (i, _)| { acc * i })
    );

    Ok(())
}

#[test]
fn test_equality_simple_cases() {
    let cases = vec![
        ("[]", "[1]", true),
        ("[1]", "[]", false),
        ("[1]", "[2]", true),
        ("[2]", "[1]", false),
        ("[[]]", "[]", false),
        ("[]", "[[]]", true),
        ("[[[[1,0]]]]", "[[[]],1]", false),
        ("[1]", "[1,2]", true),
        ("[2,1]", "[2]", false),
        ("[2]", "[2,2,2,2,1]", true),
        ("[2,2,2,2,[0]]", "[2,2,2,2,1]", true),
        ("[2,5]", "[2,1]", false),
        ("[10]", "[1]", false),
        ("[1]", "[10]", true),
        ("[[5],2]", "[[4],3]", false),
        ("[[1],[2],[3]]", "[2,3,4]", true),
        ("[1,2,3]", "[[2],[3],[4]]", true),
        ("[[1],[2,3,4]]", "[[1],4]", true),
        ("[[1],[[2],[[3]],[[[4]]]]]", "[[1],4]", true),
        ("[[1],[[2],[[3]],[[[4]]]]]", "[[1],[[[[[2,[3]]]]]],5]", true),
        (
            "[[1],[[2],[[3]],[[[4]]]]]",
            "[[1],[[[[]]]],[[[[[2,[3]]]]]],5]",
            false,
        ),
        ("[[1],[2,3,4]]", "[[1],4]", true),
        ("[1,1,1]", "[1,1]", false),
        ("[[1],1]", "[1,1,1]", true),
        ("[1,1]", "[1,1,1]", true),
        ("[9]", "[8,7,6]", false),
        ("[]", "[[]]", true),
        ("[[[]]]", "[[]]", false),
        ("[3]", "[[]]", false),
        ("[[0,[4]]", "[[0,[1]]]", false),
    ]
    .into_iter()
    .collect::<Vec<_>>();

    for (l, r, expected) in cases {
        assert_eq!(
            expected,
            equality::list_order_correct(
                &parse::line(l).unwrap_or_else(|_| panic!("failed to parse left case: {}", l)),
                &parse::line(r).unwrap_or_else(|_| panic!("failed to parse left case: {}", l))
            ),
            "case: '{}' == '{}'",
            l,
            r,
        );
    }
}

fn parse_input(input: &str) -> Result<Vec<Value>> {
    input
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.split('\n'))
        .map(parse::line)
        .collect::<Result<Vec<_>>>()
}

fn indices_sum(lines: &Vec<Value>) -> Result<usize> {
    let mut sum = 0;
    for i in (0..(lines.len() - 1)).step_by(2) {
        let l = &lines[i];
        let r = &lines[i + 1];

        if l < r {
            sum += i / 2 + 1;
        }
    }
    Ok(sum)
}

#[test]
fn test_example_1() {
    use anyhow::anyhow;

    let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    for (i, pair) in input.split("\n\n").filter(|l| !l.is_empty()).enumerate() {
        let mut pair = pair.split('\n').map(parse::line);
        let l = &pair
            .next()
            .ok_or_else(|| anyhow!("first line of input missing @ {}", i))
            .unwrap()
            .unwrap();
        let r = &pair
            .next()
            .ok_or_else(|| anyhow!("second line of input missing @ {}", i))
            .unwrap()
            .unwrap();

        print!("Entry {}: ", i + 1);
        if equality::list_order_correct(l, r) {
            println!("correct");
        } else {
            println!("incorrect");
        }
    }

    let lines = parse_input(input).unwrap();
    assert_eq!(13, indices_sum(&lines).unwrap());
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    write!(f, "{}{}", v, if i == l.len() - 1 { "" } else { "," })?;
                }
                write!(f, "]")?;
            }
            Num(n) => write!(f, "{}", n)?,
        }

        Ok(())
    }
}
