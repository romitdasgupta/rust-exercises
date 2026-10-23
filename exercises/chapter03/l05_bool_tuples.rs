//! LESSON 05 — Explicit conditions, tuples, and unit
//! Run: cargo test --test chapter03 l05_
//!
//! PREDICT: first answer `p_predictions` at the bottom (replace each `guess()`
//! with your answer). Then consider: Does `if 3 { ... }` compile? What are
//! `true as u8` and `false as u8`? How does `(7,)` differ from `(7)`? What type
//! does a function with no written return type return on normal completion?
//!
//! EXPLAIN: Why can a tuple have mixed field types? Why can `t.0` work while
//! `t[i]` doesn't? What is the single value of the unit type?
//!
//! COMPILER LAB: try `1_u8 as bool`, variable indexing of `(3, false)`, and a
//! block ending with `7` versus `7;`. Repair one error at a time.

/// A. A sample count is present exactly when it is nonzero.
fn has_samples(count: usize) -> bool {
    todo!("A: write an explicit comparison")
}

/// B. Convert a Boolean to 0 or 1 using a cast.
fn ready_count(ready: bool) -> u8 {
    todo!("B: convert bool to an integer")
}

/// C. Return the two parts around a BYTE offset, using `split_at`.
/// The caller promises a valid UTF-8 boundary within the string.
fn split_label(text: &str, mid: usize) -> (&str, &str) {
    todo!("C: return a tuple of borrowed string slices")
}

/// D. Assert that size is between 1 and 1_500 inclusive. Return unit on success.
/// Use `assert!` with comparisons. No explicit return value is necessary.
fn validate_packet_size(size: usize) {
    todo!("D: assert the contract, then finish with unit")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;

    #[test]
    fn a_only_nonzero_counts_have_samples() {
        assert!(!has_samples(0));
        assert!(has_samples(1));
        assert!(has_samples(usize::MAX));
    }

    #[test]
    fn b_booleans_convert_to_zero_or_one() {
        assert_eq!(ready_count(false), 0);
        assert_eq!(ready_count(true), 1);
    }

    #[test]
    fn c_tuple_results_can_be_destructured() {
        let (head, tail) = split_label("rust:types", 5);
        assert_eq!(head, "rust:");
        assert_eq!(tail, "types");
        assert_eq!(split_label("abc", 0), ("", "abc"));
        assert_eq!(split_label("abc", 3), ("abc", ""));
        // The offset counts bytes, not characters: "港" is three bytes.
        let text = "港:dock";
        let (head, tail) = split_label(text, 3);
        assert_eq!((head, tail), ("港", ":dock"));
        assert_eq!(tail.as_ptr(), text[3..].as_ptr(), "borrow, don't copy");
    }

    #[test]
    fn d_successful_validation_returns_unit() {
        let result: () = validate_packet_size(1);
        assert_eq!(result, ());
        validate_packet_size(1_500);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn d_zero_is_invalid() {
        validate_packet_size(0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn d_oversized_is_invalid() {
        validate_packet_size(1_501);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Sizes in bytes: unit, and two small tuples.
        let p1: (usize, usize, usize) = guess();
        assert_eq!(
            p1,
            (
                size_of::<()>(),
                size_of::<(u8, u32)>(),
                size_of::<(u8, u8)>()
            ),
            "P1: tuple sizes include alignment padding"
        );

        // P2. Tuples compare field by field, left to right.
        let p2: (bool, bool) = guess();
        assert_eq!(
            p2,
            ((1, 'z') < (2, 'a'), (2, "b") < (2, "a")),
            "P2: lexicographic tuple order"
        );

        // P3. Booleans are ordered too. Sort these flags.
        let mut flags = [true, false, true, true];
        flags.sort();
        let p3: [bool; 4] = guess();
        assert_eq!(p3, flags, "P3: false < true");

        // P4. The right-hand tuple is built before either name is rebound.
        let (a, b) = (1, 2);
        let (a, b) = (b, a + b);
        let p4: (i32, i32) = guess();
        assert_eq!(p4, (a, b), "P4: tuple assignment uses the old values");
    }
}
