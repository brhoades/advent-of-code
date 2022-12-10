use std::collections::{VecDeque, HashMap};
use std::str::FromStr;

use anyhow::Result;
use anyhow::anyhow;

pub fn run(input: String) -> Result<()> {
    let (stack_input, order_input) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("invalid input format"))?;

    let mut st = build_stack(stack_input)?;

    let orders = parse_orders(order_input)?;

    println!("input stack:");
    print_stack(st.iter());
    println!("\n");

    for order in orders {
        println!("");

        println!("{} crate(s): {} => {}", order.count, order.from, order.to);
        let mut crates: Vec<_> = st.get_mut(&order.from)
            .unwrap()
            .drain(0..(order.count as usize))
            .collect(); // so we can borrow again to insert

        // NOTE: don't want to deal with args rn.
        // PART ONE: comment the line below
        crates.reverse(); // now move multiple crates "all at once"

        // crates are moved one-by-one, reversing them
        let dest = st.get_mut(&order.to).unwrap();
        for cr in crates {
            dest.push_front(cr);
        }

        println!("");
        print_stack(st.iter());
    }

    println!("\n=====================");

    let mut sol: Vec<_> = st.iter().collect();
    sol.sort_by_cached_key(|(k, _)| k.parse::<i32>().unwrap());

    println!("solution:");
    print_stack(sol.iter());

    let ans: String = sol
        .into_iter()
        .map(|(_, v)| v.front().expect("unexpected empty stack").trim_matches(|c| c == '[' || c == ']'))
        .fold(String::new(), |a, b| a + b);
    println!("top of all stacks: '{}'", ans);

    Ok(())
}


// builds a stack by using a guarantee of retangular input to consume
// 4 byte chunks off the input
//
// A more efficient implementation would transform the input like a matrix,
// rotating it clockwise to walk the rows proceeded with numbers to build this stack.
// ... but I'm trying to have fun here.
fn build_stack(stack_input: &str) -> Result<HashMap<String, VecDeque<String>>> {
    // first, make the input retangular if there's missing trailing spaces
    let mut lines: Vec<String> = stack_input.lines().map(str::to_owned).collect();
    let max = lines.iter().max().unwrap().len();

    for line in &mut lines {
        for _ in 0..(max-line.len()) {
            line.push(' ');
        }
    }

    let input = lines.join("\n");

    // stack_input groups by four and fits either "[X] " or " #  "
    // puzzle input isn't multibyte utf8, bytes makes this easy
    let (values, labels): (Vec<_>, Vec<_>) = input
        .as_bytes()
        .chunks(4)
        .map(std::str::from_utf8)
        .map(|s| s.expect("input does not support multibyte utf8").trim())
        .partition(|s| *s == "" || s.parse::<i64>().is_err());

    let mut stack: HashMap<String, VecDeque<String>> = labels
        .clone()
        .into_iter()
        .map(str::to_owned)
        .map(|s| (s, Default::default()))
        .collect();
    let width = labels.len();
    let height = values.len() / width;

    for row in 0..height {
        for col in 0..width {
            let cell = values[row*width+col];
            if cell == "" {
                continue
            }

            stack.get_mut(labels[col]).unwrap().push_back(cell.to_owned())
        }
    }


    Ok(stack)
}


struct Order {
    count: i32,
    from: String,
    to: String,
}

fn parse_orders(input: &str) -> Result<Vec<Order>> {
    input
        .lines()
        .filter(|l| *l != "")
        .map(Order::from_str)
        .collect()
}

impl FromStr for Order {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // move # from # to #
        let mut segs = s.split(" ").filter_map(|t| t.parse::<i32>().ok());

        Ok(Order{
            count: segs.next().ok_or_else(|| anyhow!("invalid line format"))?,
            from: segs.next().ok_or_else(|| anyhow!("invalid line format"))?.to_string(),
            to: segs.next().ok_or_else(|| anyhow!("invalid line format"))?.to_string(),
        })
    }
}

fn print_stack<'a, S, I, V, It>(st: It)
where S: AsRef<str> + 'a,
    V: std::borrow::Borrow<VecDeque<String>> + 'a,
    I: std::borrow::Borrow<(S, V)>,
    It: Iterator<Item = I> {
    let mut output: Vec<_> = st.collect();
    output.sort_by_cached_key(|item| item.borrow().0.as_ref().parse::<i32>().unwrap());

    for entry in output {
        let (col, row) = entry.borrow();
        let row = row.borrow().iter().map(|s| s.to_owned()).collect::<Vec<String>>();
        println!("{}:\t{}", col.as_ref(), row.join(" "));
    }
}
