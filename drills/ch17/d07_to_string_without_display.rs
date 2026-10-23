//! DRILL 17-07 · to_string without Display
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch17`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: where does `to_string` come from?

struct Celsius(f64);

fn main() {
    let t = Celsius(21.5);
    assert_eq!(t.to_string(), "21.5°C"); // KEEP
}
