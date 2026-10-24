//! DRILL 13-03 · Generic parameters are Sized by default
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch13`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does T mean if nobody writes ?Sized?

fn byte_len<T>(value: &T) -> usize {
    std::mem::size_of_val(value)
}

fn main() {
    assert_eq!(byte_len("héllo"), 6); // KEEP
    assert_eq!(byte_len(&7_u32), 4); // KEEP
}
