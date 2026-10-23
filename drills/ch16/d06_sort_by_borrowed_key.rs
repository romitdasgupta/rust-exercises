//! DRILL 16-06 · Sorting by a borrowed key
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch16`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: how long must the key returned by the closure live?

struct Task {
    name: String,
    priority: u8,
}

fn main() {
    let mut tasks = vec![
        Task {
            name: "write".into(),
            priority: 2,
        },
        Task {
            name: "deploy".into(),
            priority: 1,
        },
    ];
    tasks.sort_by_key(|t| &t.name);
    assert_eq!(tasks[0].name, "deploy"); // KEEP
    assert_eq!(tasks[0].priority, 1); // KEEP
}
