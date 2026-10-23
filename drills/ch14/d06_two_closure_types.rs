//! DRILL 14-06 · Two closures, one return type
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch14`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: do two closures with the same signature have the same type?

fn step(double: bool, by: i32) -> impl Fn(i32) -> i32 {
    if double {
        move |x| x * by
    } else {
        move |x| x + by
    }
}

fn main() {
    assert_eq!(step(true, 2)(5) + step(false, 1)(5), 16); // KEEP
}
