use std::cmp::{Ordering, Ordering::*};

use super::value::{Value, Value::*};

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match list_order_correct_inner(self, other) {
            Some(true) => Less,
            Some(false) => Greater,
            _ => Equal,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match list_order_correct_inner(self, other) {
            Some(true) => Some(Less),
            Some(false) => Some(Greater),
            _ => Some(Equal),
        }
    }
}

#[cfg(test)]
pub fn list_order_correct(left: &Value, right: &Value) -> bool {
    list_order_correct_inner(left, right).expect("unexpected case")
}

// recursively calls itself to compare two values. None is returned
// if no decision is made. Some bool otherwise.
fn list_order_correct_inner(left: &Value, right: &Value) -> Option<bool> {
    let (l, r) = match (left, right) {
        (List(l), List(r)) => (l, r),
        // makes the example fail, but still getting the original problem wrong.
        (Num(l), r @ List(_)) => return list_order_correct_inner(&List(vec![Num(*l)]), r),
        (l @ List(_), Num(r)) => return list_order_correct_inner(l, &List(vec![Num(*r)])),
        (Num(l), Num(r)) if r == l => return None,
        (Num(l), Num(r)) => return Some(l < r),
    };

    for (l, r) in l.iter().zip(r.iter()) {
        if let Some(b) = list_order_correct_inner(l, r) {
            return Some(b);
        }
    }

    // if there's remaining left input (len(l) > len(r)), false
    // otherwise inconclusive
    if l.len() != r.len() {
        Some(l.len() < r.len())
    } else {
        None
    }
}
