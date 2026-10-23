//! DRILL 11-01 · Trait objects need dyn-compatible traits
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch11`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which method's return type cannot be known behind a vtable?

trait Shape {
    fn area(&self) -> f64;
    fn scaled(&self, factor: f64) -> Self;
}

struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
    fn scaled(&self, factor: f64) -> Self {
        Square(self.0 * factor)
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Square(2.0))]; // KEEP
    assert_eq!(shapes[0].area(), 4.0); // KEEP
    assert_eq!(Square(1.0).scaled(3.0).area(), 9.0); // KEEP
}
