//! LESSON 02 — Repetition and separators
//! Run: cargo test --test chapter22 l02_
//!
//! PREDICT before running:
//! 1. How many repetitions does * accept?
//! 2. Can a trailing comma match without adding a final element?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain nesting/separator syntax and the type-inference issue for an empty map.
//! COMPILER LAB (examples/ch22_scratch.rs): Change * to + and compare diagnostics for an empty invocation. Add an invalid missing-arrow pair and explain which rule failed.

use std::collections::BTreeMap;
/// A. Build a BTreeMap from zero or more key => value pairs. Allow one trailing
/// comma; evaluate each key and value once in source order. Last duplicate wins.
macro_rules! ordered_map {
    ($($key:expr => $value:expr),* $(,)?) => {{ todo!("A: implement the contract") }};
}
/// B. Count expression arguments WITHOUT evaluating them. Empty invocation is 0.
/// Each expression may have side effects, so use stringify! or unit replacement.
macro_rules! count_syntax {
    ($($value:expr),* $(,)?) => {{ todo!("B: implement the contract") }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn map_evaluates_pairs_in_order_and_allows_trailing_comma() {
        let mut n = 0;
        let map: BTreeMap<&str, i32> = ordered_map!("b"=>{n+=1;n},"a"=>{n+=1;n},"b"=>{n+=1;n},);
        assert_eq!(map, BTreeMap::from([("a", 2), ("b", 3)]));
        assert_eq!(n, 3);
        let empty: BTreeMap<u8, u8> = ordered_map!();
        assert!(empty.is_empty());
    }
    #[test]
    fn counting_ignores_expressions() {
        let calls = std::cell::Cell::new(0);
        let n: usize = count_syntax!(
            {
                calls.set(1);
                0
            },
            panic!("must not run"),
        );
        assert_eq!(n, 2);
        assert_eq!(calls.get(), 0);
        let zero: usize = count_syntax!();
        assert_eq!(zero, 0);
    }
}
