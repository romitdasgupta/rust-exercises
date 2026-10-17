//! LESSON 02 — Pending and wakeups
//! Run: cargo test --test chapter21 l02_
//!
//! PREDICT before running:
//! 1. Does waking a task immediately execute its poll method?
//! 2. What happens if Pending never arranges a wake?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain wake coalescing and why a future should not self-wake without making bounded progress.
//! COMPILER LAB (examples/ch21_scratch.rs): Poll a Countdown with Waker::noop and record each Poll. Then use the counted waker from the tests; compare count to pending transitions.

use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll},
};
struct Countdown(u8);
impl Future for Countdown {
    type Output = u8;
    /// A. At zero return Ready(42). Otherwise decrement once, wake_by_ref, and
    /// return Pending. Every poll does bounded work; completion needs no wake.
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u8> {
        todo!("A: implement the contract")
    }
}
struct Signal(AtomicUsize);
impl std::task::Wake for Signal {
    /// B. Record one scheduling signal. Consuming wake owns an Arc; wake_by_ref
    /// must leave the caller's Arc usable. Increment with Relaxed.
    fn wake(self: Arc<Self>) {
        todo!("B: implement the contract")
    }
    fn wake_by_ref(self: &Arc<Self>) {
        todo!("C: implement the contract")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn each_pending_transition_requests_a_repoll() {
        let s = Arc::new(Signal(AtomicUsize::new(0)));
        let w = std::task::Waker::from(Arc::clone(&s));
        let mut cx = Context::from_waker(&w);
        let mut f = std::pin::pin!(Countdown(2));
        assert_eq!(f.as_mut().poll(&mut cx), Poll::Pending);
        assert_eq!(f.as_mut().poll(&mut cx), Poll::Pending);
        assert_eq!(f.as_mut().poll(&mut cx), Poll::Ready(42));
        assert_eq!(s.0.load(Ordering::Relaxed), 2);
        w.clone().wake();
        w.wake_by_ref();
        assert_eq!(s.0.load(Ordering::Relaxed), 4);
    }
}
