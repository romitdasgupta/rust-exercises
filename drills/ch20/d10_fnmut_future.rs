//! DRILL 20-10 · A closure whose future borrows its state
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch20`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can the future outlive the `&mut` borrow of the closure call?

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

async fn twice<F, Fut>(mut callback: F)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = ()>,
{
    callback().await;
    callback().await;
}

fn main() {
    let mut calls = 0;
    run(twice(|| async {
        calls += 1;
    }));
    assert_eq!(calls, 2); // KEEP
}
