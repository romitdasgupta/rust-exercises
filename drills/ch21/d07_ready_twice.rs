//! DRILL 21-07 · Polling Ready twice
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch21`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what may a future do when polled after completion?

use std::future::{Future, ready};
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

fn main() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut config = ready(String::from("loaded"));
    let first = Pin::new(&mut config).poll(&mut cx);
    assert_eq!(first, Poll::Ready(String::from("loaded"))); // KEEP
    let again = Pin::new(&mut config).poll(&mut cx);
    assert!(again.is_ready());
}
