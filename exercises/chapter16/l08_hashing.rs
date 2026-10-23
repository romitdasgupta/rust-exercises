//! LESSON 08 — Hashing and custom builders
//! Run: cargo test --test chapter16 l08_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can unequal values have equal hashes? Should DefaultHasher
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
    use practice::guess;
    use std::hash::{BuildHasher, DefaultHasher, Hash, RandomState};
    #[test]
    fn a_streaming_hash_handles_multiple_writes_and_wraps() {
        let mut h = ByteSum(u64::MAX - 1);
        h.write(&[1, 2]);
        h.write(&[3]);
        assert_eq!(h.0, 4);
        let mut fresh = ByteSum::default();
        fresh.write(&[]);
        fresh.write(&[200, 100]);
        assert_eq!(fresh.0, 300, "bytes are summed as u64, not u8");
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
        assert_eq!(
            (m.get(&0), m.get(&255), m.get(&3)),
            (Some(&1), Some(&1), None)
        );
        assert!(counts(&[]).is_empty());
    }

    /// Sends every key to the same bucket: the worst possible hasher.
    #[derive(Default)]
    struct Zero;
    impl Hasher for Zero {
        fn write(&mut self, _: &[u8]) {}
        fn finish(&self) -> u64 {
            0
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Every key collides. How many entries does the map hold, and what
        // does it find for key 42?
        let mut map: HashMap<u32, u32, BuildHasherDefault<Zero>> = HashMap::default();
        for k in 0..100 {
            map.insert(k, k * 2);
        }
        let p1: (usize, Option<u32>) = guess();
        assert_eq!(p1, (map.len(), map.get(&42).copied()), "P1: collisions");

        // P2. Same value, same hasher setup: are the hashes equal?
        // (a) two DefaultHasher::new() states; (b) one RandomState used twice.
        let hash = |mut h: DefaultHasher| {
            "key".hash(&mut h);
            h.finish()
        };
        let state = RandomState::new();
        let p2: (bool, bool) = guess();
        assert_eq!(
            p2,
            (
                hash(DefaultHasher::new()) == hash(DefaultHasher::new()),
                state.hash_one("key") == state.hash_one("key")
            ),
            "P2: hashing is deterministic for one hasher setup"
        );

        // P3. How many bytes does the BuildHasherDefault<Zero> builder occupy?
        let p3: usize = guess();
        assert_eq!(
            p3,
            size_of::<BuildHasherDefault<Zero>>(),
            "P3: builder size"
        );
    }
}
