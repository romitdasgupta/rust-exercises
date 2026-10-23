//! DRILL 23-09 · Checked layouts
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch23`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which alignments can a real type have?

use std::alloc::Layout;

fn main() {
    let layout = Layout::from_size_align(12, 3).unwrap();
    assert_eq!(layout.size(), 12); // KEEP
}
