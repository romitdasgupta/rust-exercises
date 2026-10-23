//! LESSON 07 — Atomic counters and local state
//! Run: cargo test --test chapter19 l07_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    use std::sync::atomic::AtomicU8;
    #[test]
    fn a_reservation_never_wraps() {
        let c = AtomicUsize::new(4);
        assert_eq!(reserve(&c), Some(4));
        assert_eq!(reserve(&c), Some(5));
        assert_eq!(c.load(Ordering::Relaxed), 6);
        let max = AtomicUsize::new(usize::MAX);
        assert_eq!(reserve(&max), None);
        assert_eq!(max.load(Ordering::Relaxed), usize::MAX);
        let near = AtomicUsize::new(usize::MAX - 1);
        assert_eq!(reserve(&near), Some(usize::MAX - 1));
        assert_eq!(reserve(&near), None);
    }
    #[test]
    fn a_concurrent_reservations_are_unique() {
        let counter = AtomicUsize::new(0);
        let mut ids: Vec<usize> = thread::scope(|s| {
            let workers: Vec<_> = (0..8)
                .map(|_| {
                    s.spawn(|| {
                        (0..2_000)
                            .map(|_| reserve(&counter).unwrap())
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            workers
                .into_iter()
                .flat_map(|w| w.join().unwrap())
                .collect()
        });
        ids.sort_unstable();
        assert!(
            ids.iter().copied().eq(0..16_000),
            "two threads received the same ID: the increment must be one atomic step"
        );
    }
    #[test]
    fn b_each_worker_starts_fresh() {
        assert_eq!(local_counters(3), vec![(0, 2); 3]);
        assert!(local_counters(0).is_empty());
        assert_eq!(local_counters(16), vec![(0, 2); 16]);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. fetch_add returns which value? What does the atomic hold afterwards?
        let a = AtomicUsize::new(5);
        let returned = a.fetch_add(3, Ordering::Relaxed);
        let p1: (usize, usize) = guess();
        assert_eq!(p1, (returned, a.load(Ordering::Relaxed)), "P1: fetch_add");

        // P2. Atomic arithmetic in a debug build: panic, saturate, or wrap?
        let small = AtomicU8::new(250);
        small.fetch_add(10, Ordering::Relaxed);
        let p2: u8 = guess();
        assert_eq!(p2, small.load(Ordering::Relaxed), "P2: atomic overflow");

        // P3. compare_exchange expects 1, but the value is 8.
        let c = AtomicUsize::new(8);
        let p3: Result<usize, usize> = guess();
        assert_eq!(
            p3,
            c.compare_exchange(1, 9, Ordering::Relaxed, Ordering::Relaxed),
            "P3: a failed compare_exchange"
        );

        // P4. This thread sets its thread-local to 5. What does a new thread see?
        thread_local! {
            static SEEN: Cell<u32> = const { Cell::new(0) };
        }
        SEEN.set(5);
        let other = thread::spawn(|| SEEN.get()).join().unwrap();
        let p4: (u32, u32) = guess();
        assert_eq!(p4, (SEEN.get(), other), "P4: one thread_local per thread");
    }
}
