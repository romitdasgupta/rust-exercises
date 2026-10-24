//! DRILL 24-09 · Null function pointers
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch24`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can a Rust function pointer ever be null?

type Callback = extern "C" fn();

extern "C" fn tick() {}

fn main() {
    let mut cb: Callback = std::ptr::null();
    cb = tick;
    cb(); // KEEP
}
