//! LESSON 05 — Leaving a replacement behind
//! Run: cargo test --test chapter04 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: What remains in a String after mem::take? Why can replacing a field allow its old owner to escape?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Describe the invariant restored by replace and take, including the state observed by a caller.
//! COMPILER / TRANSFER LAB: Attempt moving a String through a mutable reference. Repair with mem::replace, then mem::take. Compare with Option<String>::take, including a second take.
//! Use examples/ch04_scratch.rs; repair intentional failures before committing.

/// A. Install the owned replacement and return the old String; do not clone.
fn exchange(slot: &mut String, replacement: String) -> String {
    todo!("Implement this task")
}
/// B. Take the optional message, leaving None. Repeated calls return None.
fn deliver(slot: &mut Option<String>) -> Option<String> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    #[test]
    fn a_replacement_is_observable() {
        let mut s = "old".to_owned();
        let p = s.as_ptr();
        let old = exchange(&mut s, "new".into());
        assert_eq!(old.as_ptr(), p);
        assert_eq!(old, "old");
        assert_eq!(s, "new");
        assert_eq!(exchange(&mut s, String::new()), "new");
        assert_eq!(s, "");
    }
    #[test]
    fn b_message_is_delivered_once() {
        let mut slot = Some("ready".into());
        assert_eq!(deliver(&mut slot), Some("ready".into()));
        assert_eq!(slot, None);
        assert_eq!(deliver(&mut slot), None);
    }

    #[test]
    fn a_b_replacement_does_not_clone() {
        let mut slot = "old".to_owned();
        let replacement = "new".to_owned();
        let (_, allocations) = count(|| exchange(&mut slot, replacement));
        assert_eq!(allocations, 0, "exchange should move both values");
        let mut message = Some("ready".to_owned());
        let (_, allocations) = count(|| deliver(&mut message));
        assert_eq!(allocations, 0, "deliver should move the message out");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Length and capacity left behind by mem::take on a String.
        let mut s = String::with_capacity(64);
        s.push_str("payload");
        let taken = std::mem::take(&mut s);
        let p1: (usize, usize) = guess();
        assert_eq!(p1, (s.len(), s.capacity()), "P1: what take leaves behind");

        // P2. Where did the 64-byte buffer go? Capacity of `taken`, at least:
        let p2: usize = guess();
        assert!(
            taken.capacity() >= p2 && p2 >= 64,
            "P2: the old buffer moved out intact"
        );

        // P3. How many heap allocations does mem::take make here?
        let mut v = vec![1, 2, 3];
        let (_, allocations) = count(|| std::mem::take(&mut v));
        let p3: usize = guess();
        assert_eq!(p3, allocations, "P3: Default for Vec does not allocate");
    }
}
