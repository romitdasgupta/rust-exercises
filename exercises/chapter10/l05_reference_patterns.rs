//! LESSON 05 — Matching references and mutating payloads
//! Run: cargo test --test chapter10 l05_
//!
//! PREDICT before running: When matching &State with State::Name(name), is name owned or borrowed? What changes when matching &mut State?
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
    #[test]
    fn a_match_borrows_owned_payload() {
        let s = State::Name("港".into());
        assert_eq!(name(&s), Some("港"));
        if let State::Name(ref text) = s {
            assert_eq!(name(&s).unwrap().as_ptr(), text.as_ptr());
        }
        assert_eq!(name(&State::Closed), None);
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
    }
}
