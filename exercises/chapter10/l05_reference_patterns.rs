//! LESSON 05 — Matching references and mutating payloads
//! Run: cargo test --test chapter10 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: When matching &State with State::Name(name), is name owned or borrowed? What changes when matching &mut State?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain match ergonomics, explicit reference patterns, and the difference between copying a small field and moving an owned payload.
//! COMPILER / TRANSFER LAB: In Rust 2024, match &Some(String::from("x")) using Some(ref s) and inspect the redundant-binding-mode diagnostic; repair to Some(s). Compare explicit &Some(ref s) under a fully explicit reference pattern and `let &[a,b] = &[1,2]`. Try moving a String out of a shared reference and explain why it is forbidden.
//! Use examples/ch10_scratch.rs; repair intentional failures before committing.

enum State {
    Name(String),
    Attempts(u16),
    Closed,
}
/// A. Borrow the name payload as &str only for Name; other variants None.
fn name(state: &State) -> Option<&str> {
    todo!("Implement this task")
}
/// B. Increment Attempts without wrapping. Return false for overflow or a
/// different variant, leaving the entire value unchanged in those cases.
fn increment(state: &mut State) -> bool {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_match_borrows_owned_payload() {
        let s = State::Name("港".into());
        assert_eq!(name(&s), Some("港"));
        if let State::Name(ref text) = s {
            assert_eq!(name(&s).unwrap().as_ptr(), text.as_ptr());
        }
        assert_eq!(name(&State::Closed), None);
        assert_eq!(name(&State::Attempts(1)), None);
        assert_eq!(name(&State::Name(String::new())), Some(""));
    }
    #[test]
    fn b_mutable_pattern_preserves_failed_state() {
        let mut s = State::Attempts(2);
        assert!(increment(&mut s));
        assert!(matches!(s, State::Attempts(3)));
        let mut max = State::Attempts(u16::MAX);
        assert!(!increment(&mut max));
        assert!(matches!(max, State::Attempts(u16::MAX)));
        let mut n = State::Name("x".into());
        assert!(!increment(&mut n));
        assert_eq!(name(&n), Some("x"));
        let mut closed = State::Closed;
        assert!(!increment(&mut closed));
        assert!(matches!(closed, State::Closed));
        let mut zero = State::Attempts(0);
        assert!(increment(&mut zero) && increment(&mut zero));
        assert!(matches!(zero, State::Attempts(2)));
    }

    fn size_of_binding<T>(_: &T) -> usize {
        size_of::<T>()
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Matching &Option<u8> with Some(n): is n a u8 or a &u8?
        // Answer with the size of n's type, in bytes, on a 64-bit target.
        let value = &Some(5u8);
        let mut p1_actual = 0;
        if let Some(n) = value {
            p1_actual = size_of_binding(&n);
        }
        let p1: usize = guess();
        assert_eq!(p1, p1_actual, "P1: default binding mode");

        // P2. Destructuring a &mut tuple binds &mut fields.
        let mut pair = (1, String::from("a"));
        let (n, s) = &mut pair;
        *n += 1;
        s.push('b');
        let p2: (i32, &str) = guess();
        assert_eq!(
            p2,
            (pair.0, pair.1.as_str()),
            "P2: mutating through bindings"
        );

        // P3. Matching `&mut opt` versus matching `opt` by value.
        let mut through_ref = Some(3);
        if let Some(n) = &mut through_ref {
            *n *= 10;
        }
        let by_value = Some(3);
        let mut copied = 0;
        if let Some(mut n) = by_value {
            n *= 10;
            copied = n;
        }
        let p3: (Option<i32>, Option<i32>) = guess();
        assert_eq!(p3, (through_ref, by_value), "P3: which Option changed?");
        assert_eq!(copied, 30);
    }
}
