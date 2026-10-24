//! LESSON 05 — Readiness registration model
//! Run: cargo test --test chapter21 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    use std::io::{ErrorKind, Read};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::Wake;
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
    fn a_b_readiness_wakes_registered_task() {
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
    fn a_b_early_readiness_is_remembered() {
        let event = Arc::new(Mutex::new(Event::default()));
        signal(&event);
        signal(&event);
        let mut f = Readiness(event);
        assert!(
            Pin::new(&mut f)
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_ready()
        );
    }
    #[test]
    fn a_ready_flag_is_not_cleared_by_polling() {
        let event = Arc::new(Mutex::new(Event {
            ready: true,
            waiter: None,
        }));
        let mut f = Readiness(Arc::clone(&event));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(Pin::new(&mut f).poll(&mut cx).is_ready());
        assert!(event.lock().unwrap().ready);
        assert!(Pin::new(&mut f).poll(&mut cx).is_ready());
    }
    #[test]
    fn a_b_latest_waker_replaces_the_old_one() {
        let event = Arc::new(Mutex::new(Event::default()));
        let mut f = Readiness(Arc::clone(&event));
        let (a, wa) = counted();
        let (b, wb) = counted();
        assert!(
            Pin::new(&mut f)
                .poll(&mut Context::from_waker(&wa))
                .is_pending()
        );
        assert!(
            Pin::new(&mut f)
                .poll(&mut Context::from_waker(&wb))
                .is_pending()
        );
        signal(&event);
        assert_eq!(
            (a.0.load(Ordering::Relaxed), b.0.load(Ordering::Relaxed)),
            (0, 1)
        );
        assert_eq!(
            Arc::strong_count(&a),
            2,
            "the replaced waker must be dropped"
        );
    }
    #[test]
    fn b_wake_happens_after_unlocking() {
        struct Probe(Arc<Mutex<Event>>, Mutex<Vec<bool>>);
        impl Wake for Probe {
            fn wake(self: Arc<Self>) {
                self.wake_by_ref();
            }
            fn wake_by_ref(self: &Arc<Self>) {
                let free = self.0.try_lock().is_ok();
                self.1.lock().unwrap().push(free);
            }
        }
        let event = Arc::new(Mutex::new(Event::default()));
        let probe = Arc::new(Probe(Arc::clone(&event), Mutex::new(Vec::new())));
        let waker = Waker::from(Arc::clone(&probe));
        let mut f = Readiness(Arc::clone(&event));
        assert!(
            Pin::new(&mut f)
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        signal(&event);
        assert_eq!(
            *probe.1.lock().unwrap(),
            [true],
            "take the waiter under the lock, wake after unlocking"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Only 2 bytes have arrived. A 3-byte read, then read_exact of 3 bytes.
        let mut arrived: &[u8] = &[1, 2];
        let mut buf = [0u8; 3];
        let n = arrived.read(&mut buf).unwrap();
        let mut again: &[u8] = &[1, 2];
        let kind = again.read_exact(&mut buf).unwrap_err().kind();
        let p1: (usize, ErrorKind) = guess();
        assert_eq!(p1, (n, kind), "P1: readiness is not a whole message");

        // P2. Reading from a source with no bytes left: error or value?
        let mut empty: &[u8] = &[];
        let p2: usize = guess();
        assert_eq!(p2, empty.read(&mut buf).unwrap(), "P2: how EOF is reported");
    }
}
