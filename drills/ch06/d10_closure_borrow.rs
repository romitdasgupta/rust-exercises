//! DRILL 06-10 · A closure that borrows mutably
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch06`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: while inc exists, who else may read count?

fn main() {
    let mut count = 0;
    let mut inc = || count += 1;
    inc();
    println!("count so far: {count}");
    inc();
    assert_eq!(count, 2); // KEEP
}
