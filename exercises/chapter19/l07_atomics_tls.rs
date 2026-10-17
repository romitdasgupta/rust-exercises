//! LESSON 07 — Atomic counters and local state
//! Run: cargo test --test chapter19 l07_
//!
//! PREDICT before running:
//! 1. Does Relaxed synchronize unrelated data?
//! 2. Do two threads share the contents of one thread_local variable?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Distinguish atomicity, ordering, and per-thread storage. Explain why a mutable static would need an invariant beyond these counters.
//! COMPILER LAB (examples/ch19_scratch.rs): Try a mutable static counter in safe code; replace it with explicit per-run state or a synchronized OnceLock. Do not execute a data race.

use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};
/// A. Atomically reserve one ID by incrementing the counter; return the previous
/// value. At usize::MAX return None without wrapping. Relaxed is sufficient:
/// the counter allocates IDs, not publication of other memory.
fn reserve(counter: &AtomicUsize) -> Option<usize> {
    todo!("A: implement the contract")
}
/// B. Spawn n threads. Each has its own Cell counter initialized at 0, increments
/// twice, and returns (old, new). Return results in spawn order. No global state.
fn local_counters(n: usize) -> Vec<(u32, u32)> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reservation_never_wraps() {
        let c = AtomicUsize::new(4);
        assert_eq!(reserve(&c), Some(4));
        assert_eq!(reserve(&c), Some(5));
        let max = AtomicUsize::new(usize::MAX);
        assert_eq!(reserve(&max), None);
        assert_eq!(max.load(Ordering::Relaxed), usize::MAX);
    }
    #[test]
    fn each_worker_starts_fresh() {
        assert_eq!(local_counters(3), vec![(0, 2); 3]);
        assert!(local_counters(0).is_empty());
    }
}
