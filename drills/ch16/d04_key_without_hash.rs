//! DRILL 16-04 · A key type without Hash
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch16`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which bounds does HashMap put on its key type?

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Cell(i32, i32);

fn main() {
    let mut board = HashMap::new();
    board.insert(Cell(0, 0), 'x');
    assert_eq!(board.len(), 1); // KEEP
}
