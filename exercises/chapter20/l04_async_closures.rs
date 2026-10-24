//! LESSON 04 — Lending async callbacks
//! Run: cargo test --test chapter20 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
//! 1. Can an async closure borrow mutable captured state across its await?
//! 2. Why can FnMut returning one fixed future type be too restrictive?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain why the callback bound is AsyncFnMut and what happens when the caller awaits each call before the next.
//! COMPILER LAB (examples/ch20_scratch.rs): Write an async closure that pushes into a captured Vec then awaits ready(()). Try passing it to a regular FnMut future factory and compare diagnostics.

use std::ops::AsyncFnMut;
/// A. Call a possibly stateful async callback once for each value, sequentially.
/// Collect callback results in order. The callback may borrow its own captures.
async fn map_async<F: AsyncFnMut(u32) -> u32>(values: &[u32], mut callback: F) -> Vec<u32> {
    todo!("A: implement the contract")
}
/// B. Use map_async with an async closure borrowing a running total. Return
/// running sums and the final total. Inputs guarantee u32 addition fits.
async fn running_sums(values: &[u32]) -> (Vec<u32>, u32) {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::{
        cell::RefCell,
        future::Future,
        pin::Pin,
        task::{Context, Poll, Waker},
    };
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
    /// Returns Pending once (waking itself first), then Ready.
    struct YieldOnce(bool);
    impl Future for YieldOnce {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if self.0 {
                return Poll::Ready(());
            }
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
    /// Polls with a no-op waker until Ready; returns the output and the poll count.
    fn drive<F: Future>(future: F) -> (F::Output, usize) {
        let mut f = std::pin::pin!(future);
        let mut cx = Context::from_waker(Waker::noop());
        for polls in 1..=1000 {
            if let Poll::Ready(x) = f.as_mut().poll(&mut cx) {
                return (x, polls);
            }
        }
        panic!("still pending after 1000 polls")
    }
    #[test]
    fn a_generic_callback_maps_in_order() {
        assert_eq!(ready(map_async(&[3, 1], async |v| v * 2)), [6, 2]);
        assert!(ready(map_async(&[], async |v| v)).is_empty());
    }
    #[test]
    fn a_callback_runs_once_per_value() {
        let mut calls = Vec::new();
        let out = ready(map_async(&[5, 6, 7], async |v| {
            calls.push(v);
            v + 1
        }));
        assert_eq!(out, [6, 7, 8]);
        assert_eq!(calls, [5, 6, 7]);
    }
    #[test]
    fn a_callbacks_run_one_at_a_time() {
        // Each callback is Pending once. Awaited one after another, three
        // callbacks need four polls of the outer future.
        let (out, polls) = drive(map_async(&[1, 2, 3], async |v| {
            YieldOnce(false).await;
            v * 10
        }));
        assert_eq!(out, [10, 20, 30]);
        assert_eq!(polls, 4, "await each callback before calling the next");
    }
    #[test]
    fn b_async_callback_can_lend_captured_state() {
        assert_eq!(ready(running_sums(&[2, 3, 0, 4])), (vec![2, 5, 5, 9], 9));
        assert_eq!(ready(running_sums(&[])), (vec![], 0));
        assert_eq!(ready(running_sums(&[7])), (vec![7], 7));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Two calls create two futures; `second` is run before `first`.
        let log = RefCell::new(Vec::new());
        let record = async |tag: &'static str| log.borrow_mut().push(tag);
        let first = record("first");
        let second = record("second");
        log.borrow_mut().push("made both");
        ready(second);
        ready(first);
        let p1: Vec<&str> = guess();
        assert_eq!(p1, *log.borrow(), "P1: calling versus polling");

        // P2. An `async move` closure returns its captured String. Same buffer?
        let s = String::from("captured");
        let before = s.as_ptr();
        let give_back = async move || s;
        let returned = ready(give_back());
        let p2: bool = guess();
        assert_eq!(
            p2,
            returned.as_ptr() == before,
            "P2: an AsyncFnOnce can move its captures out"
        );
    }
}
