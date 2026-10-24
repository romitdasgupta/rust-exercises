//! DRILL 15-07 · Growing a vector while iterating it
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch15`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does the loop hold while it runs?

fn main() {
    let mut queue = vec![1, 2, 3];
    for job in &queue {
        if *job == 2 {
            queue.push(20);
        }
    }
    assert_eq!(queue, [1, 2, 3, 20]); // KEEP
}
