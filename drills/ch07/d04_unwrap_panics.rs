//! DRILL 07-04 · unwrap on user input
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch07`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does unwrap do with an Err?

fn main() {
    let input = "70000";
    let port: u16 = input.parse().unwrap();
    let port = port.max(1024); // KEEP
    assert!(port >= 1024); // KEEP
}
