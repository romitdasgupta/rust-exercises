//! REVIEW 01 — Spaced retrieval: Chapters 5, 10, and 11 inside Chapter 12
//! Run: cargo test --test chapter12 r01_
//!
//! Do this WITHOUT reopening Chapters 5, 10, or 11 or their hints. Struggling
//! to recall is what makes the knowledge stick; look things up only after a
//! real attempt, and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (generic bounds, enum patterns, returning
//! references) with this chapter's operator traits.

use std::ops::{Add, Index, Mul};

/// A. (Ch11 generic bounds + Ch12 arithmetic traits) Return the dot product
/// (sum of pairwise products) of two equally long slices, starting from
/// `T::default()`. Return None when the lengths differ.
fn dot<T>(a: &[T], b: &[T]) -> Option<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    todo!("A: check lengths, then fold over zipped pairs")
}

#[derive(Debug, Clone, Copy)]
enum Money {
    Cents(u32),
    Dollars(u32),
}

impl PartialEq for Money {
    /// B. (Ch10 enum patterns + Ch12 equivalence) Two amounts are equal when they
    /// are worth the same number of cents, whatever the variants:
    /// Dollars(1) == Cents(100). Never overflow, even for Dollars(u32::MAX).
    fn eq(&self, other: &Self) -> bool {
        todo!("B: convert both sides to one unit wide enough to hold them")
    }
}

impl Eq for Money {}

struct Table {
    rows: Vec<(String, u32)>,
}

impl Table {
    /// C1. (Ch5 references) Borrow the value stored for `name`; None if absent.
    /// If a name appears more than once, the first row wins.
    fn get(&self, name: &str) -> Option<&u32> {
        todo!("C1: search rows by reference; return a reference into self")
    }
}

impl Index<&str> for Table {
    type Output = u32;
    /// C2. (Ch12 Index) `table["name"]` reads like `get`, but panics with
    /// "no row named NAME" when the name is absent. Reuse `get`.
    fn index(&self, name: &str) -> &u32 {
        todo!("C2: delegate to get, then turn None into a panic")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;

    #[test]
    fn a_dot_product_is_generic() {
        assert_eq!(dot(&[1, 2, 3], &[4, -5, 6]), Some(12));
        assert_eq!(dot(&[0.5, 2.0], &[4.0, 0.25]), Some(2.5));
        assert_eq!(dot::<u8>(&[], &[]), Some(0));
        assert_eq!(dot(&[1, 2], &[1]), None);
    }

    #[test]
    fn b_money_compares_across_variants() {
        assert_eq!(Money::Dollars(1), Money::Cents(100));
        assert_eq!(Money::Cents(250), Money::Cents(250));
        assert_ne!(Money::Cents(250), Money::Dollars(2));
        assert_ne!(Money::Dollars(u32::MAX), Money::Cents(u32::MAX));
        // 42_949_673 * 100 wraps to 4 in u32 arithmetic.
        assert_ne!(Money::Dollars(42_949_673), Money::Cents(4));
        assert_eq!(Money::Dollars(42_949_672), Money::Cents(4_294_967_200));
    }

    #[test]
    fn c_table_borrows_and_indexes() {
        let t = Table {
            rows: vec![("ada".into(), 36), ("bob".into(), 7), ("ada".into(), 99)],
        };
        assert_eq!(t.get("bob"), Some(&7));
        assert_eq!(t.get("ada"), Some(&36));
        assert_eq!(t.get("Bob"), None);
        assert!(std::ptr::eq(&t["bob"], &t.rows[1].1));
        assert_eq!(t["ada"] + t["bob"], 43);
    }

    #[test]
    #[should_panic(expected = "no row named zed")]
    fn c_index_panics_for_a_missing_name() {
        let t = Table { rows: vec![] };
        let _ = t["zed"];
    }

    /// P. Chapters 3, 7, 10, and 11 recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1 (Ch11). Size of a boxed trait object, in machine words.
        let p1: usize = guess();
        assert_eq!(
            p1 * size_of::<usize>(),
            size_of::<Box<dyn Fn()>>(),
            "P1: Box<dyn Trait> is a fat pointer"
        );

        // P2 (Ch10). Slice patterns.
        let v = [1, 2, 3, 4];
        let [first, .., last] = v;
        let p2: i32 = guess();
        assert_eq!(p2, first * 10 + last, "P2: [first, .., last]");

        // P3 (Ch7). Recovering from a parse failure.
        let p3: u8 = guess();
        assert_eq!(p3, "300".parse::<u8>().unwrap_or(7), "P3: unwrap_or");

        // P4 (Ch3). `as` casts between integer types.
        let p4: (i8, bool) = guess();
        assert_eq!(
            p4,
            (200_u8 as i8, -1_i32 as u32 == u32::MAX),
            "P4: as reinterprets or truncates"
        );
    }
}
