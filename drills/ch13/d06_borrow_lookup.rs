//! DRILL 13-06 · Looking up String keys with &str
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch13`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which trait lets a HashMap<String, _> be queried by &str?

use std::collections::HashMap;

fn main() {
    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert("ada".to_string(), 36);
    let key: &str = "ada";
    assert_eq!(ages.get(key), Some(&36)); // KEEP
    assert!(ages.contains_key("ada")); // KEEP
}
