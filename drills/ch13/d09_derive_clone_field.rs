//! DRILL 13-09 · Deriving Clone needs Clone fields
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch13`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: can every field of Job be cloned?

struct Token(u64);

#[derive(Clone)]
struct Job {
    name: String,
    token: Token,
}

fn main() {
    let job = Job { name: "build".into(), token: Token(1) };
    let again = job.clone(); // KEEP
    assert_eq!(again.name, job.name); // KEEP
    assert_eq!(again.token.0, 1); // KEEP
}
