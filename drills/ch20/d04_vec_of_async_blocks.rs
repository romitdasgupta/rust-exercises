//! DRILL 20-04 · Two async blocks in one Vec
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch20`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how many future types are there here?

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

/// Polls a future once with a waker that does nothing. Enough for futures
/// that never return Pending.
fn run<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    match future
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("this tiny executor cannot wait"),
    }
}

fn main() {
    let jobs = vec![async { 1 }, async { 2 }];
    let total: i32 = jobs.into_iter().map(run).sum();
    assert_eq!(total, 3); // KEEP
}
