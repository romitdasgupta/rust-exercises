//! DRILL 20-08 · A trait object with an async method
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch20`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what type does `dyn Store`'s `get` return?

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

trait Store {
    async fn get(&self, key: &str) -> Option<u32>;
}

struct Memory;

impl Store for Memory {
    async fn get(&self, key: &str) -> Option<u32> {
        (key == "a").then_some(1)
    }
}

fn main() {
    let store: Box<dyn Store> = Box::new(Memory);
    assert_eq!(run(store.get("a")), Some(1)); // KEEP
}
