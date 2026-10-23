//! REVIEW 01 — Spaced retrieval: Chapters 10–14 inside Chapter 15
//! Run: cargo test --test chapter15 r01_
//!
//! Do this WITHOUT reopening Chapters 10–14 or their hints. Struggling to recall
//! is what makes the knowledge stick; look things up only after a real attempt,
//! and note in NOTES.md anything you had to look up.
//!
//! Each task mixes an earlier idea (stateful closures, fallible conversions,
//! enum patterns) with this chapter's iterators.

/// A. (Ch14 move closures + Ch15 filter) Lazily yield each value that is greater
/// than every value before it; the first value always qualifies. Keep the
/// running maximum in state captured by a `move` closure passed to `filter`.
fn record_highs(values: &[i32]) -> impl Iterator<Item = i32> + '_ {
    let mut best: Option<i32> = None;
    values
        .iter()
        .copied()
        .filter(move |&v| -> bool { todo!("A: compare with best, update it, and decide") })
}

#[derive(Debug, PartialEq)]
struct Percent(u8);

/// B. (Ch13 TryFrom) Accept 0..=100; otherwise Err("<value> is out of range").
impl TryFrom<i32> for Percent {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        todo!("B: check the range, then convert")
    }
}

/// B. (Ch15 collect into Result) Convert every value with TryFrom. Return all
/// of them, or the first error.
fn parse_percents(values: &[i32]) -> Result<Vec<Percent>, String> {
    todo!("B: map with Percent::try_from, then collect")
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Push(i64),
    Add,
    Mul,
    Neg,
}

/// C. (Ch10 patterns + Ch15 try_fold) Evaluate the ops in order on a stack,
/// using try_fold. Push pushes its value; Add and Mul pop two values and push
/// the result; Neg replaces the top value with its negation. Return None if an
/// op lacks operands, on overflow (use checked arithmetic), or if the stack does
/// not end with exactly one value.
fn eval(ops: &[Op]) -> Option<i64> {
    todo!("C: try_fold over a Vec<i64> stack; match each op")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;

    #[test]
    fn a_running_maximum_lives_in_the_closure() {
        let v = [3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(record_highs(&v).collect::<Vec<_>>(), [3, 4, 5, 9]);
        assert_eq!(record_highs(&v).take(2).collect::<Vec<_>>(), [3, 4]);
        assert_eq!(
            record_highs(&[i32::MIN, i32::MIN]).collect::<Vec<_>>(),
            [i32::MIN]
        );
        assert_eq!(record_highs(&[2, 2, 2]).count(), 1);
        assert_eq!(record_highs(&[]).count(), 0);
    }

    #[test]
    fn b_conversions_collect_or_fail_fast() {
        assert_eq!(
            parse_percents(&[0, 100, 55]),
            Ok(vec![Percent(0), Percent(100), Percent(55)])
        );
        assert_eq!(
            parse_percents(&[5, 101, -1]),
            Err("101 is out of range".to_string())
        );
        assert_eq!(Percent::try_from(-1), Err("-1 is out of range".to_string()));
        assert_eq!(parse_percents(&[]), Ok(vec![]));
    }

    #[test]
    fn c_stack_machine_checks_everything() {
        use Op::*;
        assert_eq!(eval(&[Push(2), Push(3), Add, Push(4), Mul]), Some(20));
        assert_eq!(eval(&[Push(5), Neg]), Some(-5));
        assert_eq!(eval(&[Add]), None);
        assert_eq!(eval(&[Push(1), Add]), None);
        assert_eq!(eval(&[Push(1), Push(2)]), None);
        assert_eq!(eval(&[]), None);
        assert_eq!(eval(&[Push(i64::MAX), Push(1), Add]), None);
        assert_eq!(eval(&[Push(i64::MIN), Neg]), None);
        assert_eq!(eval(&[Push(7), Push(2), Neg, Mul]), Some(-14));
    }

    #[derive(PartialEq, PartialOrd)]
    enum Level {
        Low,
        High,
    }

    /// P. Chapters 12–14 recall: predict each value BEFORE running.
    #[test]
    fn p_predictions() {
        // P1. (Ch14) A move closure that owns an i32 is Copy.
        let mut total = 0;
        let mut add = move |n: i32| {
            total += n;
            total
        };
        add(5);
        let mut copy = add;
        let p1: (i32, i32, i32) = guess();
        assert_eq!(p1, (add(1), copy(1), total), "P1: copied closure state");

        // P2. (Ch13) (300i32 into u8 with try_into fails?, 300i32 as u8)
        let checked: Result<u8, _> = 300i32.try_into();
        let p2: (bool, u8) = guess();
        assert_eq!(p2, (checked.is_err(), 300i32 as u8), "P2: try_into vs as");

        // P3. (Ch12) Derived ordering: (Low < High, (2, "a") < (1, "z"))
        let p3: (bool, bool) = guess();
        assert_eq!(
            p3,
            (Level::Low < Level::High, (2, "a") < (1, "z")),
            "P3: declaration order and lexicographic tuples"
        );
    }
}
