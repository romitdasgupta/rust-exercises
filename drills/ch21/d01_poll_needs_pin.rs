//! DRILL 21-01 · Polling without a pin
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch21`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what is the receiver type of `poll`?

use std::future::{Future, ready};
use std::task::{Context, Poll, Waker};

fn main() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut job = ready(5);
    let result = job.poll(&mut cx);
    assert_eq!(result, Poll::Ready(5)); // KEEP
}
