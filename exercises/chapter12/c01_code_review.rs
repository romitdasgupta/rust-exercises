//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter12 c01_
//! Then: cargo clippy --test chapter12
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and some behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every broken trait contract, and every unnecessary
//!    allocation.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. Keep the types and trait signatures; change only the bodies.
//! 4. Run clippy and resolve its warnings for this file.

use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign};

/// A tag compared without regard to ASCII case: "Rust" == "rUST", but "É" and
/// "é" differ. Tags are used as HashSet and HashMap keys.
#[derive(Debug, Clone)]
struct Tag(String);

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_ascii_lowercase() == other.0.to_ascii_lowercase()
    }
}

impl Eq for Tag {}

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

/// Polynomial coefficients, lowest power first: `Poly(vec![1, 2])` is 1 + 2x.
#[derive(Debug, Clone, PartialEq)]
struct Poly(Vec<i64>);

impl Add for Poly {
    type Output = Poly;
    /// Coefficient-wise sum; the result is as long as the longer operand.
    /// Both operands arrive by value, so reuse a buffer instead of allocating.
    fn add(self, rhs: Poly) -> Poly {
        let mut out = Vec::new();
        for (a, b) in self.0.iter().zip(rhs.0.iter()) {
            out.push(a + b);
        }
        Poly(out)
    }
}

impl AddAssign<&Poly> for Poly {
    /// Same sum as `Add`, written into `self`.
    fn add_assign(&mut self, rhs: &Poly) {
        *self = self.clone() + rhs.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    use std::hash::DefaultHasher;
    use std::hint::black_box;

    fn tag(s: &str) -> Tag {
        Tag(s.to_string())
    }

    fn hash_of(t: &Tag) -> u64 {
        let mut h = DefaultHasher::new();
        t.hash(&mut h);
        h.finish()
    }

    #[test]
    fn a_tags_ignore_ascii_case_only() {
        assert_eq!(tag("Rust"), tag("rUST"));
        assert_ne!(tag("Rust"), tag("Rest"));
        assert_ne!(tag("É"), tag("é"));
        assert_ne!(tag("ab"), tag("abc"));
    }

    #[test]
    fn a_equal_tags_hash_equally() {
        assert_eq!(hash_of(&tag("Rust")), hash_of(&tag("rUST")));
        assert_eq!(hash_of(&tag("")), hash_of(&tag("")));
    }

    #[test]
    fn a_comparing_tags_does_not_allocate() {
        let (a, b) = (tag("Serde-JSON"), tag("serde-json"));
        let (equal, allocations) = count(|| black_box(&a) == black_box(&b));
        assert!(equal);
        assert_eq!(allocations, 0, "compare in place; std has a method for it");
    }

    #[test]
    fn b_add_keeps_the_longer_tail() {
        assert_eq!(
            Poly(vec![1, 2]) + Poly(vec![10, 20, 30]),
            Poly(vec![11, 22, 30])
        );
        assert_eq!(
            Poly(vec![10, 20, 30]) + Poly(vec![1]),
            Poly(vec![11, 20, 30])
        );
        assert_eq!(Poly(vec![]) + Poly(vec![5]), Poly(vec![5]));
    }

    #[test]
    fn b_add_reuses_an_operand_buffer() {
        let a = Poly(vec![1, 2, 3]);
        let b = Poly(vec![1, 1]);
        let (sum, allocations) = count(|| black_box(a + b));
        assert_eq!(sum, Poly(vec![2, 3, 3]));
        assert_eq!(allocations, 0, "a + b owns both buffers already");
    }

    #[test]
    fn b_add_assign_works_in_place() {
        let mut a = Poly(vec![1, 2, 3]);
        let b = Poly(vec![-1, -2]);
        let before = a.0.as_ptr();
        let (_, allocations) = count(|| a += &b);
        assert_eq!(a, Poly(vec![0, 0, 3]));
        assert_eq!(a.0.as_ptr(), before);
        assert_eq!(allocations, 0, "+= should not copy either side");
        let mut short = Poly(vec![1]);
        short += &Poly(vec![1, 1, 1]);
        assert_eq!(short, Poly(vec![2, 1, 1]));
    }
}
