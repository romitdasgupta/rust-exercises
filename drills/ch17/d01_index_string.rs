//! DRILL 17-01 · Indexing a String
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch17`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what would `s[0]` return if a character can take up to four bytes?

fn main() {
    let word = String::from("héllo");
    let first = word[0];
    assert_eq!(first, 'h'); // KEEP
}
