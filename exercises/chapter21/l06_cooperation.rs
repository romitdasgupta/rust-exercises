//! LESSON 06 — Cooperative work budgets
//! Run: cargo test --test chapter21 l06_
//!
//! PREDICT before running:
//! 1. Can a future starve peers even though it returns a Future type?
//! 2. Does a runtime preempt arbitrary CPU loops inside poll?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain bounded work per poll and why a self-wake after progress is appropriate here.
//! COMPILER LAB (examples/ch21_scratch.rs): Compare budgets 1, 3, and 100 by counting poll calls for the same input. Explain why a poll count is not a timing benchmark.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
struct BudgetSum<'a> {
    values: &'a [u32],
    position: usize,
    total: u64,
    budget: usize,
}
impl Future for BudgetSum<'_> {
    type Output = u64;
    /// A. Process at most budget elements per poll, widening before addition.
    /// If unfinished wake and return Pending; else return Ready(total).
    /// Constructor guarantees budget > 0 and total fits u64.
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u64> {
        todo!("A: implement the contract")
    }
}
/// B. Construct with a nonzero budget, rejecting zero as None.
fn budget_sum(values: &[u32], budget: usize) -> Option<BudgetSum<'_>> {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    struct Counter(AtomicUsize);
    impl std::task::Wake for Counter {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
        fn wake_by_ref(self: &Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    fn counted() -> (Arc<Counter>, std::task::Waker) {
        let c = Arc::new(Counter(AtomicUsize::new(0)));
        let w = std::task::Waker::from(Arc::clone(&c));
        (c, w)
    }
    #[test]
    fn work_is_bounded_and_wakes_only_when_needed() {
        let mut f = budget_sum(&[1, 2, 3, 4, 5], 2).unwrap();
        let (c, w) = counted();
        let mut cx = Context::from_waker(&w);
        assert_eq!(Pin::new(&mut f).poll(&mut cx), Poll::Pending);
        assert_eq!(f.position, 2);
        assert_eq!(Pin::new(&mut f).poll(&mut cx), Poll::Pending);
        assert_eq!(f.position, 4);
        assert_eq!(Pin::new(&mut f).poll(&mut cx), Poll::Ready(15));
        assert_eq!(c.0.load(Ordering::Relaxed), 2);
    }
    #[test]
    fn empty_and_invalid_budget() {
        assert!(budget_sum(&[], 0).is_none());
        let mut f = budget_sum(&[], 1).unwrap();
        assert_eq!(
            Pin::new(&mut f).poll(&mut Context::from_waker(std::task::Waker::noop())),
            Poll::Ready(0)
        );
    }
}
