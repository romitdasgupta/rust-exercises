//! REVIEW 01 — Spaced retrieval: Chapters 7, 9, and 10 inside Chapter 11
//! Run: cargo test --test chapter11 r01_
//!
//! Do this WITHOUT reopening Chapters 7, 9, or 10 or their hints. Struggling to
//! recall is what makes the knowledge stick; look things up only after a real
//! attempt, and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (recursive enums, generic structs, error
//! propagation) with this chapter's traits and generics.

use std::error::Error;

trait Weight {
    fn weight(&self) -> u64;
}

enum Parcel {
    Letter,
    Packet { grams: u32 },
    Crate(Vec<Parcel>),
}

impl Weight for Parcel {
    /// A. (Ch10 recursive enums + Ch11 trait impls) A Letter weighs 20 grams, a
    /// Packet weighs its grams, and a Crate weighs 500 grams plus everything in it.
    /// Use one `match`; recurse through the trait method.
    fn weight(&self) -> u64 {
        todo!("A: match on the variant; a Crate sums its contents")
    }
}

#[derive(Debug, PartialEq)]
struct Bounds<T> {
    low: T,
    high: T,
}

/// B. (Ch9 generic structs + Ch11 bounds) Return the smallest and largest items,
/// or None for an empty slice. Ask only for the bounds you need.
fn bounds<T: PartialOrd + Copy>(items: &[T]) -> Option<Bounds<T>> {
    todo!("B: start from the first item, then widen")
}

/// C. (Ch7 error propagation + Ch11 trait objects) Parse every item, after
/// trimming, as a u16 and return them in order. Propagate the first parse error
/// with `?` (it must still be a ParseIntError underneath). A parsed 0 is an
/// error with the message "zero is not a valid id".
fn parse_ids(items: &[&str]) -> Result<Vec<u16>, Box<dyn Error>> {
    todo!("C: `?` converts any Error into Box<dyn Error>; so does `.into()` on &str")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::num::ParseIntError;

    #[test]
    fn a_weight_recurses_through_crates() {
        assert_eq!(Parcel::Letter.weight(), 20);
        assert_eq!(Parcel::Packet { grams: 0 }.weight(), 0);
        let nested = Parcel::Crate(vec![
            Parcel::Letter,
            Parcel::Crate(vec![Parcel::Packet { grams: u32::MAX }]),
        ]);
        assert_eq!(nested.weight(), 500 + 20 + 500 + u64::from(u32::MAX));
        assert_eq!(Parcel::Crate(vec![]).weight(), 500);
    }

    #[test]
    fn b_bounds_work_for_any_partially_ordered_copy_type() {
        assert_eq!(bounds(&[3, -7, 9, 0]), Some(Bounds { low: -7, high: 9 }));
        assert_eq!(
            bounds(&[2.5]),
            Some(Bounds {
                low: 2.5,
                high: 2.5
            })
        );
        assert_eq!(
            bounds(&['m', 'a', 'z']),
            Some(Bounds {
                low: 'a',
                high: 'z'
            })
        );
        assert_eq!(bounds::<u8>(&[]), None);
    }

    #[test]
    fn c_errors_propagate_as_trait_objects() {
        assert_eq!(parse_ids(&[" 7", "65535 "]).unwrap(), [7, 65_535]);
        assert!(parse_ids(&[]).unwrap().is_empty());
        let err = parse_ids(&["1", "65536", "x"]).unwrap_err();
        assert!(err.downcast_ref::<ParseIntError>().is_some());
        let err = parse_ids(&["4", "0"]).unwrap_err();
        assert_eq!(err.to_string(), "zero is not a valid id");
        assert!(err.downcast_ref::<ParseIntError>().is_none());
    }

    #[derive(PartialEq, PartialOrd)]
    struct Version {
        minor: u8,
        major: u8,
    }

    /// P. Chapters 5, 7, 9, and 10 recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1 (Ch10). Which arm matches first?
        let label = match (3, -3) {
            (x, y) if x + y == 0 => "opposite",
            (x, _) if x > 0 => "positive",
            _ => "other",
        };
        let p1: &str = guess();
        assert_eq!(p1, label, "P1: guards are tried in order");

        // P2 (Ch9). Derived PartialOrd compares fields in declaration order.
        let newer = Version { major: 2, minor: 0 } > Version { major: 1, minor: 9 };
        let p2: bool = guess();
        assert_eq!(p2, newer, "P2: derived ordering");

        // P3 (Ch7). What does `parse` accept?
        let p3: (Option<i32>, Option<u8>) = guess();
        assert_eq!(
            p3,
            (" 42".parse::<i32>().ok(), "+7".parse::<u8>().ok()),
            "P3: parse is strict about spaces"
        );

        // P4 (Ch5). Equal values, distinct places.
        let a = String::from("x");
        let b = String::from("x");
        let p4: (bool, bool) = guess();
        assert_eq!(p4, (a == b, std::ptr::eq(&a, &b)), "P4: == vs ptr::eq");
    }
}
