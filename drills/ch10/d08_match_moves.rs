//! DRILL 10-08 · A match moves its payload
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch10`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: after the match, what does `message` still own?

fn main() {
    let message = Some(String::from("deploy"));
    let length = match message {
        Some(text) => text.len(),
        None => 0,
    };
    assert_eq!(length, 6); // KEEP
    assert_eq!(message.as_deref(), Some("deploy")); // KEEP
}
