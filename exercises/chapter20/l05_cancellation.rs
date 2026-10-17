//! LESSON 05 — Cancellation by dropping
//! Run: cargo test --test chapter20 l05_
//!
//! PREDICT before running:
//! 1. Does dropping a not-yet-polled future run code at the start of its body?
//! 2. Are already-captured values still dropped?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Distinguish captured resources from resources created after polling. Explain what cancellation does to local guards.
//! COMPILER LAB (examples/ch20_scratch.rs): Construct a future owning a guard, then drop it unpolled. Compare a guard constructed inside the async body; record which Drop counters change.

use std::{
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
struct Ticket {
    id: usize,
    dropped: Arc<AtomicUsize>,
}
impl Drop for Ticket {
    fn drop(&mut self) {
        self.dropped.fetch_add(1, Ordering::Relaxed);
    }
}
/// A. Keep ticket alive across waiting. On completion return its id. Both normal
/// completion and dropping a suspended future must release ticket exactly once.
async fn while_waiting<F: Future<Output = ()>>(ticket: Ticket, waiting: F) -> usize {
    todo!("A: implement the contract")
}
/// B. Await an optional result, then add it to total; None leaves total unchanged.
/// There must be no mutation before the await completes, so cancellation while
/// suspended cannot leave a partially applied update. Inputs do not overflow.
async fn apply_when_ready<F: Future<Output = Option<u32>>>(total: &mut u32, input: F) -> bool {
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
    fn suspended_future_owns_and_releases_ticket() {
        let dropped = Arc::new(AtomicUsize::new(0));
        let ticket = Ticket {
            id: 7,
            dropped: Arc::clone(&dropped),
        };
        let mut f = Box::pin(while_waiting(ticket, std::future::pending()));
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        assert!(f.as_mut().poll(&mut cx).is_pending());
        assert_eq!(dropped.load(Ordering::Relaxed), 0);
        drop(f);
        assert_eq!(dropped.load(Ordering::Relaxed), 1);
    }
    #[test]
    fn completed_future_releases_ticket() {
        let dropped = Arc::new(AtomicUsize::new(0));
        assert_eq!(
            ready(while_waiting(
                Ticket {
                    id: 8,
                    dropped: Arc::clone(&dropped)
                },
                std::future::ready(())
            )),
            8
        );
        assert_eq!(dropped.load(Ordering::Relaxed), 1);
    }
    #[test]
    fn cancellation_does_not_partially_apply() {
        let mut total = 10;
        {
            let mut f = Box::pin(apply_when_ready(&mut total, std::future::pending()));
            let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
            assert!(f.as_mut().poll(&mut cx).is_pending());
        }
        assert_eq!(total, 10);
        assert!(ready(apply_when_ready(
            &mut total,
            std::future::ready(Some(4))
        )));
        assert_eq!(total, 14);
        assert!(!ready(apply_when_ready(
            &mut total,
            std::future::ready(None)
        )));
    }
}
