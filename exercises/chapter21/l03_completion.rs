//! LESSON 03 — A completion cell
//! Run: cargo test --test chapter21 l03_
//!
//! PREDICT before running:
//! 1. Must poll replace an old waker when the executor supplies a new one?
//! 2. Should a completion function invoke wake while holding its state mutex?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Trace a completion-before-poll and poll-before-completion interleaving. Explain why one locked state prevents lost wakeups.
//! COMPILER LAB (examples/ch21_scratch.rs): Replace a registered counted waker with another, complete, and inspect which counter changes. Discuss dropping a receiver before completion.

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};
struct State<T> {
    value: Option<T>,
    waiter: Option<Waker>,
}
struct Receive<T>(Arc<Mutex<State<T>>>);
struct Complete<T>(Arc<Mutex<State<T>>>);
/// A. Construct a single-producer/single-consumer cell. Complete is consumed
/// when fulfilled. Dropping an unfulfilled producer leaves a pending receiver;
/// this minimal model deliberately has no disconnect result.
fn completion<T>() -> (Complete<T>, Receive<T>) {
    todo!("A: implement the contract")
}
impl<T> Future for Receive<T> {
    type Output = T;
    /// B. Under one lock: take a present value, otherwise register the current
    /// waker and return Pending. Never block waiting for a value.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        todo!("B: implement the contract")
    }
}
impl<T> Complete<T> {
    /// C. Store the value under the lock, take the registered waker, unlock,
    /// then wake if present. One producer means no overwrite is possible.
    fn finish(self, value: T) {
        todo!("C: implement the contract")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    struct Counter(AtomicUsize);
    impl std::task::Wake for Counter {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
        fn wake_by_ref(self: &Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    fn counted() -> (Arc<Counter>, std::task::Waker) {
        let c = Arc::new(Counter(AtomicUsize::new(0)));
        let w = std::task::Waker::from(Arc::clone(&c));
        (c, w)
    }
    #[test]
    fn complete_before_poll_is_ready() {
        let (tx, mut rx) = completion();
        tx.finish(String::from("ok"));
        let mut cx = Context::from_waker(Waker::noop());
        assert_eq!(
            Pin::new(&mut rx).poll(&mut cx),
            Poll::Ready(String::from("ok"))
        );
    }
    #[test]
    fn latest_registered_waker_is_used() {
        let (tx, mut rx) = completion();
        let (a, wa) = counted();
        let (b, wb) = counted();
        assert!(
            Pin::new(&mut rx)
                .poll(&mut Context::from_waker(&wa))
                .is_pending()
        );
        assert!(
            Pin::new(&mut rx)
                .poll(&mut Context::from_waker(&wb))
                .is_pending()
        );
        tx.finish(17);
        assert_eq!(a.0.load(Ordering::Relaxed), 0);
        assert_eq!(b.0.load(Ordering::Relaxed), 1);
        assert_eq!(
            Pin::new(&mut rx).poll(&mut Context::from_waker(&wb)),
            Poll::Ready(17)
        );
    }
}
