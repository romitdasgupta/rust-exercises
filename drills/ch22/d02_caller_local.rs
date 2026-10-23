//! DRILL 22-02 · A macro cannot see the caller's locals
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch22`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: where are the names in a macro body looked up?

macro_rules! add_tax {
    ($price:expr) => {
        $price + tax
    };
}

fn main() {
    let tax = 5; // KEEP
    assert_eq!(add_tax!(100), 105); // KEEP
}
