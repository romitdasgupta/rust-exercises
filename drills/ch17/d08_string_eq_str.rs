//! DRILL 17-08 · Comparing String with &str
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch17`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: is there a PartialEq impl between String and &str?

fn main() {
    let owned = String::from("ok");
    let borrowed: &str = "ok";
    assert!(owned == borrowed); // KEEP
    assert!(borrowed == owned); // KEEP
}
