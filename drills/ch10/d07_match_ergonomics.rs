//! DRILL 10-07 · Mutating through a match on &mut
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch10`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what type does `n` have here?

enum Counter {
    Running(u32),
    Stopped,
}

fn tick(counter: &mut Counter) {
    match counter {
        Counter::Running(n) => *n += 1,
        Counter::Stopped => {}
    }
}

fn main() {
    let mut c = Counter::Running(1);
    tick(&mut c);
    assert!(matches!(c, Counter::Running(2))); // KEEP
}
