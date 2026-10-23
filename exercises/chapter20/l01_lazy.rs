//! LESSON 01 — Lazy futures and await
//! Run: cargo test --test chapter20 l01_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
//! 1. Does creating an async future increment a counter inside its body?
//! 2. Does awaiting twice mean two futures run concurrently?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Trace future creation, first polling, and await sequencing.
//! COMPILER LAB (examples/ch20_scratch.rs): Store futures from two different async blocks in a Vec. Explain the type mismatch; repair with a common async function or boxed trait objects.

use std::{
    future::Future,
    sync::atomic::{AtomicUsize, Ordering},
};
/// A. Increment on execution, returning the previous count. Merely creating
/// this future must have no effect. Use Relaxed; this is a local observation.
async fn next_count(counter: &AtomicUsize) -> usize {
    todo!("A: implement the contract")
}
/// B. Await both computations sequentially, preserving their returned order.
async fn sequence<A, B>(left: A, right: B) -> (A::Output, B::Output)
where
    A: Future,
    B: Future,
{
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::{
        cell::RefCell,
        pin::Pin,
        task::{Context, Poll, Waker},
    };
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
    /// Returns Pending once (waking itself first), then Ready.
    struct YieldOnce(bool);
    impl Future for YieldOnce {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if self.0 {
                return Poll::Ready(());
            }
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
    /// Polls with a no-op waker until Ready; returns the output and the poll count.
    fn drive<F: Future>(future: F) -> (F::Output, usize) {
        let mut f = std::pin::pin!(future);
        let mut cx = Context::from_waker(Waker::noop());
        for polls in 1..=1000 {
            if let Poll::Ready(x) = f.as_mut().poll(&mut cx) {
                return (x, polls);
            }
        }
        panic!("still pending after 1000 polls")
    }
    #[test]
    fn a_creation_is_lazy() {
        let c = AtomicUsize::new(0);
        let f = next_count(&c);
        assert_eq!(c.load(Ordering::Relaxed), 0);
        assert_eq!(ready(f), 0);
        assert_eq!(c.load(Ordering::Relaxed), 1);
        drop(next_count(&c));
        assert_eq!(
            c.load(Ordering::Relaxed),
            1,
            "a dropped, unpolled future does nothing"
        );
    }
    #[test]
    fn b_await_runs_in_sequence() {
        let c = AtomicUsize::new(5);
        assert_eq!(ready(sequence(next_count(&c), next_count(&c))), (5, 6));
        assert_eq!(c.load(Ordering::Relaxed), 7);
    }
    #[test]
    fn b_right_starts_after_left_finishes() {
        let log = RefCell::new(Vec::new());
        let left = async {
            log.borrow_mut().push("left start");
            YieldOnce(false).await;
            log.borrow_mut().push("left end");
            'L'
        };
        let right = async {
            log.borrow_mut().push("right start");
            'R'
        };
        let mut f = std::pin::pin!(sequence(left, right));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(f.as_mut().poll(&mut cx).is_pending());
        assert_eq!(
            *log.borrow(),
            ["left start"],
            "right must not start until left has finished"
        );
        assert_eq!(drive(f).0, ('L', 'R'));
        assert_eq!(*log.borrow(), ["left start", "left end", "right start"]);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. In what order do these entries reach the log?
        let log = RefCell::new(Vec::new());
        let f = async {
            log.borrow_mut().push("body");
        };
        log.borrow_mut().push("created");
        ready(f);
        let p1: Vec<&str> = guess();
        assert_eq!(p1, *log.borrow(), "P1: when an async body runs");

        // P2. Two futures that each return Pending once, awaited one after the other.
        // How many polls does the outer future need to complete?
        let (_, polls) = drive(async {
            YieldOnce(false).await;
            YieldOnce(false).await;
        });
        let p2: usize = guess();
        assert_eq!(p2, polls, "P2: sequential awaits");

        // P3. `x` is changed after the future is created but before it runs.
        // What does the future return, and what is `x` afterwards?
        let mut x = 1;
        let f = async move { x * 10 };
        x = 5;
        let p3: (i32, i32) = guess();
        assert_eq!(p3, (ready(f), x), "P3: when async move captures");
    }
}
