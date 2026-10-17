//! LESSON 04 — Lending async callbacks
//! Run: cargo test --test chapter20 l04_
//!
//! PREDICT before running:
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
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
    #[test]
    fn async_callback_can_lend_captured_state() {
        assert_eq!(ready(running_sums(&[2, 3, 0, 4])), (vec![2, 5, 5, 9], 9));
        assert_eq!(ready(running_sums(&[])), (vec![], 0));
    }
    #[test]
    fn generic_callback_maps_in_order() {
        assert_eq!(ready(map_async(&[3, 1], async |v| v * 2)), [6, 2]);
    }
}
