//! DRILL 23-08 · A mutable static
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch23`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what could another thread be doing to HITS at the same moment?

static mut HITS: u32 = 0;

fn hit() {
    HITS += 1;
}

fn hits() -> u32 {
    unsafe { HITS }
}

fn main() {
    hit(); // KEEP
    hit(); // KEEP
    assert_eq!(hits(), 2); // KEEP
}
