//! DRILL 22-09 · Where #[macro_export] puts a macro
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch22`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which module does an exported macro live in?

mod shapes {
    #[macro_export]
    macro_rules! area {
        ($w:expr, $h:expr) => {
            $w * $h
        };
    }
}

fn main() {
    let a = shapes::area!(2, 3);
    assert_eq!(a, 6); // KEEP
}
