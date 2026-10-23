//! DRILL 09-05 · Calling a &mut self method
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch09`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what does `&mut self` require of the binding?

#[derive(Default)]
struct Counter {
    hits: u32,
}

impl Counter {
    fn bump(&mut self) {
        self.hits += 1;
    }
}

fn main() {
    let counter = Counter::default();
    counter.bump();
    assert_eq!(counter.hits, 1); // KEEP
}
