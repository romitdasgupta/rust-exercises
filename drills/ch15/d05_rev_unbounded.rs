//! DRILL 15-05 · Reversing an endless range
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch15`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: where would `rev` start?

fn main() {
    let mut last_three: Vec<u32> = Vec::new();
    for n in (1..).rev() {
        if last_three.len() == 3 {
            break;
        }
        last_three.push(n);
    }
    assert_eq!(last_three, [10, 9, 8]); // KEEP
}
