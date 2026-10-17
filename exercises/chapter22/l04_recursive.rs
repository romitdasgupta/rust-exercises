//! LESSON 04 — Recursive structured literals
//! Run: cargo test --test chapter22 l04_
//!
//! PREDICT before running:
//! 1. Can one tt match a whole bracket-delimited array?
//! 2. Why must special-case rules precede a general expression rule?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain the recursive grammar and its deliberate limitations (negative/compound expressions must be grouped).
//! COMPILER LAB (examples/ch22_scratch.rs): Add nested empty arrays/objects, then try an ungrouped negative value inside an array. Repair with parentheses; do not pretend this is a full JSON parser.

use std::collections::BTreeMap;
#[derive(Debug, PartialEq)]
enum Data {
    Null,
    Bool(bool),
    Number(i64),
    Text(String),
    Array(Vec<Data>),
    Object(BTreeMap<String, Data>),
}
impl From<bool> for Data {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<i64> for Data {
    fn from(v: i64) -> Self {
        Self::Number(v)
    }
}
impl From<&str> for Data {
    fn from(v: &str) -> Self {
        Self::Text(v.to_owned())
    }
}
/// A. Expand null, nested arrays, objects with literal keys, and expression
/// leaves. Delimited nested values are one tt. Empty containers/trailing commas
/// work. Object keys become Strings; a duplicate key's last value wins.
macro_rules! data {
    (null) => {{ todo!("A: implement the contract") }};
    ([$($value:tt),* $(,)?]) => {{ todo!("B: implement the contract") }};
    ({$($key:literal : $value:tt),* $(,)?}) => {{ todo!("C: implement the contract") }};
    ($other:expr) => {{ todo!("D: implement the contract") }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recursive_literals_preserve_structure() {
        let value: Data = data!([null,true,7,"hi",[(-2),],{"n":(1+2),},]);
        assert_eq!(
            value,
            Data::Array(vec![
                Data::Null,
                Data::Bool(true),
                Data::Number(7),
                Data::Text("hi".into()),
                Data::Array(vec![Data::Number(-2)]),
                Data::Object(BTreeMap::from([("n".into(), Data::Number(3))]))
            ])
        );
    }
    #[test]
    fn empty_and_duplicate_objects() {
        let empty: Data = data!({});
        assert_eq!(empty, Data::Object(BTreeMap::new()));
        let repeated: Data = data!({"a":1,"a":2});
        assert_eq!(
            repeated,
            Data::Object(BTreeMap::from([("a".into(), Data::Number(2))]))
        );
        let array: Data = data!([]);
        assert_eq!(array, Data::Array(vec![]));
    }
}
