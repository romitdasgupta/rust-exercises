//! REVIEW 01 — Spaced retrieval: Chapters 15, 19, and 21 inside Chapter 22
//! Run: cargo test --test chapter22 r01_
//!
//! Do this WITHOUT reopening Chapters 15, 19, or 21 or their hints. Struggling
//! to recall is what makes the knowledge stick; look things up only after a
//! real attempt, and note in NOTES.md anything you had to look up.
//!
//! Each task wraps an earlier idea (polling futures, scoped threads, lazy
//! iterator adapters) in a macro from this chapter.

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

/// A. (Ch21 futures + Ch22 repetition) Poll each future expression exactly
/// once, left to right, with a no-op waker, and return the results in order.
/// The futures may have different types but share one Output type. An empty
/// invocation gives an empty Vec. Do not spawn threads or allocate per future.
macro_rules! poll_each {
    ($($fut:expr),* $(,)?) => {{ todo!("A: build one Context, then pin and poll each future") }};
}

/// B. (Ch19 threads + Ch22 repetition) Run each expression on its own scoped
/// thread, so expressions may borrow local data, join every thread, and
/// return the results in argument order. At least one expression is required.
macro_rules! spawn_join {
    ($($task:expr),+ $(,)?) => {{ todo!("B: std::thread::scope, one spawn per expression") }};
}

/// C. (Ch15 iterators + Ch22 recursion) Chain one or more IntoIterator
/// expressions into ONE lazy iterator, in argument order. Build it by recursion
/// over the arguments; do not collect into a Vec or allocate.
/// (The `iter::empty()` after `todo!` only lets the starter compile; replace it.)
macro_rules! chain_all {
    ($only:expr $(,)?) => {{
        todo!("C: the base case");
        ::std::iter::empty()
    }};
    ($first:expr, $($rest:expr),+ $(,)?) => {{
        todo!("C: chain the first onto the recursive rest");
        ::std::iter::empty()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::cell::{Cell, RefCell};
    use std::collections::HashSet;
    use std::future::{pending, ready};
    use std::thread::{self, ThreadId};

    /// Counts its polls and is never ready.
    struct CountPolls<'a>(&'a Cell<u32>);
    impl Future for CountPolls<'_> {
        type Output = i32;
        fn poll(self: std::pin::Pin<&mut Self>, _: &mut Context<'_>) -> Poll<i32> {
            self.0.set(self.0.get() + 1);
            Poll::Pending
        }
    }

    #[test]
    fn a_each_future_is_polled_once_in_order() {
        let polls = Cell::new(0);
        let log = RefCell::new(Vec::<&str>::new());
        let results: Vec<Poll<i32>> =
            poll_each!(ready(1), pending::<i32>(), CountPolls(&polls), async {
                log.borrow_mut().push("async body");
                3
            },);
        assert_eq!(
            results,
            [Poll::Ready(1), Poll::Pending, Poll::Pending, Poll::Ready(3)]
        );
        assert_eq!(polls.get(), 1, "poll each future exactly once");
        assert_eq!(*log.borrow(), ["async body"]);
        let none: Vec<Poll<u8>> = poll_each!();
        assert!(none.is_empty());
    }

    #[test]
    fn a_polling_does_not_allocate_per_future() {
        let (results, allocations): (Vec<Poll<u8>>, usize) =
            count(|| poll_each!(ready(1), ready(2), ready(3), ready(4), ready(5)));
        assert_eq!(results.len(), 5);
        assert!(
            allocations <= 1,
            "only the result Vec may allocate; got {allocations}"
        );
    }

    #[test]
    fn b_each_expression_runs_on_its_own_thread() {
        let ids: Vec<ThreadId> = spawn_join!(
            thread::current().id(),
            thread::current().id(),
            thread::current().id(),
        );
        let distinct: HashSet<ThreadId> = ids.iter().copied().collect();
        assert_eq!(distinct.len(), 3, "one thread per expression");
        assert!(!ids.contains(&thread::current().id()));
    }

    #[test]
    fn b_results_keep_argument_order_and_may_borrow() {
        let words = [String::from("ünï"), String::from("code")];
        let lengths: Vec<usize> = spawn_join!(
            words[0].chars().count(),
            words[1].len(),
            words.iter().map(String::len).sum::<usize>(),
        );
        assert_eq!(lengths, [3, 4, 9]);
        assert_eq!(words.len(), 2, "the threads only borrowed `words`");
    }

    #[test]
    fn c_chain_keeps_order_across_iterable_kinds() {
        let v: Vec<i32> = chain_all!(vec![1, 2], [3], 4..6, Some(6), None::<i32>).collect();
        assert_eq!(v, [1, 2, 3, 4, 5, 6]);
        let single: Vec<char> = chain_all!("héllo".chars()).collect();
        assert_eq!(single, ['h', 'é', 'l', 'l', 'o']);
    }

    #[test]
    fn c_chain_is_lazy_and_does_not_allocate() {
        let log = RefCell::new(Vec::new());
        let note = |x: i32| {
            log.borrow_mut().push(x);
            x
        };
        let mut it = chain_all!([1, 2].into_iter().map(note), [3].into_iter().map(note));
        assert!(log.borrow().is_empty(), "nothing runs before iteration");
        assert_eq!(it.next(), Some(1));
        assert_eq!(*log.borrow(), [1]);

        fn total(it: impl Iterator<Item = i32>) -> i32 {
            it.sum()
        }
        let (sum, allocations) = count(|| total(chain_all!(1..4, [10, 20], Some(100))));
        assert_eq!(sum, 136);
        assert_eq!(allocations, 0, "chaining adapters never allocates");
    }

    /// P. Chapters 15–21 recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        let mut cx = Context::from_waker(Waker::noop());

        // P1. A future is created and dropped without being polled. Did its body run?
        let mut ran = false;
        let fut = async {
            ran = true;
        };
        drop(fut);
        let p1: bool = guess();
        assert_eq!(p1, ran, "P1: futures are lazy");

        // P2. Poll once: the body pushes 1, awaits a future that is never
        // ready, then would push 2. Result of the poll, and the log afterward?
        let mut steps = Vec::new();
        let poll = {
            let mut fut = pin!(async {
                steps.push(1);
                pending::<()>().await;
                steps.push(2);
            });
            fut.as_mut().poll(&mut cx).is_pending()
        };
        let p2: (bool, Vec<i32>) = guess(); // (is_pending, steps)
        assert_eq!(p2, (poll, steps), "P2: progress up to the first Pending");

        // P3. Three threads each send one value on a clone of `tx`; then we
        // drop our `tx`. How many values does `rx.iter()` yield before ending?
        let (tx, rx) = std::sync::mpsc::channel();
        for i in 0..3 {
            let tx = tx.clone();
            thread::spawn(move || tx.send(i).unwrap());
        }
        drop(tx);
        let p3: usize = guess();
        assert_eq!(p3, rx.iter().count(), "P3: when a channel's iterator ends");

        // P4. Uppercasing "ß": the text and its byte length.
        let upper = "ß".to_uppercase();
        let p4: (&str, usize) = guess();
        assert_eq!(p4, (upper.as_str(), upper.len()), "P4: case mapping");
    }
}
