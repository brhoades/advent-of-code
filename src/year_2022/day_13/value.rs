#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    List(Vec<Value>),
    Num(u8),
}

pub use Value::*;
