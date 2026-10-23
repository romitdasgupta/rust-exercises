//! DRILL 11-07 · Associated types are part of the impl
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch11`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does the caller of next() learn the item type from?

struct Countdown(u32);

impl Iterator for Countdown {
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            return None;
        }
        self.0 -= 1;
        Some(self.0)
    }
}

fn main() {
    assert_eq!(Countdown(3).collect::<Vec<_>>(), [2, 1, 0]); // KEEP
}
