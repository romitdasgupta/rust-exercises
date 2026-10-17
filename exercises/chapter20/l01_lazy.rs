//! LESSON 01 — Lazy futures and await
//! Run: cargo test --test chapter20 l01_
//!
//! PREDICT before running:
//! 1. Does creating an async future increment a counter inside its body?
//! 2. Does awaiting twice mean two futures run concurrently?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Trace future creation, first polling, and await sequencing.
//! COMPILER LAB (examples/ch20_scratch.rs): Store futures from two different async blocks in a Vec. Explain the type mismatch; repair with a common async function or boxed trait objects.

use std::{
    future::Future,
    sync::atomic::{AtomicUsize, Ordering},
};
/// A. Increment on execution, returning the previous count. Merely creating
/// this future must have no effect. Use Relaxed; this is a local observation.
async fn next_count(counter: &AtomicUsize) -> usize {
    todo!("A: implement the contract")
}
/// B. Await both computations sequentially, preserving their returned order.
async fn sequence<A, B>(left: A, right: B) -> (A::Output, B::Output)
where
    A: Future,
    B: Future,
{
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
    #[test]
    fn creation_is_lazy() {
        let c = AtomicUsize::new(0);
        let f = next_count(&c);
        assert_eq!(c.load(Ordering::Relaxed), 0);
        assert_eq!(ready(f), 0);
        assert_eq!(c.load(Ordering::Relaxed), 1);
    }
    #[test]
    fn await_runs_in_sequence() {
        let c = AtomicUsize::new(5);
        assert_eq!(ready(sequence(next_count(&c), next_count(&c))), (5, 6));
        assert_eq!(c.load(Ordering::Relaxed), 7);
    }
}
