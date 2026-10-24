//! DRILL 10-09 · Array patterns have a length
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch10`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how many elements does the type of `rgb` have?

fn main() {
    let rgb = [255u8, 128, 0];
    let [red, green] = rgb;
    assert_eq!(u32::from(red) + u32::from(green), 383); // KEEP
}
