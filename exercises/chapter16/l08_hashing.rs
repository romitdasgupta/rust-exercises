//! LESSON 08 — Hashing and custom builders
//! Run: cargo test --test chapter16 l08_
//!
//! PREDICT before executing: Can unequal values have equal hashes? Should DefaultHasher
//! output be persisted as a stable file identifier?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why does a collision not change map correctness? Why would this
//! deliberately weak teaching hasher be inappropriate for untrusted keys?
//!
//! COMPILER / MANUAL LAB (use examples/ch16_scratch.rs):
//! Implement a rolling byte hasher in scratch and create
//! HashMap<K,V,BuildHasherDefault<H>>. Check equal keys and collisions without relying on
//! the order or byte chunking used by Hash for arbitrary types. Compare
//! security/performance tradeoffs with RandomState.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
#[derive(Default)]
struct ByteSum(u64);
impl Hasher for ByteSum {
    /// A. Sum all bytes across all write calls with wrapping u64 arithmetic.
    fn write(&mut self, bytes: &[u8]) {
        todo!("Task 1: implement the documented contract")
    }
    /// B. Return current state without changing it.
    fn finish(&self) -> u64 {
        todo!("Task 2: implement the documented contract")
    }
}
/// C. Count u8 keys using this custom builder. Collisions must remain correct.
fn counts(values: &[u8]) -> HashMap<u8, usize, BuildHasherDefault<ByteSum>> {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_streaming_hash_handles_multiple_writes_and_wraps() {
        let mut h = ByteSum(u64::MAX - 1);
        h.write(&[1, 2]);
        h.write(&[3]);
        assert_eq!(h.0, 4);
    }
    #[test]
    fn b_finish_is_repeatable() {
        let h = ByteSum(9);
        assert_eq!(h.finish(), 9);
        assert_eq!(h.finish(), 9);
    }
    #[test]
    fn c_custom_map_keeps_distinct_keys_separate() {
        let m = counts(&[1, 2, 1, 0, 255]);
        assert_eq!(m.get(&1), Some(&2));
        assert_eq!(m.get(&2), Some(&1));
        assert_eq!(m.len(), 4);
    }
}
