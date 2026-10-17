//! LESSON 05 — Leaving a replacement behind
//! Run: cargo test --test chapter04 l05_
//!
//! PREDICT before running: What remains in a String after mem::take? Why can replacing a field allow its old owner to escape?
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
}
