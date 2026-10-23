//! DRILL 11-02 · Trait methods need the trait in scope
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch11`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: where is write_all defined, and is that name visible here?

fn main() {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.write_all(b"ok").unwrap(); // KEEP
    assert_eq!(buffer, b"ok"); // KEEP
}
