//! DRILL 15-09 · Resuming after by_ref
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch15`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does `take` consume the iterator it is called on?

fn main() {
    let mut lines = ["header", "a", "b", "c"].into_iter();
    let head: Vec<&str> = lines.by_ref().take(1).collect();
    let body: Vec<&str> = lines.collect();
    assert_eq!((head.len(), body.len()), (1, 3)); // KEEP
}
