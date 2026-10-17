//! LESSON 05 — Readiness registration model
//! Run: cargo test --test chapter21 l05_
//!
//! PREDICT before running:
//! 1. Can readiness become stale before the actual I/O attempt?
//! 2. Does readiness mean a whole application message is available?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Separate a readiness signal from I/O completion. Explain registering a waker under the same lock as the ready flag.
//! COMPILER LAB (examples/ch21_scratch.rs): Write the real reactor lab using nonblocking loopback I/O. Record WouldBlock followed by registration and a later wake.

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};
#[derive(Default)]
struct Event {
    ready: bool,
    waiter: Option<Waker>,
}
struct Readiness(Arc<Mutex<Event>>);
impl Future for Readiness {
    type Output = ();
    /// A. If ready, return Ready without clearing the flag; otherwise register
    /// the latest waker and return Pending. This is a single-waiter model.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        todo!("A: implement the contract")
    }
}
/// B. Mark ready and take the waiter under the lock; wake outside the lock.
/// Repeated readiness signals without a waiter are harmless.
fn signal(event: &Arc<Mutex<Event>>) {
    todo!("B: implement the contract")
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
    fn readiness_wakes_registered_task() {
        let event = Arc::new(Mutex::new(Event::default()));
        let mut f = Readiness(Arc::clone(&event));
        let (c, w) = counted();
        let mut cx = Context::from_waker(&w);
        assert!(Pin::new(&mut f).poll(&mut cx).is_pending());
        signal(&event);
        assert_eq!(c.0.load(Ordering::Relaxed), 1);
        assert!(Pin::new(&mut f).poll(&mut cx).is_ready());
        signal(&event);
        assert_eq!(c.0.load(Ordering::Relaxed), 1);
    }
    #[test]
    fn early_readiness_is_remembered() {
        let event = Arc::new(Mutex::new(Event::default()));
        signal(&event);
        let mut f = Readiness(event);
        assert!(
            Pin::new(&mut f)
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_ready()
        );
    }
}
