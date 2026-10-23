//! LESSON 03 — A completion cell
//! Run: cargo test --test chapter21 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
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
    /// A waker that records whether `lock` was free at the moment it was woken.
    struct Probe<T> {
        lock: Arc<Mutex<T>>,
        seen: Mutex<Vec<bool>>,
    }
    impl<T: Send> Wake for Probe<T> {
        fn wake(self: Arc<Self>) {
            self.wake_by_ref();
        }
        fn wake_by_ref(self: &Arc<Self>) {
            let free = self.lock.try_lock().is_ok();
            self.seen.lock().unwrap().push(free);
        }
    }
    #[test]
    fn a_b_c_complete_before_poll_is_ready() {
        let (tx, mut rx) = completion();
        tx.finish(String::from("ok"));
        let mut cx = Context::from_waker(Waker::noop());
        assert_eq!(
            Pin::new(&mut rx).poll(&mut cx),
            Poll::Ready(String::from("ok"))
        );
    }
    #[test]
    fn a_b_c_latest_registered_waker_is_used() {
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
    #[test]
    fn a_b_dropped_producer_leaves_the_receiver_pending() {
        let (tx, mut rx) = completion::<u8>();
        drop(tx);
        let (c, w) = counted();
        assert!(
            Pin::new(&mut rx)
                .poll(&mut Context::from_waker(&w))
                .is_pending()
        );
        assert_eq!(c.0.load(Ordering::Relaxed), 0);
    }
    #[test]
    fn c_wake_happens_after_unlocking() {
        let (tx, mut rx) = completion::<u32>();
        let probe = Arc::new(Probe {
            lock: Arc::clone(&rx.0),
            seen: Mutex::new(Vec::new()),
        });
        let waker = Waker::from(Arc::clone(&probe));
        assert!(
            Pin::new(&mut rx)
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        tx.finish(5);
        assert_eq!(
            *probe.seen.lock().unwrap(),
            [true],
            "wake exactly once, after the state lock is released"
        );
        let mut cx = Context::from_waker(Waker::noop());
        assert_eq!(Pin::new(&mut rx).poll(&mut cx), Poll::Ready(5));
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. The waker checks whether `state` is free. It is woken once while the
        // lock is held, and once after the guard is dropped.
        let state = Arc::new(Mutex::new(0u8));
        let probe = Arc::new(Probe {
            lock: Arc::clone(&state),
            seen: Mutex::new(Vec::new()),
        });
        let waker = Waker::from(Arc::clone(&probe));
        {
            let _guard = state.lock().unwrap();
            waker.wake_by_ref();
        }
        waker.wake_by_ref();
        let p1: Vec<bool> = guess();
        assert_eq!(p1, *probe.seen.lock().unwrap(), "P1: waking under a lock");

        // P2. A slot holds a clone of waker `wa`. Strong count of `a` before and
        // after the slot's waker is replaced by a clone of `wb`.
        let (a, wa) = counted();
        let (_b, wb) = counted();
        let mut slot = Some(wa.clone());
        let before = Arc::strong_count(&a);
        slot.replace(wb.clone());
        let after = Arc::strong_count(&a);
        let p2: (usize, usize) = guess();
        assert_eq!(
            p2,
            (before, after),
            "P2: replacing a stored waker drops the old one"
        );
        drop(slot);
    }
}
