//! LESSON 05 — Borrow and borrowed lookup
//! Run: cargo test --test chapter13 l05_
//!
//! PREDICT before executing: Why can HashMap<String,_> be queried by &str? Would a
//! case-insensitive key be allowed to Borrow<str> with normal str equality?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Explain the stronger equality/order/hash contract of Borrow compared
//! with AsRef. What does BorrowMut add?
//!
//! COMPILER / MANUAL LAB (use examples/ch13_scratch.rs):
//! Implement a wrapper with Borrow<str> and matching derived equality/hash, then query a
//! HashMap with &str. Propose a case-insensitive key and identify why borrowing normal str
//! would violate its identity contract.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
/// A. Query owned string keys without allocating an owned lookup key.
fn lookup(map: &HashMap<String, u32>, name: &str) -> Option<u32> {
    todo!("Task 1: implement the documented contract")
}
/// B. Accept any key that can borrow str; return its UTF-8 byte length.
fn key_length<K: Borrow<str>>(key: &K) -> usize {
    todo!("Task 2: implement the documented contract")
}
/// C. Append to a borrowed mutable String. This operates on standalone values,
/// never on a key inside a map (which would break a container invariant).
fn append_owned<B: BorrowMut<String>>(value: &mut B, suffix: &str) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_lookup_uses_borrowed_identity() {
        let map = HashMap::from([("é".into(), 7)]);
        assert_eq!(lookup(&map, "é"), Some(7));
        assert_eq!(lookup(&map, "e"), None);
    }
    #[test]
    fn b_borrow_exposes_the_same_key_identity() {
        assert_eq!(key_length(&String::from("🦀")), 4);
        assert_eq!(key_length(&"abc"), 3);
    }
    #[test]
    fn c_mutable_borrow_edits_an_owned_value() {
        let mut s = String::from("a");
        append_owned(&mut s, "β");
        assert_eq!(s, "aβ");
    }
}
