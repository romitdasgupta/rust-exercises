//! DRILL 08-10 · An environment variable read at compile time
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch08`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: when is option_env! evaluated, and what if the variable is unset?

fn main() {
    let mode = option_env!("CH08_DRILL_MODE_UNSET").expect("mode must be set");
    assert!(!mode.is_empty()); // KEEP
}
