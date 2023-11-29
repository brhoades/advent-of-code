use anyhow::{anyhow, bail, Result};

use super::{Value, Value::*};

pub fn line(input: &str) -> Result<Value> {
    value(&input.split("").filter(|c| !c.is_empty()).collect::<Vec<_>>()).map(|(_, v)| v)
}

// str as a value, consuming input until a ] is
// received. Returns the position of the last ]
// along with data actually parsed so far.
pub fn value(input: &[&str]) -> Result<(usize, Value)> {
    let mut i = 0;
    let mut thisvec = vec![];
    let mut acc = String::new();

    while i < input.len() {
        let c = input[i].chars().next().unwrap();

        if !acc.is_empty() && !c.is_ascii_digit() {
            let next = acc
                .parse()
                .map_err(|e| anyhow!("'{}' is not a number, [ nor ]. invalid input: {}", c, e))?;
            thisvec.push(Num(next));
            acc = String::new();
        }
        // running number
        match c {
            '0'..='9' => acc.push(c),
            '[' if i == 0 => (),
            '[' => {
                let (ni, next) =
                    value(&input[i..]).map_err(|e| anyhow!("at {} in {:?}: {}", i, input, e))?;
                thisvec.push(next);
                i += ni;
            }
            ']' => break,
            ',' => (),
            c => bail!("'{}' is not a number, [ nor ]", c),
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
        ("[1,2]", List(vec![1, 2].into_iter().map(Num).collect())), //
        ("[1,2]", List(vec![1, 2].into_iter().map(Num).collect())), //
        ("[[]]", List(vec![List(vec![])])),
        ("[1,[]]", List(vec![Num(1), List(vec![])])),
        (
            "[1,[],2,3]",
            List(vec![Num(1), List(vec![]), Num(2), Num(3)]),
        ),
        (
            "[1,[],[1,2],3]",
            List(vec![
                Num(1),
                List(vec![]),
                List(vec![Num(1), Num(2)]),
                Num(3),
            ]),
        ),
        ("[10,[]]", List(vec![Num(10), List(vec![])])),
        (
            "[10,50,55,250]",
            List(vec![Num(10), Num(50), Num(55), Num(250)]),
        ),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    for (case, expected) in cases {
        assert_eq!(
            expected,
            line(case).unwrap_or_else(|_| panic!("failed to parse: {}", case))
        );
    }
}
