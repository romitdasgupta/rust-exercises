//! DRILL 11-04 · Generic code only gets what its bounds promise
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch11`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does the compiler know about an arbitrary T?

fn largest<T>(items: &[T]) -> &T {
    let mut best = &items[0];
    for item in items {
        if item > best {
            best = item;
        }
    }
    best
}

fn main() {
    assert_eq!(*largest(&[3, 9, 4]), 9); // KEEP
    assert_eq!(*largest(&["b", "a"]), "b"); // KEEP
}
