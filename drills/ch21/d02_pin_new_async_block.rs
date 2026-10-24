//! DRILL 21-02 · Pin::new on an async block
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch21`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: may this future move after it starts?

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

fn main() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut job = async {
        let text = String::from("borrowed across an await");
        let view = &text;
        std::future::ready(()).await;
        view.len()
    };
    let result = Pin::new(&mut job).poll(&mut cx);
    assert_eq!(result, Poll::Ready(24)); // KEEP
}
