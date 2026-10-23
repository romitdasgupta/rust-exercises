//! LESSON 02 — Pending and wakeups
//! Run: cargo test --test chapter21 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    use std::task::{Wake, Waker};
    #[test]
    #[allow(clippy::waker_clone_wake)] // deliberately exercises the consuming `wake`
    fn a_b_c_each_pending_transition_requests_a_repoll() {
        let s = Arc::new(Signal(AtomicUsize::new(0)));
        let w = Waker::from(Arc::clone(&s));
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
    #[test]
    fn a_b_c_zero_is_ready_without_a_wake() {
        let s = Arc::new(Signal(AtomicUsize::new(0)));
        let w = Waker::from(Arc::clone(&s));
        let mut f = std::pin::pin!(Countdown(0));
        assert_eq!(
            f.as_mut().poll(&mut Context::from_waker(&w)),
            Poll::Ready(42)
        );
        assert_eq!(s.0.load(Ordering::Relaxed), 0);
    }
    #[test]
    fn a_b_c_one_decrement_per_poll() {
        let s = Arc::new(Signal(AtomicUsize::new(0)));
        let w = Waker::from(Arc::clone(&s));
        let mut cx = Context::from_waker(&w);
        let mut f = std::pin::pin!(Countdown(255));
        let polls = (1..=300)
            .find(|_| f.as_mut().poll(&mut cx).is_ready())
            .expect("Countdown(255) never finished");
        assert_eq!(polls, 256);
        assert_eq!(s.0.load(Ordering::Relaxed), 255);
    }
    #[test]
    #[allow(clippy::waker_clone_wake)] // deliberately exercises the consuming `wake`
    fn b_c_waking_keeps_the_arc_balanced() {
        let s = Arc::new(Signal(AtomicUsize::new(0)));
        let w = Waker::from(Arc::clone(&s));
        w.wake_by_ref();
        assert_eq!(
            Arc::strong_count(&s),
            2,
            "wake_by_ref must not consume or leak"
        );
        w.clone().wake();
        assert_eq!(Arc::strong_count(&s), 2, "wake consumes its own Arc");
        drop(w);
        assert_eq!(Arc::strong_count(&s), 1);
        assert_eq!(s.0.load(Ordering::Relaxed), 2);
    }

    struct Task(AtomicUsize);
    impl Wake for Task {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Strong counts of `task`: after making a Waker, after cloning the
        // Waker, and after the clone's consuming wake().
        let task = Arc::new(Task(AtomicUsize::new(0)));
        let waker = Waker::from(Arc::clone(&task));
        let made = Arc::strong_count(&task);
        let copy = waker.clone();
        let cloned = Arc::strong_count(&task);
        copy.wake();
        let woken = Arc::strong_count(&task);
        let p1: (usize, usize, usize) = guess();
        assert_eq!(p1, (made, cloned, woken), "P1: a Waker owns an Arc");

        // P2. The future wakes itself during its first poll, and the test wakes it
        // twice more. How many times has it been polled?
        let mut polls = 0;
        let mut f = std::pin::pin!(std::future::poll_fn(|cx| {
            polls += 1;
            cx.waker().wake_by_ref();
            Poll::<()>::Pending
        }));
        let mut cx = Context::from_waker(&waker);
        let _ = f.as_mut().poll(&mut cx);
        waker.wake_by_ref();
        waker.wake_by_ref();
        let p2: usize = guess();
        assert_eq!(p2, polls, "P2: waking schedules, it does not poll");
    }
}
