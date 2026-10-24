//! DRILL 08-03 · A path that does not exist
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch08`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: where exactly is Circle defined?

mod shapes {
    pub mod round {
        pub struct Circle {
            pub radius: f64,
        }
    }
}

use shapes::Circle;

fn main() {
    let c = Circle { radius: 2.0 }; // KEEP
    assert_eq!(c.radius, 2.0); // KEEP
}
