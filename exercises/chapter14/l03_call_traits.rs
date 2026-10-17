//! LESSON 03 — Choosing callable bounds
//! Run: cargo test --test chapter14 l03_
//!
//! PREDICT before executing: Which bound admits a stateful counter? Can a function pointer
//! satisfy Fn? Why can two closures with the same signature have different concrete types?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Explain why apply_once should not require Fn and why invoke_twice
//! cannot accept a closure that consumes a captured String.
//!
//! COMPILER / MANUAL LAB (use examples/ch14_scratch.rs):
//! Try storing two different closures directly in one Vec. Repair using Vec<Box<dyn
//! Fn(i32)->i32>>. Compare size_of_val for a zero-capture closure and a closure capturing
//! data; explain why this is not a performance benchmark.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Invoke once with no unnecessary restrictions on captured ownership.
fn apply_once<F: FnOnce() -> String>(f: F) -> String {
    todo!("Task 1: implement the documented contract")
}
/// B. Call stateful f on 2 then 5 and return both results.
fn invoke_twice<F: FnMut(i32) -> i32>(mut f: F) -> (i32, i32) {
    todo!("Task 2: implement the documented contract")
}
/// C. Read-only callable: apply to every input, preserving order.
fn transform<F: Fn(i32) -> i32>(values: &[i32], f: F) -> Vec<i32> {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_fn_once_allows_consuming_captures() {
        let s = String::from("once");
        assert_eq!(apply_once(move || s), "once");
    }
    #[test]
    fn b_fn_mut_allows_stateful_captures() {
        let mut total = 0;
        assert_eq!(
            invoke_twice(|n| {
                total += n;
                total
            }),
            (2, 7)
        );
        assert_eq!(total, 7);
    }
    #[test]
    fn c_fn_accepts_closures_and_function_pointers() {
        fn square(n: i32) -> i32 {
            n * n
        }
        assert_eq!(transform(&[1, -2, 3], square), [1, 4, 9]);
        let offset = 4;
        assert_eq!(transform(&[0, 2], |n| n + offset), [4, 6]);
        assert!(transform(&[], square).is_empty());
    }
}
