//! REVIEW 01 — Spaced retrieval: Chapters 3, 5 and 6 inside Chapter 7
//! Run: cargo test --test chapter07 r01_
//!
//! Do this WITHOUT reopening earlier chapters or their hints. Struggling to
//! recall is what makes the knowledge stick; look things up only after a real
//! attempt, and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (early loop exits, borrowed slices,
//! integer ranges) with this chapter's error handling.

use std::num::ParseIntError;

/// A. (Ch6 loops + Ch7 ?) Parse each trimmed line as i64, in order, and
/// return the zero-based index of the first value greater than `limit`, or
/// Ok(None). Propagate a parse error with ?, but stop as soon as you have the
/// answer: lines after it are never parsed.
fn first_over(lines: &[&str], limit: i64) -> Result<Option<usize>, ParseIntError> {
    todo!("A: a for loop with an early return")
}

#[derive(Debug, PartialEq)]
enum FieldError {
    /// The record has only `count` fields.
    Missing { index: usize, count: usize },
    /// The field exists but is blank after trimming.
    Blank { index: usize },
}

/// B. (Ch5 lifetimes + Ch7 typed errors) Split `record` at every ',' and
/// return the trimmed field at zero-based `index`, borrowed from `record`.
/// A missing field reports how many fields exist; a blank one is an error too.
fn field(record: &str, index: usize) -> Result<&str, FieldError> {
    todo!("B: the Ok value borrows from record")
}

/// C. (Ch3 integer ranges + Ch7 conversions) Convert every value to u8 with
/// u8::try_from (no `as`). On the first value out of range, return
/// Err(its zero-based index).
fn narrow(values: &[i32]) -> Result<Vec<u8>, usize> {
    todo!("C: try_from, then map_err to the index")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};

    #[test]
    fn a_first_over_stops_at_the_answer() {
        assert_eq!(first_over(&["1", " 9 ", "bad"], 5), Ok(Some(1)));
        assert!(first_over(&["1", "bad", "9"], 5).is_err());
        assert_eq!(first_over(&["1", "2"], 5), Ok(None));
        assert_eq!(first_over(&[], 5), Ok(None));
        assert_eq!(first_over(&["5", "-9223372036854775808"], 5), Ok(None));
        assert_eq!(first_over(&["6"], 5), Ok(Some(0)));
    }

    #[test]
    fn b_field_borrows_and_reports_typed_errors() {
        let record = String::from("ada, 36 ,, london");
        let (age, allocations) = count(|| field(&record, 1));
        assert_eq!(age, Ok("36"));
        assert_eq!(age.unwrap().as_ptr() as usize, record.as_ptr() as usize + 5);
        assert_eq!(allocations, 0, "split and trim borrow; nothing to collect");
        assert_eq!(field(&record, 2), Err(FieldError::Blank { index: 2 }));
        assert_eq!(field(&record, 3), Ok("london"));
        assert_eq!(
            field(&record, 4),
            Err(FieldError::Missing { index: 4, count: 4 })
        );
        assert_eq!(field("港", 0), Ok("港"));
        assert_eq!(field("", 0), Err(FieldError::Blank { index: 0 }));
        assert_eq!(
            field("a", 9),
            Err(FieldError::Missing { index: 9, count: 1 })
        );
    }

    #[test]
    fn c_narrow_reports_the_first_out_of_range_index() {
        assert_eq!(narrow(&[0, 255, 7]), Ok(vec![0, 255, 7]));
        assert_eq!(narrow(&[1, 256, -1]), Err(1));
        assert_eq!(narrow(&[-1]), Err(0));
        assert_eq!(narrow(&[i32::MIN]), Err(0));
        assert_eq!(narrow(&[]), Ok(vec![]));
    }

    /// P. Earlier-chapter recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        let mut n = 1u32;
        let r = loop {
            n *= 2;
            if n > 20 {
                break n - 1;
            }
        };
        let p1: u32 = guess();
        assert_eq!(p1, r, "P1: Ch6 loop value");

        let (a, b, c) = (-1i16, 3.99f32, 256u16);
        let p2: (u16, u8, u8) = guess();
        assert_eq!(p2, (a as u16, b as u8, c as u8), "P2: Ch6 casts");

        let values = [1, 2, 3];
        let (r1, r2) = (&values[1..], &values[1..]);
        let p3: (bool, usize) = guess();
        assert_eq!(
            p3,
            (std::ptr::eq(r1, r2), r1.len()),
            "P3: Ch5 slice identity"
        );

        let p4: bool = guess();
        assert_eq!(
            p4,
            0.1f32 + 0.2f32 == 0.3f32,
            "P4: Ch3 float equality, in f32"
        );
    }
}
