//! LESSON 01 — Destruction and resource lifetimes
//! Run: cargo test --test chapter13 l01_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: In what order are local guards destroyed?
//! Does moving a guard create another destructor call? What does drop(guard) consume?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Explain why calling .drop() directly is prohibited and why
//! std::mem::drop is an ordinary generic function. When does forgetting a value leak
//! resources?
//!
//! COMPILER / MANUAL LAB (use examples/ch13_scratch.rs):
//! Create two guards in a nested scope and print their drop order. Move one into a new
//! binding. Then use catch_unwind around a panic with a separate non-panicking scratch Drop
//! logger and inspect the log. The provided exercise Guard deliberately skips its TODO
//! helper during unwinding to avoid double panics; that guard is scaffold protection, not
//! general Drop behavior.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::cell::RefCell;
use std::rc::Rc;
type Log = Rc<RefCell<Vec<String>>>;
/// A. Record exactly one label. The Drop scaffold below calls this helper.
fn record_drop(log: &Log, label: &str) {
    todo!("Task 1: implement the documented contract")
}
struct Guard {
    label: &'static str,
    log: Log,
}
impl Drop for Guard {
    fn drop(&mut self) {
        // Keeps unfinished TODO helpers from causing a second panic during unwinding.
        if !std::thread::panicking() {
            record_drop(&self.log, self.label);
        }
    }
}
/// B. Using guards (not direct log pushes), produce ["early","inner","outer"].
/// Create outer first; inner in a nested scope; explicitly drop early inside it.
fn scoped_order() -> Vec<String> {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_destructor_records_one_event_per_owner() {
        let log = Rc::new(RefCell::new(vec![]));
        {
            let first = Guard {
                label: "one",
                log: Rc::clone(&log),
            };
            let moved = first;
            drop(moved);
        }
        assert_eq!(*log.borrow(), ["one"]);
    }
    #[test]
    fn b_scope_and_explicit_drop_define_event_order() {
        assert_eq!(scoped_order(), ["early", "inner", "outer"]);
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    struct Pair<'a> {
        first: Noisy<'a>,
        second: Noisy<'a>,
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// A wrong prediction is useful: read the failure, then explain it in NOTES.md.
    #[test]
    fn p_predictions() {
        // P1. Locals drop in reverse order. In what order do a struct's fields drop?
        let log = RefCell::new(Vec::new());
        drop(Pair {
            first: Noisy("first", &log),
            second: Noisy("second", &log),
        });
        let p1: Vec<&str> = guess();
        assert_eq!(p1, *log.borrow(), "P1: field drop order");

        // P2. `let _ = ...` vs `let _name = ...`.
        let log = RefCell::new(Vec::new());
        {
            let _ = Noisy("underscore", &log);
            log.borrow_mut().push("middle");
            let _named = Noisy("named", &log);
            log.borrow_mut().push("end");
        }
        let p2: Vec<&str> = guess();
        assert_eq!(p2, *log.borrow(), "P2: `_` does not bind");

        // P3. Assigning to an initialized variable. What is logged BEFORE the block ends?
        let log = RefCell::new(Vec::new());
        let mut slot = Noisy("old", &log);
        assert_eq!(slot.0, "old");
        slot = Noisy("new", &log);
        let p3: Vec<&str> = guess();
        assert_eq!(p3, *log.borrow(), "P3: assignment drops the old value");
        drop(slot);

        // P4. mem::forget: how many destructor calls are logged?
        let log = RefCell::new(Vec::new());
        std::mem::forget(Noisy("forgotten", &log));
        let p4: usize = guess();
        assert_eq!(p4, log.borrow().len(), "P4: forget skips Drop");
    }
}
