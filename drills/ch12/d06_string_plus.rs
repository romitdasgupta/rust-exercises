//! DRILL 12-06 · String + &String
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch12`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does + need on the right, and can a &String become one?

fn main() {
    let host = String::from("example.com");
    let url = String::from("https://") + &host + "/";
    assert_eq!(url, "https://example.com/"); // KEEP
    assert_eq!(host, "example.com"); // KEEP
}
