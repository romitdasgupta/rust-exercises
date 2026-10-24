//! DRILL 12-09 · Indexing a map with a missing key
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch12`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what can Index return when there is nothing to point at?

use std::collections::HashMap;

fn main() {
    let ports = HashMap::from([("http", 80), ("https", 443)]);
    let ssh = ports["ssh"];
    assert_eq!(ssh, 22); // KEEP
}
