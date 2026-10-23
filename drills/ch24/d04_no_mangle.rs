//! DRILL 24-04 · Exporting a symbol
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch24`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: why can an unmangled symbol name be unsafe?

#[no_mangle]
pub extern "C" fn ch24_answer() -> i32 {
    42
}

fn main() {
    assert_eq!(ch24_answer(), 42); // KEEP
}
