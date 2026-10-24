//! DRILL 23-10 · Moving out through a raw pointer
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch23`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how many owners of the String's buffer would there be?

fn main() {
    let name = String::from("ferris");
    let p: *const String = &name;
    let copy: String = unsafe { *p };
    assert_eq!(copy, name); // KEEP
}
