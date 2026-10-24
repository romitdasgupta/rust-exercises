//! DRILL 22-08 · Locals declared by a macro
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch22`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who wrote the name `x`: the caller or the macro?

macro_rules! make_x {
    () => {
        let x = 1;
    };
}

fn main() {
    make_x!();
    assert_eq!(x, 1); // KEEP
}
