//! DRILL 16-10 · Looking up a String key with &str
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch16`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does the Borrow trait let a map accept as a lookup key?

use std::collections::HashMap;

fn main() {
    let mut ports: HashMap<String, u16> = HashMap::new();
    ports.insert("http".to_string(), 80);
    assert_eq!(ports.get("http"), Some(&80)); // KEEP
    assert!(ports.contains_key("http")); // KEEP
}
