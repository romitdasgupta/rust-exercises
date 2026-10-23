//! DRILL 04-08 · Removing past the end
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch04`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: is this a compile-time question or a run-time one?

fn main() {
    let mut queue = vec![String::from("job")];
    let taken = queue.remove(1);
    assert_eq!(taken, "job"); // KEEP
    assert!(queue.is_empty()); // KEEP
}
