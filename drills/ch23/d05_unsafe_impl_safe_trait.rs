//! DRILL 23-05 · unsafe impl of a safe trait
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch23`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: does Clone have any safety contract to promise?

#[derive(Debug, PartialEq)]
struct Meters(u32);

unsafe impl Clone for Meters {
    fn clone(&self) -> Self {
        Meters(self.0)
    }
}

fn main() {
    assert_eq!(Meters(3).clone(), Meters(3)); // KEEP
}
