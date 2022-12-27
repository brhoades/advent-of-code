mod equality;
mod parse;
mod value;

use anyhow::{anyhow, Result};
use std::fmt;

use value::{Value, Value::*};

pub fn run(input: String) -> Result<()> {
    println!("sum of indices: {}", indices_sum(&input)?);

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
                &parse::line(&l).expect(&format!("failed to parse left case: {}", l)),
                &parse::line(&r).expect(&format!("failed to parse left case: {}", l))
            ),
            "case: '{}' == '{}'",
            l,
            r,
        );
    }
}

fn indices_sum(input: &str) -> Result<usize> {
    let mut sum = 0;
    for (i, pair) in input.split("\n\n").filter(|l| *l != "").enumerate() {
        let mut pair = pair.split("\n").map(parse::line);
        let l = &pair
            .next()
            .ok_or_else(|| anyhow!("first line of input missing @ {}", i))??;
        let r = &pair
            .next()
            .ok_or_else(|| anyhow!("second line of input missing @ {}", i))??;

        // println!("i: {}\t{}\t{:?} == {:?}", i, list_order_correct(l, r), l, r);
        println!("Problem {}: {}", i + 1, equality::list_order_correct(l, r),);
        if equality::list_order_correct(l, r) {
            sum += i + 1;
        }
    }
    Ok(sum)
}

#[test]
fn test_example_1() {
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

    for (i, pair) in input.split("\n\n").filter(|l| *l != "").enumerate() {
        let mut pair = pair.split("\n").map(parse::line);
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

    assert_eq!(13, indices_sum(input).unwrap());
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
