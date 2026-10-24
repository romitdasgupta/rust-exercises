//! DRILL 08-08 · Re-exporting a crate-private item
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch08`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can pub use make an item more visible than it was declared?

mod engine {
    pub(crate) fn start() -> &'static str {
        "started"
    }
}

pub use engine::start;

fn main() {
    assert_eq!(start(), "started"); // KEEP
}
