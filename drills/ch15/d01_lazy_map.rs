//! DRILL 15-01 · An adapter nobody consumes
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch15`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: when does the closure passed to `map` run?

fn main() {
    let readings = [3, 5, 8];
    let mut logged = Vec::new();
    readings.iter().map(|r| logged.push(r * 10));
    assert_eq!(logged, [30, 50, 80]); // KEEP
}
