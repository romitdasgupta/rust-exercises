//! DRILL 22-05 · A trailing comma
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch22`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which token does the matcher see after the last element?

macro_rules! list {
    ($($x:expr),*) => {
        vec![$($x),*]
    };
}

fn main() {
    let v = list![
        1,
        2,
        3, // KEEP
    ];
    assert_eq!(v, [1, 2, 3]); // KEEP
}
