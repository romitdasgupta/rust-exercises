//! LESSON 04 — A parker block_on
//! Run: cargo test --test chapter21 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    use std::{
        cell::{Cell, RefCell},
        sync::{
            atomic::{AtomicUsize, Ordering},
            mpsc,
        },
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
    fn a_b_executor_handles_ready_and_wake_before_park() {
        assert_eq!(block_on(async { 3 }), 3);
        assert_eq!(block_on(yield_once(String::from("resumed"))), "resumed");
    }
    #[test]
    fn a_wake_from_another_thread_resumes_the_task() {
        let (tx, rx) = mpsc::channel();
        let polls = Cell::new(0);
        let worker = RefCell::new(None);
        let future = std::future::poll_fn(|cx| {
            polls.set(polls.get() + 1);
            if let Ok(value) = rx.try_recv() {
                return Poll::Ready(value);
            }
            if worker.borrow().is_none() {
                let waker = cx.waker().clone();
                let tx = tx.clone();
                *worker.borrow_mut() = Some(thread::spawn(move || {
                    tx.send(7).unwrap();
                    waker.wake();
                }));
            }
            Poll::Pending
        });
        assert_eq!(block_on(future), 7);
        worker.into_inner().unwrap().join().unwrap();
        assert!(
            polls.get() <= 10,
            "{} polls: park until woken instead of spinning",
            polls.get()
        );
    }
    #[test]
    fn b_yielding_future_does_bounded_work() {
        let mut f = std::pin::pin!(yield_once(9));
        let (c, w) = counted();
        let mut cx = Context::from_waker(&w);
        assert!(f.as_mut().poll(&mut cx).is_pending());
        assert_eq!(
            c.0.load(Ordering::Relaxed),
            1,
            "Pending must arrange a wake"
        );
        assert_eq!(f.as_mut().poll(&mut cx), Poll::Ready(9));
        assert_eq!(c.0.load(Ordering::Relaxed), 1, "Ready needs no wake");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. unpark() is called BEFORE park(). Does this thread get past park()?
        thread::current().unpark();
        thread::park();
        let reached = true;
        let p1: bool = guess();
        assert_eq!(p1, reached, "P1: unpark stores a token");

        // P2. A future that returns Pending twice (waking itself each time), driven
        // by a simple loop. How many polls until Ready?
        let mut left = 2;
        let mut f = std::pin::pin!(std::future::poll_fn(|cx| {
            if left == 0 {
                return Poll::Ready(());
            }
            left -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }));
        let mut cx = Context::from_waker(Waker::noop());
        let mut polls = 1;
        while f.as_mut().poll(&mut cx).is_pending() {
            polls += 1;
        }
        let p2: usize = guess();
        assert_eq!(p2, polls, "P2: one poll per wake, plus the first");
    }
}
