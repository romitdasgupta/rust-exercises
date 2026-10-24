//! DRILL 14-10 · A nested fn is not a closure
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch14`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what environment can an inner `fn` item see?

fn main() {
    let rate = 3;
    fn scale(x: i32) -> i32 {
        x * rate
    }
    assert_eq!([1, 2].map(scale), [3, 6]); // KEEP
}
