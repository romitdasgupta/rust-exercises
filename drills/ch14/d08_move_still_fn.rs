//! DRILL 14-08 · A move closure called twice
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch14`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does `move` decide how often a closure can be called?

fn main() {
    let greeting = String::from("hello");
    let shout = move |name: &str| format!("{greeting}, {name}!");
    let a = shout("ada");
    let b = shout("grace");
    assert_eq!(a.len() + b.len(), 11 + 13); // KEEP
}
