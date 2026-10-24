//! DRILL 10-03 · A lowercase name in a pattern
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch10`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does `expected` in the pattern compare, or bind?

fn check(code: u16) -> &'static str {
    let expected = 200;
    match code {
        expected => "ok",
        _ => "unexpected",
    }
}

fn main() {
    assert_eq!(check(200), "ok"); // KEEP
    assert_eq!(check(404), "unexpected"); // KEEP
}
