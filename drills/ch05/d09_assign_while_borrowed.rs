//! DRILL 05-09 · Assigning to a borrowed value
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch05`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: while `r` is still used, may `limit` change?

fn main() {
    let mut limit = 10;
    let r = &limit;
    limit = 20;
    assert_eq!(*r, 10); // KEEP
    assert_eq!(limit, 20); // KEEP
}
