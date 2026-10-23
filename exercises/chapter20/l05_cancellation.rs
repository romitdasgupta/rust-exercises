//! LESSON 05 — Cancellation by dropping
//! Run: cargo test --test chapter20 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    use std::cell::RefCell;
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
    #[test]
    fn a_suspended_future_owns_and_releases_ticket() {
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
    fn a_completed_future_releases_ticket() {
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
        let unpolled = while_waiting(
            Ticket {
                id: 9,
                dropped: Arc::clone(&dropped),
            },
            std::future::ready(()),
        );
        assert_eq!(dropped.load(Ordering::Relaxed), 1);
        drop(unpolled);
        assert_eq!(
            dropped.load(Ordering::Relaxed),
            2,
            "an unpolled future still owns its ticket"
        );
    }
    #[test]
    fn b_cancellation_does_not_partially_apply() {
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
        assert_eq!(total, 14);
        assert!(ready(apply_when_ready(
            &mut total,
            std::future::ready(Some(0))
        )));
        assert_eq!(total, 14);
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. A future captures `captured`, then is dropped without ever being polled.
        let log = RefCell::new(Vec::new());
        let log_ref = &log;
        let captured = Noisy("captured", log_ref);
        let f = async move {
            log_ref.borrow_mut().push("body");
            let _inner = Noisy("inner", log_ref);
            std::future::pending::<()>().await;
            drop(captured);
        };
        drop(f);
        let p1: Vec<&str> = guess();
        assert_eq!(p1, *log.borrow(), "P1: dropping an unpolled future");

        // P2. The same future, polled once (to the pending await) and then dropped.
        // Did "body" run, and how many Noisy values were dropped?
        let log = RefCell::new(Vec::new());
        let log_ref = &log;
        let captured = Noisy("captured", log_ref);
        let mut f = Box::pin(async move {
            log_ref.borrow_mut().push("body");
            let _inner = Noisy("inner", log_ref);
            std::future::pending::<()>().await;
            drop(captured);
        });
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        assert!(f.as_mut().poll(&mut cx).is_pending());
        drop(f);
        let body_ran = log.borrow().contains(&"body");
        let drops = log.borrow().iter().filter(|e| **e != "body").count();
        let p2: (bool, usize) = guess();
        assert_eq!(p2, (body_ran, drops), "P2: dropping a suspended future");

        // P3. Work before the first await is not undone by cancellation.
        let mut total = 0;
        {
            let mut f = Box::pin(async {
                total += 1;
                std::future::pending::<()>().await;
                total += 1;
            });
            assert!(f.as_mut().poll(&mut cx).is_pending());
        }
        let p3: i32 = guess();
        assert_eq!(p3, total, "P3: a partially applied update");
    }
}
