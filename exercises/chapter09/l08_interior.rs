//! LESSON 08 — Cell and RefCell
//! Run: cargo test --test chapter09 l08_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can &self methods change a Cell value? What happens if RefCell mutable and shared borrows overlap?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain interior mutability, runtime borrow guards, try_borrow_mut errors, and why RefCell is not a thread synchronization primitive.
//! COMPILER / TRANSFER LAB: Create a RefCell, keep its borrow guard alive, and try borrow_mut inside catch_unwind with AssertUnwindSafe only in scratch. Then use try_borrow_mut to handle the same conflict. Drop the guard and retry. Compare Cell::get for Copy data with Cell::take for owned defaultable data.
//! Use examples/ch09_scratch.rs; repair intentional failures before committing.

use std::cell::{Cell, RefCell};
struct Journal {
    visits: Cell<u32>,
    entries: RefCell<Vec<String>>,
}
impl Journal {
    /// A. Increment shared-access visits with saturation at u32::MAX; return
    /// the new count. This method deliberately takes &self.
    fn visit(&self) -> u32 {
        todo!("Implement this task")
    }
    /// B. Try appending an owned entry. Return false on a runtime borrow
    /// conflict and leave existing entries unchanged; do not panic.
    fn try_append(&self, text: String) -> bool {
        todo!("Implement this task")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_cell_changes_through_shared_access() {
        let j = Journal {
            visits: Cell::new(0),
            entries: RefCell::new(vec![]),
        };
        assert_eq!(j.visit(), 1);
        assert_eq!(j.visit(), 2);
        j.visits.set(u32::MAX);
        assert_eq!(j.visit(), u32::MAX);
    }
    #[test]
    fn b_conflicting_borrow_is_recoverable() {
        let j = Journal {
            visits: Cell::new(0),
            entries: RefCell::new(vec!["first".into()]),
        };
        let guard = j.entries.borrow();
        assert!(!j.try_append("blocked".into()));
        assert_eq!(&*guard, &[String::from("first")]);
        drop(guard);
        assert!(j.try_append("second".into()));
        assert_eq!(
            &*j.entries.borrow(),
            &[String::from("first"), String::from("second")]
        );
    }
    #[test]
    fn b_exclusive_guard_also_blocks_append() {
        let j = Journal {
            visits: Cell::new(0),
            entries: RefCell::new(vec![]),
        };
        let mut guard = j.entries.borrow_mut();
        assert!(!j.try_append("blocked".into()));
        guard.push("held".into());
        drop(guard);
        assert!(j.try_append("after".into()));
        assert_eq!(j.entries.borrow().as_slice(), ["held", "after"]);
        assert_eq!(j.visits.get(), 0, "try_append must not count visits");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. While one shared guard is alive: can you borrow again? Mutably?
        let cell = RefCell::new(vec![1]);
        let reader = cell.borrow();
        let p1: (bool, bool) = guess();
        assert_eq!(
            p1,
            (cell.try_borrow().is_ok(), cell.try_borrow_mut().is_ok()),
            "P1: many readers or one writer"
        );
        drop(reader);

        // P2. Cell::replace returns the old value.
        let c = Cell::new(5);
        let old = c.replace(9);
        let p2: (i32, i32) = guess();
        assert_eq!(p2, (old, c.get()), "P2: Cell::replace");

        // P3. A temporary guard in a match scrutinee, and in a let statement:
        // is the RefCell still borrowed when the next expression runs?
        let in_match = match cell.borrow().len() {
            0 => false,
            _ => cell.try_borrow_mut().is_ok(),
        };
        let after_let = {
            let n = cell.borrow().len();
            cell.try_borrow_mut().is_ok()
        };
        let p3: (bool, bool) = guess(); // (in_match, after_let)
        assert_eq!(p3, (in_match, after_let), "P3: when temporaries drop");
    }
}
