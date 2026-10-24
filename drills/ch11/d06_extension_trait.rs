//! DRILL 11-06 · Extension traits on foreign types
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch11`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: whose trait is being implemented, and is that allowed?

trait Halves {
    fn halves(&self) -> (&Self, &Self);
}

impl Halves for str {
    fn halves(&self) -> (&str, &str) {
        let mut mid = self.len() / 2;
        while !self.is_char_boundary(mid) {
            mid -= 1;
        }
        self.split_at(mid)
    }
}

fn main() {
    assert_eq!("rust".halves(), ("ru", "st")); // KEEP
    assert_eq!("añb".halves(), ("a", "ñb")); // KEEP
}
