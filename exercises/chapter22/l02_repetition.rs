//! LESSON 02 — Repetition and separators
//! Run: cargo test --test chapter22 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: How many repetitions does * accept? Can a trailing comma match without adding a final element?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
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
    use practice::guess;
    use std::cell::RefCell;
    #[test]
    fn a_map_evaluates_pairs_in_order_and_allows_trailing_comma() {
        let mut n = 0;
        let map: BTreeMap<&str, i32> = ordered_map!("b"=>{n+=1;n},"a"=>{n+=1;n},"b"=>{n+=1;n},);
        assert_eq!(map, BTreeMap::from([("a", 2), ("b", 3)]));
        assert_eq!(n, 3);
        let empty: BTreeMap<u8, u8> = ordered_map!();
        assert!(empty.is_empty());
    }
    #[test]
    fn a_keys_and_values_interleave_in_source_order() {
        let log = RefCell::new(Vec::new());
        let note = |s: &'static str| {
            log.borrow_mut().push(s);
            s
        };
        let map: BTreeMap<&str, &str> =
            ordered_map!(note("k1") => note("v1"), note("k2") => note("v2"));
        assert_eq!(map.len(), 2);
        assert_eq!(*log.borrow(), ["k1", "v1", "k2", "v2"]);
    }
    #[test]
    fn b_counting_ignores_expressions() {
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
    #[test]
    fn b_commas_inside_an_expression_do_not_count() {
        let n: usize = count_syntax!(i32::max(1, 2), [3, 4, 5], (6, 7));
        assert_eq!(n, 3);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. This counts token trees. How many does `a, b, c` contain?
        macro_rules! count_tt {
            () => { 0usize };
            ($head:tt $($tail:tt)*) => { 1usize + count_tt!($($tail)*) };
        }
        let p1: usize = guess();
        assert_eq!(p1, count_tt!(a, b, c), "P1: what counts as a token tree");

        // P2. `vec![elem; n]`: how often does the element expression run?
        let mut calls = 0;
        let v = vec![
            {
                calls += 1;
                calls
            };
            3
        ];
        let p2: (i32, Vec<i32>) = guess();
        assert_eq!(p2, (calls, v), "P2: how vec![elem; n] builds its elements");

        // P3. Nested repetition: one sum per bracket group, including the empty one.
        macro_rules! sums {
            ($([$($x:expr),*]),*) => { [$(0 $(+ $x)*),*] };
        }
        let p3: Vec<i32> = guess();
        assert_eq!(p3, sums!([1, 2], [], [3]), "P3: inner and outer repetition");
    }
}
