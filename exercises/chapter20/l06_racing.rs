//! LESSON 06 — Polling a race
//! Run: cargo test --test chapter20 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
//! 1. If both futures are ready, which should this left-biased race choose?
//! 2. Must a Pending branch arrange to be woken?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain how losing futures are dropped, and why this small model does not cover tokio::select! syntax, fairness, task aborts, or timers.
//! COMPILER LAB (examples/ch20_scratch.rs): Give both branches a Drop counter. Poll the race once with a left-ready/right-pending pair, then compare dropping an unpolled race.

use std::{
    future::{Future, poll_fn},
    task::Poll,
};
/// A. Race two same-output futures, checking left first on each poll. Pin both
/// without requiring Unpin. Return Pending only after polling both Pending.
async fn race<L, R, T>(left: L, right: R) -> T
where
    L: Future<Output = T>,
    R: Future<Output = T>,
{
    todo!("A: implement the contract")
}
/// B. Await the race, returning only a successful value. Preserve the winner's
/// error; do not fall back to another branch after a ready error.
async fn race_result<L, R, T, E>(left: L, right: R) -> Result<T, E>
where
    L: Future<Output = Result<T, E>>,
    R: Future<Output = Result<T, E>>,
{
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::{
        cell::Cell,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
        task::{Context, Wake, Waker},
    };
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
    /// A future that counts how often it is polled and is ready from poll `ready_at` on.
    fn counted<'a>(
        polls: &'a Cell<u32>,
        ready_at: u32,
        value: u32,
    ) -> impl Future<Output = u32> + 'a {
        poll_fn(move |_| {
            polls.set(polls.get() + 1);
            if polls.get() >= ready_at {
                Poll::Ready(value)
            } else {
                Poll::Pending
            }
        })
    }
    #[test]
    fn a_race_is_left_biased_and_accepts_pending_peer() {
        assert_eq!(ready(race(std::future::ready(1), std::future::ready(2))), 1);
        assert_eq!(
            ready(race(std::future::pending::<u32>(), std::future::ready(9))),
            9
        );
    }
    #[test]
    fn a_both_pending_stays_pending() {
        let mut f = std::pin::pin!(race(
            std::future::pending::<u32>(),
            std::future::pending::<u32>()
        ));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(f.as_mut().poll(&mut cx).is_pending());
    }
    #[test]
    fn a_left_is_polled_first_on_every_poll() {
        let (left, right) = (Cell::new(0), Cell::new(0));
        let mut f = std::pin::pin!(race(counted(&left, 3, 1), counted(&right, 99, 2)));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(f.as_mut().poll(&mut cx).is_pending());
        assert!(f.as_mut().poll(&mut cx).is_pending());
        assert_eq!(
            (left.get(), right.get()),
            (2, 2),
            "Pending only after polling both"
        );
        assert_eq!(f.as_mut().poll(&mut cx), Poll::Ready(1));
        assert_eq!(
            (left.get(), right.get()),
            (3, 2),
            "a ready left branch wins without polling right"
        );
    }
    #[test]
    fn a_race_accepts_futures_that_are_not_unpin() {
        let text = String::from("borrowed across an await");
        let left = async {
            let r = &text;
            std::future::pending::<()>().await;
            r.len()
        };
        let right = async {
            let r = &text;
            std::future::ready(()).await;
            r.len() * 2
        };
        assert_eq!(ready(race(left, right)), 48);
    }
    #[test]
    fn b_race_result_keeps_the_winners_error() {
        assert_eq!(
            ready(race_result(
                std::future::ready(Err::<u32, _>("bad")),
                std::future::ready(Ok(2))
            )),
            Err("bad")
        );
        assert_eq!(
            ready(race_result(
                std::future::pending::<Result<u32, &str>>(),
                std::future::ready(Err("right failed"))
            )),
            Err("right failed")
        );
        assert_eq!(
            ready(race_result(
                std::future::ready(Ok::<u32, &str>(5)),
                std::future::ready(Err("ignored"))
            )),
            Ok(5)
        );
    }

    struct CountWakes(AtomicUsize);
    impl Wake for CountWakes {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. How many wakeups does each future request when polled once?
        let wakes = Arc::new(CountWakes(AtomicUsize::new(0)));
        let waker = Waker::from(Arc::clone(&wakes));
        let mut cx = Context::from_waker(&waker);
        let _ = std::pin::pin!(std::future::pending::<()>()).poll(&mut cx);
        let after_pending = wakes.0.load(Ordering::Relaxed);
        let _ = std::pin::pin!(poll_fn(|cx| {
            cx.waker().wake_by_ref();
            Poll::<()>::Pending
        }))
        .poll(&mut cx);
        let after_yield = wakes.0.load(Ordering::Relaxed) - after_pending;
        let p1: (usize, usize) = guess();
        assert_eq!(p1, (after_pending, after_yield), "P1: who arranges a wake?");

        // P2. poll_fn has no memory of finishing. What do four polls return?
        let mut n = 0;
        let mut f = std::pin::pin!(poll_fn(|_| {
            n += 1;
            if n < 3 { Poll::Pending } else { Poll::Ready(n) }
        }));
        let mut cx = Context::from_waker(Waker::noop());
        let polls: Vec<Poll<u32>> = (0..4).map(|_| f.as_mut().poll(&mut cx)).collect();
        let p2: Vec<Poll<u32>> = guess();
        assert_eq!(p2, polls, "P2: polling after Ready");
    }
}
