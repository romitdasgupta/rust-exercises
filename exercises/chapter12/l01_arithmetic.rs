//! LESSON 01 — Unary, binary, and assignment operators
//! Run: cargo test --test chapter12 l01_
//!
//! PREDICT before executing: Does a+b necessarily return the same type as a? Does a+=b have
//! an Output associated type?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why are assignment traits separate? What are the move semantics of
//! each receiver in these implementations?
//!
//! COMPILER / MANUAL LAB (use examples/ch12_scratch.rs):
//! Implement a bit-mask newtype with BitAnd, BitOr, Not, and BitOrAssign in scratch; test 0
//! and all bits set. Try overloading &&, as, or = and identify why no corresponding trait
//! exists.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::ops::{Add, AddAssign, Mul, Neg};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Delta {
    x: i32,
    y: i32,
}
impl Add for Delta {
    type Output = Self;
    /// A. Componentwise addition. Inputs are bounded so arithmetic fits.
    fn add(self, rhs: Self) -> Self {
        todo!("Task 1: implement the documented contract")
    }
}
impl Neg for Delta {
    type Output = Self;
    /// B. Negate both coordinates.
    fn neg(self) -> Self {
        todo!("Task 2: implement the documented contract")
    }
}
impl AddAssign for Delta {
    /// C. Mutate both coordinates with componentwise addition.
    fn add_assign(&mut self, rhs: Self) {
        todo!("Task 3: implement the documented contract")
    }
}
impl Mul<i32> for Delta {
    type Output = Self;
    /// D. Scale both coordinates; the RHS type differs from Self.
    fn mul(self, rhs: i32) -> Self {
        todo!("Task 4: implement the documented contract")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_addition_is_componentwise() {
        assert_eq!(
            Delta { x: 2, y: -4 } + Delta { x: -2, y: 7 },
            Delta { x: 0, y: 3 }
        );
    }
    #[test]
    fn b_unary_negation_handles_signs() {
        assert_eq!(-Delta { x: -3, y: 0 }, Delta { x: 3, y: 0 });
    }
    #[test]
    fn c_assignment_changes_the_original() {
        let mut d = Delta { x: 4, y: 9 };
        d += Delta { x: 1, y: -9 };
        assert_eq!(d, Delta { x: 5, y: 0 });
    }
    #[test]
    fn d_rhs_can_be_another_type() {
        assert_eq!(Delta { x: 3, y: -2 } * 4, Delta { x: 12, y: -8 });
        assert_eq!(Delta { x: 1, y: 2 } * 0, Delta { x: 0, y: 0 });
    }
}
