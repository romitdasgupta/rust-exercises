//! DRILL 18-05 · Printing a Path
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch18`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can every path be shown as text without a decision about invalid Unicode?

use std::path::Path;

fn main() {
    let path = Path::new("logs/today.txt");
    let shown = format!("{}", path);
    assert_eq!(shown, "logs/today.txt"); // KEEP
}
