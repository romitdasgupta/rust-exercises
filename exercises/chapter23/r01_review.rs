//! REVIEW 01 — Spaced retrieval: Chapters 17, 21, and 22 inside Chapter 23
//! Run: cargo test --test chapter23 r01_
//!
//! Do this WITHOUT reopening Chapters 17, 21, or 22 or their hints. Struggling
//! to recall is what makes the knowledge stick; look things up only after a
//! real attempt, and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (macro hygiene and evaluation, wakers,
//! UTF-8 boundaries) with this chapter's unsafe contracts. Write a SAFETY
//! comment for every unsafe block.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{RawWaker, RawWakerVTable, Waker};

/// A. (Ch22 macros + Ch23 unchecked access) Expand to an `Option<&T>`:
/// evaluate the slice expression once, then the index expression once, check
/// the bounds, and only then use `get_unchecked` inside an unsafe block.
macro_rules! checked_get {
    ($slice:expr, $index:expr) => {{ todo!("A: bind both once, check, then get_unchecked") }};
}

/// B. (Ch21 wakers + Ch23 raw pointers) Build a Waker by hand, without the
/// `std::task::Wake` trait. Its data pointer comes from `Arc::into_raw`, and
/// each live Waker owns one strong count of that Arc. `wake` and `wake_by_ref`
/// add 1 to the counter; clone adds an owner; drop (supplied) releases one.
fn counting_waker(wakes: Arc<AtomicUsize>) -> Waker {
    todo!("B: Arc::into_raw, then RawWaker::new with VTABLE")
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, wake_raw, wake_by_ref_raw, drop_raw);

// Contract for every vtable function: `data` came from
// `Arc::<AtomicUsize>::into_raw`, and the calling Waker owns one strong count.

unsafe fn clone_raw(data: *const ()) -> RawWaker {
    todo!("B: add a strong count for the new Waker")
}
unsafe fn wake_raw(data: *const ()) {
    todo!("B: count the wake, then release this Waker's strong count")
}
unsafe fn wake_by_ref_raw(data: *const ()) {
    todo!("B: count the wake without touching the strong count")
}
// Supplied so an unfinished task cannot panic again while a test unwinds.
unsafe fn drop_raw(data: *const ()) {
    // SAFETY: per the contract above, this Waker owns one strong count.
    drop(unsafe { Arc::from_raw(data.cast::<AtomicUsize>()) });
}

