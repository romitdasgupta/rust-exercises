//! REVIEW 01 — Spaced retrieval: Chapters 13, 17–19 inside Chapter 20
//! Run: cargo test --test chapter20 r01_
//!
//! Do this WITHOUT reopening Chapters 13–19 or their hints. Struggling to recall
//! is what makes the knowledge stick; look things up only after a real attempt,
//! and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (threads, Unicode text, Drop guards) with
//! this chapter's futures.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
    thread,
};

/// A. (Ch19 spawn/join + Ch20 Send futures) Move each future to its own new
/// thread. That thread pins it and polls it with `Waker::noop()` until Ready,
/// calling `thread::yield_now()` after every Pending. Return the outputs in
/// input order. (A real executor would wait for a wake instead of spinning.)
fn run_on_threads<T: Send + 'static>(
    futures: Vec<Pin<Box<dyn Future<Output = T> + Send>>>,
) -> Vec<T> {
    todo!("A: one thread per future; join in order")
}

/// B. (Ch17 Unicode text + Ch20 borrowed futures) Return the whitespace-separated
/// word with the most Unicode scalars (the first one on ties), or None if there
/// are no words. The future borrows `text` and the result points into it: no
/// allocation.
async fn longest_word(text: &str) -> Option<&str> {
    todo!("B: compare chars().count(), keep the first maximum")
}

/// C. (Ch13 Drop + Ch20 cancellation) Push every item onto `log`, then await
/// `pause`, then commit. If the future is dropped before it commits, `log` must
/// return to its original length. Use a guard type with a `Drop` impl (define
/// it in this file) instead of relying on code after the await.
async fn append_all(log: &mut Vec<u32>, items: &[u32], pause: impl Future<Output = ()>) {
    todo!("C: a rollback guard that remembers the original length")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::{
        cell::RefCell,
        collections::HashSet,
        io::{Cursor, Read},
        sync::Mutex,
    };

    fn ready<F: Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = Context::from_waker(Waker::noop());
        match f.as_mut().poll(&mut cx) {
            Poll::Ready(x) => x,
            Poll::Pending => panic!("expected immediately ready future"),
        }
    }

    /// Returns Pending `n` times (waking itself each time), then Ready.
    fn yield_times(mut n: u32) -> impl Future<Output = ()> + Send {
        std::future::poll_fn(move |cx| {
            if n == 0 {
                return Poll::Ready(());
            }
            n -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        })
    }

    #[test]
    fn a_outputs_keep_input_order() {
        let futures: Vec<Pin<Box<dyn Future<Output = String> + Send>>> = vec![
            Box::pin(async { "ready".to_string() }),
            Box::pin(async {
                yield_times(3).await;
                "after three yields".to_string()
            }),
            Box::pin(std::future::ready(String::new())),
        ];
        assert_eq!(run_on_threads(futures), ["ready", "after three yields", ""]);
        assert!(run_on_threads::<u8>(Vec::new()).is_empty());
    }

    #[test]
    fn a_each_future_runs_on_its_own_thread() {
        let futures: Vec<Pin<Box<dyn Future<Output = thread::ThreadId> + Send>>> = (0..5)
            .map(|n| {
                Box::pin(async move {
                    yield_times(n).await;
                    thread::current().id()
                }) as Pin<Box<dyn Future<Output = _> + Send>>
            })
            .collect();
        let ids: HashSet<_> = run_on_threads(futures).into_iter().collect();
        assert_eq!(
            ids.len(),
            5,
            "each future must be polled on a different thread"
        );
        assert!(!ids.contains(&thread::current().id()));
    }

    #[test]
    fn b_longest_word_counts_scalars_and_borrows() {
        let text = String::from("ab ñññ 🦀🦀🦀 xyz\tlonger");
        assert_eq!(ready(longest_word(&text)), Some("longer"));
        let tie = String::from("ééé abcd wxyz");
        let found = ready(longest_word(&tie)).unwrap();
        assert_eq!(found, "abcd");
        assert_eq!(found.as_ptr(), tie[7..].as_ptr());
        assert_eq!(ready(longest_word("  \n ")), None);
        assert_eq!(ready(longest_word("")), None);
    }

    #[test]
    fn b_longest_word_does_not_allocate() {
        let text = String::from("one three fifteen");
        let (found, allocations) = count(|| ready(longest_word(&text)));
        assert_eq!(found, Some("fifteen"));
        assert_eq!(allocations, 0);
    }

    #[test]
    fn c_completed_append_commits() {
        let mut log = vec![1];
        ready(append_all(&mut log, &[2, 3], std::future::ready(())));
        assert_eq!(log, [1, 2, 3]);
        ready(append_all(&mut log, &[], std::future::ready(())));
        assert_eq!(log, [1, 2, 3]);
    }

    #[test]
    fn c_cancelled_append_rolls_back() {
        let mut log = vec![1];
        {
            let mut f = Box::pin(append_all(&mut log, &[2, 3], std::future::pending()));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(f.as_mut().poll(&mut cx).is_pending());
        }
        assert_eq!(log, [1], "dropping the future must undo the pushes");
        drop(append_all(&mut log, &[9], std::future::ready(())));
        assert_eq!(log, [1], "an unpolled future changes nothing");
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    /// P. Earlier-chapter recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1 (Ch19). Can this thread lock the mutex again while `_held` is alive?
        let m = Mutex::new(0);
        let _held = m.lock().unwrap();
        let p1: bool = guess();
        assert_eq!(p1, m.try_lock().is_ok(), "P1: try_lock while locked");

        // P2 (Ch18). Two reads into a 3-byte buffer from a 4-byte cursor.
        let mut cursor = Cursor::new(vec![1u8, 2, 3, 4]);
        let mut buf = [0u8; 3];
        let first = cursor.read(&mut buf).unwrap();
        let second = cursor.read(&mut buf).unwrap();
        let p2: (usize, usize) = guess();
        assert_eq!(p2, (first, second), "P2: read returns what is available");

        // P3 (Ch17). The byte index and value of the last char.
        let p3: Option<(usize, char)> = guess();
        assert_eq!(p3, "crab 🦀".char_indices().last(), "P3: char_indices");

        // P4 (Ch13). In what order are a struct's fields dropped?
        struct Pair<'a> {
            first: Noisy<'a>,
            second: Noisy<'a>,
        }
        let log = RefCell::new(Vec::new());
        drop(Pair {
            first: Noisy("first", &log),
            second: Noisy("second", &log),
        });
        let p4: Vec<&str> = guess();
        assert_eq!(p4, *log.borrow(), "P4: field drop order");
    }
}
