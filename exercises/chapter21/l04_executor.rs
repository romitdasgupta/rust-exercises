//! LESSON 04 — A parker block_on
//! Run: cargo test --test chapter21 l04_
//!
//! PREDICT before running:
//! 1. Can unpark happen before park without losing the signal?
//! 2. Why does using Waker::noop in a blocking executor fail for a pending future?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain the wake token, spurious park returns, and the runtime features this tiny executor lacks.
//! COMPILER LAB (examples/ch21_scratch.rs): Try block_on on an immediately ready async block and on a self-waking one-yield future. Do not run an eternally Pending future.

use std::{
    future::Future,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
    thread,
};
struct WakeThread(thread::Thread);
impl Wake for WakeThread {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}
/// A. Pin future, make a waker for the current thread, then poll in a loop.
/// Ready returns output; Pending parks until a signal (and then rechecks).
/// Contract: the supplied future eventually becomes ready and wakes correctly.
/// This executor has no timers, task spawning, I/O reactor, or cancellation.
fn block_on<F: Future>(future: F) -> F::Output {
    todo!("A: implement the contract")
}
/// B. Return a future that yields Pending exactly once and wakes its caller,
/// then returns value. Use poll_fn with state captured by the closure.
fn yield_once<T>(value: T) -> impl Future<Output = T> {
    let mut yielded = false;
    let mut value = Some(value);
    std::future::poll_fn(move |cx| todo!("B: implement the contract"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn executor_handles_ready_and_wake_before_park() {
        assert_eq!(block_on(async { 3 }), 3);
        assert_eq!(block_on(yield_once(String::from("resumed"))), "resumed");
    }
    #[test]
    fn yielding_future_does_bounded_work() {
        let mut f = std::pin::pin!(yield_once(9));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(f.as_mut().poll(&mut cx).is_pending());
        assert_eq!(f.as_mut().poll(&mut cx), Poll::Ready(9));
    }
}
