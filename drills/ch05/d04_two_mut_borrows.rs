//! DRILL 05-04 · Two mutable borrows into one vector
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch05`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can the compiler tell that index 0 and index 1 differ?

fn main() {
    let mut pair = vec![String::from("left"), String::from("right")];
    let a = &mut pair[0];
    let b = &mut pair[1];
    std::mem::swap(a, b);
    assert_eq!(pair, ["right", "left"]); // KEEP
}
