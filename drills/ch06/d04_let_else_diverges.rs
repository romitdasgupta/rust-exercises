//! DRILL 06-04 · let else must leave
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch06`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what is the else block of a let else allowed to evaluate to?

fn port(text: &str) -> u16 {
    let Ok(n) = text.parse::<u16>() else {
        0
    };
    n
}

fn main() {
    assert_eq!(port("8080"), 8080); // KEEP
    assert_eq!(port("nope"), 0); // KEEP
}
