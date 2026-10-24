//! LESSON 01 — Implement Future
//! Run: cargo test --test chapter21 l01_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
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
    use practice::guess;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
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
    fn a_b_output_is_moved_exactly_once() {
        assert_eq!(ready(one_shot(String::from("owned"))), "owned");
        assert_eq!(ready(one_shot(7)), 7);
    }
    #[test]
    fn a_b_pin_box_can_move_its_handle() {
        let first = one_shot(vec![1, 2]);
        let address = &*first as *const _;
        let moved = first;
        assert_eq!(&*moved as *const _, address);
        assert_eq!(ready(moved), [1, 2]);
    }
    #[test]
    fn a_output_is_taken_not_cloned() {
        let value = String::from("moved, not copied");
        let p = value.as_ptr();
        let mut f = Once(Some(value));
        let (_, w) = counted();
        let out = Pin::new(&mut f).poll(&mut Context::from_waker(&w));
        match out {
            Poll::Ready(s) => assert_eq!(s.as_ptr(), p),
            Poll::Pending => panic!("Once must be ready on the first poll"),
        }
        assert!(f.0.is_none(), "the value must be taken out of the future");
    }
    #[test]
    fn a_ready_needs_no_wake() {
        let (c, w) = counted();
        let mut f = Once(Some(1u8));
        assert_eq!(
            Pin::new(&mut f).poll(&mut Context::from_waker(&w)),
            Poll::Ready(1)
        );
        assert_eq!(
            c.0.load(Ordering::Relaxed),
            0,
            "Ready must not request another poll"
        );
    }
    #[test]
    #[should_panic(expected = "polled after completion")]
    fn a_second_poll_panics() {
        let mut f = Once(Some(3));
        let mut cx = Context::from_waker(std::task::Waker::noop());
        let _ = Pin::new(&mut f).poll(&mut cx);
        let _ = Pin::new(&mut f).poll(&mut cx);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. Sizes on a 64-bit target: Pin<Box<u64>> and Box<u64>.
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (size_of::<Pin<Box<u64>>>(), size_of::<Box<u64>>()),
            "P1: what Pin costs at run time"
        );

        // P2. std::future::ready is polled a second time after returning Ready.
        // Does that second poll panic?
        let mut f = std::future::ready(5);
        let mut cx = Context::from_waker(std::task::Waker::noop());
        let _ = Pin::new(&mut f).poll(&mut cx);
        let second = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = Pin::new(&mut f).poll(&mut cx);
        }));
        let p2: bool = guess();
        assert_eq!(p2, second.is_err(), "P2: polling a finished future");
    }
}
