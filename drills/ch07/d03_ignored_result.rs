//! DRILL 07-03 · An ignored Result
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch07`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: is an unused Result an error or only a warning?

fn main() {
    let mut attempts = 0;
    let mut save = || -> Result<(), String> {
        attempts += 1;
        Err("disk full".to_string())
    };
    save();
    assert_eq!(attempts, 1); // KEEP
}
