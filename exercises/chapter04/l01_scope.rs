//! LESSON 01 — Ownership scopes
//! Run: cargo test --test chapter04 l01_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does dropping a Vec<String> also drop its elements? Does a move allocate a new string buffer?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Draw the owner tree for a vector of strings and explain when destruction happens.
//! COMPILER / TRANSFER LAB: Create a nested block containing a String; try using it outside the block. Then move it into an outer binding and show the repaired owner tree. Use a tiny Drop logger to observe reverse declaration order.
//! Use examples/ch04_scratch.rs; repair intentional failures before committing.

/// A. Build two owned labels, "north" and "south", inside a nested block;
/// return their vector so its ownership leaves the block.
fn labels() -> Vec<String> {
    todo!("Implement this task")
}
/// B. Consume the String and return it unchanged. Do not clone it.
fn handoff(value: String) -> String {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::cell::RefCell;
    #[test]
    fn a_nested_scope_returns_owners() {
        assert_eq!(labels(), ["north", "south"]);
    }
    #[test]
    fn b_handoff_preserves_allocation() {
        let s = String::from("🦀 route");
        let p = s.as_ptr();
        let out = handoff(s);
        assert_eq!(out, "🦀 route");
        assert_eq!(out.as_ptr(), p);
        assert_eq!(handoff(String::new()), "");
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. In what order are these two owners dropped?
        let log = RefCell::new(Vec::new());
        {
            let _first = Noisy("first", &log);
            let _second = Noisy("second", &log);
        }
        let p1: Vec<&str> = guess();
        assert_eq!(p1, *log.borrow(), "P1: drop order at the end of a block");

        // P2. Dropping a Vec<Noisy>: are its elements dropped, and in what order?
        let log = RefCell::new(Vec::new());
        drop(vec![Noisy("x", &log), Noisy("y", &log)]);
        let p2: Vec<&str> = guess();
        assert_eq!(
            p2,
            *log.borrow(),
            "P2: a vector owns and drops its elements"
        );

        // P3. After `let w = v;`, does w's first String use the same heap buffer?
        let v = vec![String::from("north")];
        let before = v[0].as_ptr();
        let w = v;
        let p3: bool = guess();
        assert_eq!(
            p3,
            w[0].as_ptr() == before,
            "P3: moving a Vec copies only its header"
        );
    }
}
