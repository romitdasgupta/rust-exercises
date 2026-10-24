//! DRILL 05-03 · Holding a borrow across a push
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch05`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what can happen to a vector's buffer when it grows?

fn main() {
    let mut scores = vec![10, 20, 30];
    let first = &scores[0];
    scores.push(40);
    assert_eq!(*first, 10); // KEEP
    assert_eq!(scores.len(), 4); // KEEP
}
