//! LESSON 13 — Sensor report capstone
//! Run: cargo test --test chapter03 l13_
//! Finish by running the entire chapter in debug and release profiles.
//!
//! BEFORE CODING: design the representation on paper. Which values own data?
//! Which are borrowed? Why use u16 for one total but u8 for the other? Why can
//! even a u16 total overflow? Predict every result for label " lab🦀 " and
//! readings [250, 10, 0], including the trimmed label's byte/scalar counts.
//!
//! EXPLAIN AFTERWARD: Why is the sorted output independent of the input?
//! Where are numeric conversions necessary? Does aliasing Summary create a new
//! type? What does the compiler guarantee about your &str inputs?
//! Add your own edge-case test, then explain each field without the book.
//!
//! This tuple keeps the challenge within this chapter. A later structs lesson
//! can replace it with a named record without changing its behavior.

// (message, sorted owned readings, (checked total, saturated total),
//  (trimmed label byte length, trimmed label Unicode scalar count))
type Summary = (String, Vec<u8>, (Option<u16>, u8), (usize, usize));

/// A. Sum into u16. Return Some(total), or None if the total cannot fit.
/// Empty input has total zero. Do not wrap, panic, or truncate the sum.
fn checked_total(readings: &[u8]) -> Option<u16> {
    todo!("A: design the sum's overflow behavior explicitly")
}

/// B. Sum into u8 with saturation at 255. Empty input has total zero.
fn saturated_total(readings: &[u8]) -> u8 {
    todo!("B: the same readings, a deliberately different policy")
}

/// C. Build a report:
/// - Trim surrounding whitespace from the label.
/// - Format exactly "LABEL: N readings" (including for N = 1).
/// - Own a sorted copy of the readings; leave the input data unchanged.
/// - Use your two helpers for the totals.
/// - Measure the TRIMMED LABEL's bytes and Unicode scalars, not the message.
/// - Support empty input, non-ASCII labels, duplicates, and total overflow.
fn summarize(label: &str, readings: &[u8]) -> Summary {
    todo!("C: assemble the report from appropriate types and operations")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_checked_total_accepts_the_limit_and_rejects_overflow() {
        assert_eq!(checked_total(&[]), Some(0));
        assert_eq!(checked_total(&[250, 10]), Some(260));
        assert_eq!(checked_total(&vec![255; 257]), Some(u16::MAX));
        assert_eq!(checked_total(&vec![255; 258]), None);
    }

    #[test]
    fn b_saturated_total_clamps_instead_of_wrapping() {
        assert_eq!(saturated_total(&[]), 0);
        assert_eq!(saturated_total(&[10, 20]), 30);
        assert_eq!(saturated_total(&[250, 10]), 255);
        assert_eq!(saturated_total(&[255, 255, 1]), 255);
    }

    #[test]
    fn c_report_combines_text_storage_and_numeric_policies() {
        let readings = [250, 10, 0];
        let (message, sorted, totals, metrics) = summarize(" lab🦀 ", &readings);
        assert_eq!(message, "lab🦀: 3 readings");
        assert_eq!(sorted, [0, 10, 250]);
        assert_eq!(totals, (Some(260), 255));
        assert_eq!(metrics, (7, 4));
        assert_eq!(readings, [250, 10, 0]);
        assert_ne!(sorted.as_ptr(), readings.as_ptr());
    }

    #[test]
    fn c_report_handles_empty_data_and_labels() {
        assert_eq!(
            summarize(" \n\t", &[]),
            (": 0 readings".to_string(), vec![], (Some(0), 0), (0, 0))
        );
    }

    #[test]
    fn c_report_preserves_duplicate_readings_and_counts_scalars() {
        assert_eq!(
            summarize(" e\u{301} ", &[2, 1, 2]),
            (
                "e\u{301}: 3 readings".to_string(),
                vec![1, 2, 2],
                (Some(5), 5),
                (3, 2)
            )
        );
    }

    #[test]
    fn c_report_handles_one_reading_and_checked_overflow() {
        assert_eq!(
            summarize(" A ", &[0]),
            ("A: 1 readings".to_string(), vec![0], (Some(0), 0), (1, 1))
        );
        let readings = vec![255; 258];
        let (message, sorted, totals, metrics) = summarize("max", &readings);
        assert_eq!(message, "max: 258 readings");
        assert_eq!(sorted, readings);
        assert_eq!(totals, (None, 255));
        assert_eq!(metrics, (3, 3));
    }
}
