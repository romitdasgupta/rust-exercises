//! LESSON 08 — Cell and RefCell
//! Run: cargo test --test chapter09 l08_
//!
//! PREDICT before running: Can &self methods change a Cell value? What happens if RefCell mutable and shared borrows overlap?
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
}
