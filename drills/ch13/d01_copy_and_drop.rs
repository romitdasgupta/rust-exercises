//! DRILL 13-01 · Copy and Drop cannot mix
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch13`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: if a value is copied bit for bit, how many times would its destructor run?

#[derive(Clone, Copy)]
struct Handle(u32);

impl Drop for Handle {
    fn drop(&mut self) {
        println!("closing {}", self.0);
    }
}

fn main() {
    let a = Handle(7);
    let b = a.clone(); // KEEP
    assert_eq!(a.0 + b.0, 14); // KEEP
}
