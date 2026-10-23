//! DRILL 05-02 · Returning a reference to a local
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch05`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who owns `upper` after the function returns?

fn shout(text: &str) -> &str {
    let upper = text.to_uppercase();
    &upper
}

fn main() {
    assert_eq!(shout("hi"), "HI"); // KEEP
}
