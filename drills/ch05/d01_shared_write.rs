//! DRILL 05-01 · Writing through a shared reference
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch05`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does a & reference promise the owner?

fn add_suffix(text: &String) {
    text.push_str("!");
}

fn main() {
    let mut greeting = String::from("hi");
    add_suffix(&mut greeting);
    assert_eq!(greeting, "hi!"); // KEEP
}
