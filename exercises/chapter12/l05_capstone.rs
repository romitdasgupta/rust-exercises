//! LESSON 05 — Capstone: a small bit-set API
//! Run: cargo test --test chapter12 l05_
//!
//! PREDICT before running: Predict the results of union, intersection, and complement on
//! 8-bit masks. Does complement require knowing a collection length?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why does Eq remain consistent with derived Hash here? Why is indexing
//! a mask less natural than asking contains(bit)?
//!
//! COMPILER / MANUAL LAB (use examples/ch12_scratch.rs):
//! Add BitXor and BitAndAssign, test identities x^x=0 and x&all=x, then explain why Deref,
//! function-call syntax, and ? have different trait rules from ordinary arithmetic
//! operators.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::ops::{BitAnd, BitOr, BitOrAssign, Not};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Flags(u8);
impl BitOr for Flags {
    type Output = Self;
    /// A. Union: a bit is set when it is set in either operand.
    fn bitor(self, rhs: Self) -> Self {
        todo!("Task 1: implement the documented contract")
    }
}
impl BitAnd for Flags {
    type Output = Self;
    /// B. Intersection: a bit is set when it is set in both operands.
    fn bitand(self, rhs: Self) -> Self {
        todo!("Task 2: implement the documented contract")
    }
}
impl Not for Flags {
    type Output = Self;
    /// C. Complement: flip all 8 bits.
    fn not(self) -> Self {
        todo!("Task 3: implement the documented contract")
    }
}
impl BitOrAssign for Flags {
    /// D. Union in place: add rhs's bits to self.
    fn bitor_assign(&mut self, rhs: Self) {
        todo!("Task 4: implement the documented contract")
    }
}
impl Flags {
    /// E. Return false for bit indices >=8, otherwise test that bit.
    fn contains(self, bit: u32) -> bool {
        todo!("Task 5: implement the documented contract")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_union_combines_bits() {
        assert_eq!(Flags(0b0101) | Flags(0b1010), Flags(15));
        assert_eq!(Flags(5) | Flags(5), Flags(5));
    }
    #[test]
    fn b_intersection_keeps_shared_bits() {
        assert_eq!(Flags(0b1100) & Flags(0b1010), Flags(8));
        assert_eq!(Flags(255) & Flags(17), Flags(17));
    }
    #[test]
    fn c_complement_is_eight_bits_wide() {
        assert_eq!(!Flags(0), Flags(255));
        assert_eq!(!!Flags(19), Flags(19));
        assert_eq!(!Flags(0b1010_0101), Flags(0b0101_1010));
    }
    #[test]
    fn d_assignment_updates_the_receiver() {
        let mut f = Flags(1);
        f |= Flags(4);
        assert_eq!(f, Flags(5));
    }
    #[test]
    fn e_membership_checks_the_width_before_shifting() {
        assert!(Flags(129).contains(0));
        assert!(Flags(129).contains(7));
        assert!(!Flags(129).contains(1));
        assert!(!Flags(255).contains(8));
        assert!(!Flags(255).contains(u32::MAX));
    }
}
