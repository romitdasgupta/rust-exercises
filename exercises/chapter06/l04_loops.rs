//! LESSON 04 — for, while, break and continue
//! Run: cargo test --test chapter06 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: What does continue skip? When does a while condition run? Does a range include its endpoint?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain why the sentinel must be checked before filtering negative inputs, and how a loop changes program state.
//! COMPILER / TRANSFER LAB: Rewrite a for loop as while over an index. Deliberately place continue before incrementing that index, reason about nontermination without running it, then repair it. Compare 0..n and 0..=n.
//! Use examples/ch06_scratch.rs; repair intentional failures before committing.

/// A. Scan until the FIRST zero. Skip negatives; sum positives as i64.
/// Use for, break, and continue explicitly. Empty input sums to zero.
fn before_sentinel(values: &[i16]) -> i64 {
    todo!("Implement this task")
}
/// B. Count how many integer halvings bring n to zero. Use while and /=.
fn halvings(mut n: u64) -> u32 {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_stop_and_skip_are_distinct() {
        assert_eq!(before_sentinel(&[3, -2, 4, 0, 9]), 7);
        assert_eq!(before_sentinel(&[-1, 0, 7]), 0);
        assert_eq!(before_sentinel(&[]), 0);
        assert_eq!(before_sentinel(&[5, 0, 0, 6]), 5);
        assert_eq!(before_sentinel(&[i16::MIN, 5]), 5);
    }
    #[test]
    fn a_sum_is_widened_before_adding() {
        assert_eq!(before_sentinel(&[i16::MAX, i16::MAX, i16::MAX]), 98301);
        assert_eq!(before_sentinel(&[i16::MAX; 1000]), 32_767_000);
    }
    #[test]
    fn b_while_has_explicit_progress() {
        assert_eq!(halvings(0), 0);
        assert_eq!(halvings(1), 1);
        assert_eq!(halvings(8), 4);
        assert_eq!(halvings(7), 3);
        assert_eq!(halvings(u64::MAX), 64);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Sums over a half-open and an inclusive range.
        let mut open = 0;
        for i in 0..3 {
            open += i;
        }
        let mut closed = 0;
        for i in 0..=3 {
            closed += i;
        }
        let p1: (i32, i32) = guess();
        assert_eq!(p1, (open, closed), "P1: 0..n excludes n, 0..=n includes it");

        // P2. How many times does the while condition run?
        let mut checks = 0;
        let mut i = 0;
        let mut below_three = |i: i32| {
            checks += 1;
            i < 3
        };
        while below_three(i) {
            i += 1;
        }
        let p2: i32 = guess();
        assert_eq!(p2, checks, "P2: the condition runs once more than the body");

        // P3. continue skips the rest of one iteration; break leaves the loop.
        let mut kept = Vec::new();
        for i in 1..=6 {
            if i % 3 == 0 {
                continue;
            }
            if i == 5 {
                break;
            }
            kept.push(i);
        }
        let p3: Vec<i32> = guess();
        assert_eq!(p3, kept, "P3: continue versus break");
    }
}
