//! DRILL 16-01 · Holding a reference across push
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch16`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: could this push move the buffer that `first` points into?

fn main() {
    let mut scores = Vec::with_capacity(10);
    scores.extend([3, 1, 4]);
    let first = &scores[0];
    scores.push(1);
    assert_eq!(*first, 3); // KEEP
    assert_eq!(scores.len(), 4); // KEEP
}
