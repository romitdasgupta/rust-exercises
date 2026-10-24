//! DRILL 14-03 · The borrow ends at the last call
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch14`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: where is the closure last used?

fn main() {
    let mut count = 0;
    let mut bump = || count += 1;
    bump();
    bump();
    println!("count: {count}");
    assert_eq!(count, 2); // KEEP
}
