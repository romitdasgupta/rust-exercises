//! DRILL 15-10 · A computed step of zero
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch15`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what is `values.len() / 4` for three values?

fn main() {
    let values = [10, 20, 30];
    let step = values.len() / 4;
    let sampled: Vec<i32> = values.iter().copied().step_by(step).collect();
    assert_eq!(sampled[0], 10); // KEEP
}
