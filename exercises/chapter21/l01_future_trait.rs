//! LESSON 01 — Implement Future
//! Run: cargo test --test chapter21 l01_
//!
//! PREDICT before running:
//! 1. Does poll consume a future?
//! 2. May a caller expect a completed future to be reusable?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain Output, Pin<&mut Self>, and why this implementation uses Unpin.
//! COMPILER LAB (examples/ch21_scratch.rs): Remove T: Unpin and try get_mut(). Explain the diagnostic. Compare Box<T> with Pin<Box<T>>.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
struct Once<T>(Option<T>);
impl<T: Unpin> Future for Once<T> {
    type Output = T;
    /// A. Return the stored value immediately, taking it exactly once. Panic
    /// with "polled after completion" if polled again; no self-wake is needed.
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        todo!("A: implement the contract")
    }
}
/// B. Allocate and pin a new one-shot future without manually invoking poll.
fn one_shot<T: Unpin>(value: T) -> Pin<Box<Once<T>>> {
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
    fn output_is_moved_exactly_once() {
        assert_eq!(ready(one_shot(String::from("owned"))), "owned");
        assert_eq!(ready(one_shot(7)), 7);
    }
    #[test]
    fn pin_box_can_move_its_handle() {
        let first = one_shot(vec![1, 2]);
        let address = &*first as *const _;
        let moved = first;
        assert_eq!(&*moved as *const _, address);
        assert_eq!(ready(moved), [1, 2]);
    }
}
