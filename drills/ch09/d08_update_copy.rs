//! DRILL 09-08 · Update syntax with Copy fields
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch09`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does `..base` do with a field that is Copy?

#[derive(Debug)]
struct Style {
    width: u8,
    bold: bool,
}

fn main() {
    let base = Style { width: 1, bold: true };
    let wide = Style { width: 4, ..base };
    assert_eq!(base.width + wide.width, 5); // KEEP
    assert!(base.bold && wide.bold); // KEEP
}
