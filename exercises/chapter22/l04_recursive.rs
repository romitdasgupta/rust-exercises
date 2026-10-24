//! LESSON 04 — Recursive structured literals
//! Run: cargo test --test chapter22 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can one tt match a whole bracket-delimited array? Why must special-case rules precede a general expression rule?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
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
    use practice::guess;
    #[test]
    fn a_b_c_d_recursive_literals_preserve_structure() {
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
    fn a_d_top_level_leaves() {
        let null: Data = data!(null);
        assert_eq!(null, Data::Null);
        let text: Data = data!("naïve 🦀");
        assert_eq!(text, Data::Text("naïve 🦀".into()));
        let n = 41_i64;
        let computed: Data = data!(n + 1);
        assert_eq!(computed, Data::Number(42));
    }
    #[test]
    fn b_c_empty_and_duplicate_objects() {
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
    #[test]
    fn b_c_nulls_nest_inside_containers() {
        let nested: Data = data!({"list": [null, [null]], "inner": {"none": null}});
        assert_eq!(
            nested,
            Data::Object(BTreeMap::from([
                (
                    "list".into(),
                    Data::Array(vec![Data::Null, Data::Array(vec![Data::Null])])
                ),
                (
                    "inner".into(),
                    Data::Object(BTreeMap::from([("none".into(), Data::Null)]))
                ),
            ]))
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. How many token trees are in `[1, 2] {a} (b c) d`?
        macro_rules! count_tts {
            ($($t:tt)*) => { 0 $(+ { let _ = stringify!($t); 1 })* };
        }
        let p1: i32 = guess();
        assert_eq!(p1, count_tts!([1, 2] {a} (b c) d), "P1: delimited groups");

        // P2. The general rule comes first here. What does `null` match?
        macro_rules! general_first {
            ($e:expr) => {
                "expr"
            };
            (null) => {
                "null"
            };
        }
        let p2: &str = guess();
        assert_eq!(p2, general_first!(null), "P2: rule order");

        // P3. Forward `null` to a matcher as tt, then as expr. Is it null each time?
        macro_rules! is_null {
            (null) => {
                true
            };
            ($e:expr) => {
                false
            };
        }
        macro_rules! via_tt {
            ($v:tt) => {
                is_null!($v)
            };
        }
        macro_rules! via_expr {
            ($v:expr) => {
                is_null!($v)
            };
        }
        let p3: (bool, bool) = guess();
        assert_eq!(
            p3,
            (via_tt!(null), via_expr!(null)),
            "P3: forwarding fragments"
        );

        // P4. What is the FIRST token tree of `-2`, as text?
        macro_rules! first_tt {
            ($first:tt $($rest:tt)*) => {
                stringify!($first)
            };
        }
        let p4: &str = guess();
        assert_eq!(p4, first_tt!(-2), "P4: negative numbers");
    }
}
