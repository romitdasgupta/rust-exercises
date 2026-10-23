//! DRILL 09-09 · Moving a field out of &self
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch09`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who owns `self.name` while the method runs?

struct Badge {
    name: String,
}

impl Badge {
    fn name(&self) -> String {
        self.name
    }
}

fn main() {
    let badge = Badge { name: String::from("ana") };
    assert_eq!(badge.name(), "ana"); // KEEP
}
