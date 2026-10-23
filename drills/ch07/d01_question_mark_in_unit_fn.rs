//! DRILL 07-01 · ? needs somewhere to return the error
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch07`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what would ? return from a function whose return type is ()?

fn print_double(text: &str) {
    let n: i32 = text.parse()?;
    println!("{}", n * 2);
}

fn main() {
    print_double("21"); // KEEP
}
