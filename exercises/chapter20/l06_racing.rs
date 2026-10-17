//! LESSON 06 — Polling a race
//! Run: cargo test --test chapter20 l06_
//!
//! PREDICT before running:
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
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
    #[test]
    fn race_is_left_biased_and_accepts_pending_peer() {
        assert_eq!(ready(race(std::future::ready(1), std::future::ready(2))), 1);
        assert_eq!(
            ready(race(std::future::pending::<u32>(), std::future::ready(9))),
            9
        );
        assert_eq!(
            ready(race_result(
                std::future::ready(Err::<u32, _>("bad")),
                std::future::ready(Ok(2))
            )),
            Err("bad")
        );
    }
    #[test]
    fn both_pending_stays_pending() {
        let mut f = std::pin::pin!(race(
            std::future::pending::<u32>(),
            std::future::pending::<u32>()
        ));
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        assert!(f.as_mut().poll(&mut cx).is_pending());
    }
}
