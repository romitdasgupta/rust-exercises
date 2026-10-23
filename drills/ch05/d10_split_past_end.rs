//! DRILL 05-10 · Splitting past the end
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch05`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: is the split point checked by the compiler or at run time?

fn main() {
    let mut values = [1, 2, 3];
    let mid = values.len() + 1;
    let (left, right) = values.split_at_mut(mid);
    left[0] = 9;
    assert_eq!(right.len(), 0); // KEEP
    assert_eq!(values[0], 9); // KEEP
}
