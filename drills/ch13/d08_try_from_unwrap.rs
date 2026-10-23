//! DRILL 13-08 · TryFrom at run time
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch13`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does try_from do with a value that does not fit?

fn main() {
    let reading: u16 = "300".parse().unwrap();
    let byte = u8::try_from(reading).unwrap();
    assert_eq!(byte, u8::MAX); // KEEP
}
