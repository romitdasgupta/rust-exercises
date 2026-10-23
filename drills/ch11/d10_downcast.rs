//! DRILL 11-10 · Downcasting a trait object
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch11`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what type does the literal 5 have when nothing else constrains it?

use std::any::Any;

fn main() {
    let boxed: Box<dyn Any> = Box::new(5);
    let n = *boxed.downcast::<i64>().unwrap();
    assert_eq!(n, 5); // KEEP
}
