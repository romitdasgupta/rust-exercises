//! DRILL 08-01 · Private by default
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch08`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who can call an item that has no pub?

mod billing {
    fn fee(items: u32) -> u32 {
        3 + 2 * items
    }
}

fn main() {
    assert_eq!(billing::fee(2), 7); // KEEP
}
