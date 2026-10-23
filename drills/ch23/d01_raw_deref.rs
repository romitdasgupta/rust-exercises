//! DRILL 23-01 · Dereferencing a raw pointer
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch23`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which raw-pointer operation is the unsafe one?

fn main() {
    let x = 42;
    let p = &x as *const i32;
    let y = *p;
    assert_eq!(y, 42); // KEEP
}
