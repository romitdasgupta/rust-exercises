//! LESSON 06 — Accumulators, extrema, and sequence comparisons
//! Run: cargo test --test chapter15 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: What do sum and product return for empty
//! input? Are any([]) and all([]) true or false? Are iterator comparisons lexicographic?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why must an extrema function specify tie behavior? How do max_by_key
//! and max_by differ?
//!
//! COMPILER / MANUAL LAB (use examples/ch15_scratch.rs):
//! Compare min_by_key and max_by_key on equal keys and record which equal candidate wins.
//! Use min_by/max_by with case-insensitive ASCII comparison. Compare two iterators of
//! unequal length with eq/cmp.
//!
//! EXTREMA LAB: use fresh iterators over [3, -1, 3] to call min and max;
//! check that both return None for empty input. Then compare min_by_key
//! and max_by_key on equal-length words and explain the tie guarantees.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Return (count,sum,product), widening each value to i64 first.
/// Inputs contain at most 8 values of magnitude <=10.
fn totals(values: &[i32]) -> (usize, i64, i64) {
    todo!("Task 1: implement the documented contract")
}
/// B. Return first word with the fewest Unicode scalars, or None for empty.
fn shortest<'a>(words: &'a [&str]) -> Option<&'a str> {
    todo!("Task 2: implement the documented contract")
}
/// C. Return (any negative, all even). Empty input gives (false,true).
fn predicates(values: &[i32]) -> (bool, bool) {
    todo!("Task 3: implement the documented contract")
}
/// D. Compare sequences lexicographically, using iterator comparison.
fn compare(left: &[i32], right: &[i32]) -> std::cmp::Ordering {
    todo!("Task 4: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_empty_identities_and_wide_accumulation() {
        assert_eq!(totals(&[]), (0, 0, 1));
        assert_eq!(totals(&[2, -3, 4]), (3, 3, -24));
        assert_eq!(totals(&[0, 9]), (2, 9, 0));
    }
    #[test]
    fn b_key_extremum_uses_first_tie() {
        assert_eq!(shortest(&["abc", "🦀", "x", "éé"]), Some("🦀"));
        assert_eq!(shortest(&[]), None);
    }
    #[test]
    fn c_predicates_handle_empty_and_odd_values() {
        assert_eq!(predicates(&[]), (false, true));
        assert_eq!(predicates(&[-2, 4]), (true, true));
        assert_eq!(predicates(&[1, 2]), (false, false));
    }
    #[test]
    fn d_comparison_is_lexicographic() {
        use std::cmp::Ordering::*;
        assert_eq!(compare(&[1, 3], &[1, 2, 100]), Greater);
        assert_eq!(compare(&[1], &[1, 0]), Less);
        assert_eq!(compare(&[], &[]), Equal);
        assert_eq!(compare(&[-1], &[0]), Less);
        assert_eq!(compare(&[], &[i32::MIN]), Less);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. Over an empty slice: (sum, product, max).
        let empty: [i32; 0] = [];
        let p1: (i32, i32, Option<i32>) = guess();
        assert_eq!(
            p1,
            (
                empty.iter().sum(),
                empty.iter().product(),
                empty.iter().copied().max()
            ),
            "P1: identities and the empty extremum"
        );

        // P2. Ties on length: (max_by_key's choice, min_by_key's choice).
        let words = ["bb", "a", "cc", "d"];
        let longest = words.iter().max_by_key(|w| w.len()).copied();
        let shortest = words.iter().min_by_key(|w| w.len()).copied();
        let p2: (Option<&str>, Option<&str>) = guess();
        assert_eq!(p2, (longest, shortest), "P2: which tie wins");

        // P3. ([1, 2] < [1, 2, 0] as iterators, any over empty, all over empty)
        let p3: (bool, bool, bool) = guess();
        assert_eq!(
            p3,
            (
                [1, 2].iter().lt([1, 2, 0].iter()),
                empty.iter().any(|&n| n > 0),
                empty.iter().all(|&n| n > 0)
            ),
            "P3: prefixes and vacuous truth"
        );
    }
}