/// C. (Ch17 UTF-8 + Ch23 unchecked slicing) Split `text` at the FIRST `sep`
/// byte, which is removed. Return None if `sep` is not ASCII or does not
/// occur. Build both halves with `str::get_unchecked`, justified by the fact
/// that an ASCII byte is always a char boundary. Borrow; do not allocate.
fn split_once_ascii(text: &str, sep: u8) -> Option<(&str, &str)> {
    todo!("C: check sep.is_ascii(), find it in the bytes, then slice unchecked")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::cell::RefCell;
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll};

    #[test]
    fn a_checked_get_bounds_and_identity() {
        let values = [10, 20, 30];
        let middle: Option<&i32> = checked_get!(&values, 1);
        assert!(std::ptr::eq(middle.unwrap(), &values[1]));
        let past: Option<&i32> = checked_get!(&values, 3);
        assert_eq!(past, None);
        let huge: Option<&i32> = checked_get!(&values, usize::MAX);
        assert_eq!(huge, None);
        let empty: Option<&u8> = checked_get!(&[][..], 0);
        assert_eq!(empty, None);
    }

    #[test]
    fn a_checked_get_evaluates_each_argument_once_in_order() {
        let log = RefCell::new(Vec::<&str>::new());
        let values = [String::from("x"), String::from("y")];
        let got: Option<&String> = checked_get!(
            {
                log.borrow_mut().push("slice");
                &values[..]
            },
            {
                log.borrow_mut().push("index");
                1
            }
        );
        assert_eq!(got.map(String::as_str), Some("y"));
        assert_eq!(*log.borrow(), ["slice", "index"]);
    }

    #[test]
    fn b_wakes_are_counted_and_owners_balanced() {
        let wakes = Arc::new(AtomicUsize::new(0));
        let waker = counting_waker(Arc::clone(&wakes));
        assert_eq!(Arc::strong_count(&wakes), 2, "the Waker owns one count");
        waker.wake_by_ref();
        waker.wake_by_ref();
        assert_eq!(wakes.load(Ordering::SeqCst), 2);
        let copy = waker.clone();
        assert_eq!(Arc::strong_count(&wakes), 3, "clone adds an owner");
        drop(copy);
        assert_eq!(Arc::strong_count(&wakes), 2, "drop releases one");
        waker.wake();
        assert_eq!(wakes.load(Ordering::SeqCst), 3);
        assert_eq!(Arc::strong_count(&wakes), 1, "wake consumes the Waker");
    }

    #[test]
    fn b_a_future_can_wake_through_it() {
        /// Ready on the second poll; wakes itself after the first.
        struct YieldOnce(bool);
        impl Future for YieldOnce {
            type Output = ();
            fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
                if self.0 {
                    return Poll::Ready(());
                }
                self.0 = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
        let wakes = Arc::new(AtomicUsize::new(0));
        let waker = counting_waker(Arc::clone(&wakes));
        let mut cx = Context::from_waker(&waker);
        let mut fut = pin!(YieldOnce(false));
        assert!(fut.as_mut().poll(&mut cx).is_pending());
        assert_eq!(wakes.load(Ordering::SeqCst), 1);
        assert!(fut.as_mut().poll(&mut cx).is_ready());
        drop(waker);
        assert_eq!(Arc::strong_count(&wakes), 1);
    }

    #[test]
    fn c_split_respects_char_boundaries() {
        assert_eq!(split_once_ascii("naïve=été", b'='), Some(("naïve", "été")));
        assert_eq!(split_once_ascii("a=b=c", b'='), Some(("a", "b=c")));
        assert_eq!(split_once_ascii("=x", b'='), Some(("", "x")));
        assert_eq!(split_once_ascii("x=", b'='), Some(("x", "")));
        assert_eq!(split_once_ascii("no separator", b'='), None);
        assert_eq!(split_once_ascii("", b'='), None);
    }

    #[test]
    fn c_non_ascii_separators_are_rejected() {
        // 0xA9 is the second byte of "é": splitting there would break UTF-8.
        assert!(split_once_ascii("é", 0xA9).is_none());
        assert!(split_once_ascii("aé b", 0xC3).is_none());
    }

    #[test]
    fn c_split_borrows_without_allocating() {
        let text = String::from("key=value");
        let (parts, allocations) = count(|| split_once_ascii(&text, b'='));
        let (key, value) = parts.unwrap();
        assert_eq!(allocations, 0);
        assert_eq!(key.as_ptr(), text.as_ptr());
        assert_eq!(value.as_ptr(), text[4..].as_ptr());
    }

    /// P. Chapters 17–22 recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1. Expression fragments keep their grouping.
        macro_rules! neg {
            ($x:expr) => {
                -$x
            };
        }
        let p1: i32 = guess();
        assert_eq!(p1, neg!(2 - 5), "P1: macro substitution");

        // P2. Polls needed until this async block is ready, and its output.
        struct YieldOnce(bool);
        impl Future for YieldOnce {
            type Output = ();
            fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
                if self.0 {
                    return Poll::Ready(());
                }
                self.0 = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
        let mut cx = Context::from_waker(Waker::noop());
        let mut fut = pin!(async {
            YieldOnce(false).await;
            7
        });
        let mut polls = 0;
        let output = loop {
            polls += 1;
            if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
                break v;
            }
        };
        let p2: (i32, i32) = guess(); // (polls, output)
        assert_eq!(p2, (polls, output), "P2: polling to completion");

        // P3. Byte index of 'v' in "naïve".
        let p3: Option<usize> = guess();
        assert_eq!(p3, "naïve".find('v'), "P3: byte offsets");
    }
}
