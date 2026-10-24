//! DRILL 20-05 · Rc held across an await
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch20`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which values does the future store while it is suspended?

use std::future::{Future, ready};
use std::rc::Rc;

fn spawn_check<F: Future + Send + 'static>(future: F) -> F {
    future
}

async fn report() -> usize {
    let shared = Rc::new(vec![1, 2, 3]);
    ready(()).await;
    shared.len()
}

fn main() {
    let task = spawn_check(report()); // KEEP
    drop(task);
}
