use anyhow::{anyhow, Result};

pub fn run(input: String) -> Result<()> {
    println!("sum of indices: {}", indices_sum(&input)?);

    Ok(())
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Value {
    List(Vec<Value>),
    Num(u8),
}

use Value::*;

fn parse_line(input: &str) -> Result<Value> {
    parse_value(&input.split("").filter(|c| *c != "").collect::<Vec<_>>()).map(|(_, v)| v)
}

// str as a value, consuming input until a ] is
// received. Returns the position of the last ]
// along with data actually parsed so far.
fn parse_value(input: &[&str]) -> Result<(usize, Value)> {
    let mut i = 0;
    let mut thisvec = vec![];

    while i < input.len() {
        match input[i] {
            "[" if i == 0 => (),
            "[" => {
                let (ni, next) = parse_value(&input[i..]).map_err(|e| anyhow!("at {} in {:?}: {}", i, input, e))?;
                thisvec.push(next);
                i += ni;
            }
            "]" => break,
            "," => (),
            c => {
                let next = c.parse().map_err(|e| anyhow!("'{}' is not a number, [ nor ]. invalid input: {}",  c, e))?;
                thisvec.push(Num(next));
            },
        }

        i += 1;
    }

    Ok((i, List(thisvec)))
}

#[test]
fn test_parse_simple_cases() {
    use std::collections::HashMap;

    let cases = vec![
        ("[]", List(vec![])),
        ("[1]", List(vec![1].into_iter().map(Num).collect())),
        ("[1,2]", List(vec![1,2].into_iter().map(Num).collect())), //
        ("[[]]", List(vec![List(vec![])])),
        ("[1,[]]", List(vec![Num(1),List(vec![])])),
        ("[1,[],2,3]", List(vec![Num(1),List(vec![]),Num(2),Num(3)])),
        ("[1,[],[1,2],3]", List(vec![Num(1),List(vec![]),List(vec![Num(1),Num(2)]),Num(3)])),
    ].into_iter().collect::<HashMap<_, _>>();

    for (case, expected) in cases {
        assert_eq!(expected, parse_line(&case).expect(&format!("failed to parse: {}", case)));
    }

}

// recursively calls itself to compare two values, returning true if
// they meet the criteria outlined in the problem.
fn list_order_correct(left: &Value, right: &Value) -> bool {
    let (l, r) = match (left, right) {
        (List(l), List(r)) => (l, r),
        /*
        makes the example fail, but still getting the original problem wrong.
        (Num(l), List(r)) => {
            extra.push(Num(*l));
            (&extra, r)
        },
        (List(l), Num(r)) => {
            extra.push(Num(*r));
            (l, &extra)
        },
        */
        (Num(l), List(r)) => {
            if let Some(Num(r)) = r.first() {
                return l <= r;
            }
            return false;
        },
        (List(l), Num(r)) => {
            if let Some(Num(l)) = l.first() {
                return l <= r;
            }
            return true;
        },
        (Num(l), Num(r)) => return l <= r,
    };

    if l.len() > r.len() {
        return false;
    }
    if l.len() == 0 {
        return true;
    }

    l.iter().zip(r.iter()).map(|(l, r)| {
        let res = list_order_correct(l, r);
        println!("{}\t{:?} =?= {:?}", res, l, r);
        res
    }).all(|v| v)
}

#[test]
fn test_equality_simple_cases() {
    let cases = vec![
        ("[]", "[]", true),
        ("[1]", "[1]", true),
        ("[1,2]", "[1,2]", true),
        ("[1,2]", "[1,2]", true),
        ("[[]]", "[[]]", true),
        ("[1,[]]", "[1,[]]", true),
        ("[1,[],[1,2],3]", "[1,[],[1,2],3]", true),
        ("[]", "[1]", true),
        ("[1]", "[]", false),
        ("[1]", "[1,2]", true),
        ("[2,1]", "[2]", false),
        ("[2,5]", "[2,1]", false),
        ("[[1],[2,3,4]]", "[[1],4]", true),
    ].into_iter().collect::<Vec<_>>();

    for (l, r, expected) in cases {
        assert_eq!(
            expected,
            list_order_correct(&parse_line(&l).expect(&format!("failed to parse left case: {}", l)),
            &parse_line(&r).expect(&format!("failed to parse left case: {}", l))),
            "case: '{}' == '{}'",
            l, r,
        );
    }

}

fn indices_sum(input: &str) -> Result<usize> {
    let mut sum = 0;
    for (i, pair) in input.split("\n\n").filter(|l| *l != "").enumerate() {
        let mut pair = pair.split("\n").map(parse_line);
        let l = &pair.next().ok_or_else(|| anyhow!("first line of input missing @ {}", i))??;
        let r = &pair.next().ok_or_else(|| anyhow!("second line of input missing @ {}", i))??;

        println!("i: {}\t{}\t{:?} == {:?}", i, list_order_correct(l, r), l, r);
        if list_order_correct(l, r) {
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

    assert_eq!(13, indices_sum(input).unwrap());

}
