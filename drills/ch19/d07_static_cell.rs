//! DRILL 19-07 · A Cell in a static
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch19`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how many threads can see a static at once?

use std::cell::Cell;

static REQUESTS: Cell<u32> = Cell::new(0);

/// Counts one request and returns the new total.
fn record() -> u32 {
    REQUESTS.set(REQUESTS.get() + 1);
    REQUESTS.get()
}

fn main() {
    assert_eq!(record(), 1); // KEEP
    assert_eq!(record(), 2); // KEEP
}
