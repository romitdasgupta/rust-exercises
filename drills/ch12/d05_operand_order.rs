//! DRILL 12-05 · Operand order matters
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch12`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: which type is Self, and which is Rhs, in 3 * v?

use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2(i32, i32);

impl Mul<i32> for Vec2 {
    type Output = Vec2;
    fn mul(self, k: i32) -> Vec2 {
        Vec2(self.0 * k, self.1 * k)
    }
}

fn main() {
    let v = Vec2(1, -2);
    assert_eq!(v * 3, Vec2(3, -6)); // KEEP
    assert_eq!(3 * v, Vec2(3, -6)); // KEEP
}
