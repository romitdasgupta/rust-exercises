//! LESSON 03 — Paths, imports and reexports
//! Run: cargo test --test chapter08 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does use create a new item or make a name available? Does pub struct make every field public?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain absolute paths, self/super/crate, imported aliases, prelude names, and the role of a facade reexport.
//! COMPILER / TRANSFER LAB: In a disposable library expose Ticket via pub use while keeping its implementation module private. From a separate binary try constructing Ticket with a private field, then use its constructor. Try pub(crate) and pub(super) and inspect external access. Use std::option::Option::Some explicitly, then remove the qualification and identify the prelude. Make one field pub and explain the invariant lost.
//! Use examples/ch08_scratch.rs; repair intentional failures before committing.

mod implementation {
    #[derive(Debug, PartialEq)]
    pub struct Ticket {
        name: String,
    }
    impl Ticket {
        /// A. Trim and own a nonempty name; reject an empty name.
        pub fn new(name: &str) -> Option<Self> {
            todo!("Implement this task")
        }
        pub fn name(&self) -> &str {
            &self.name
        }
    }
}
use implementation::Ticket as InternalTicket;
pub use implementation::Ticket;
/// B. Use the imported alias for construction; expose the reexported type.
fn issued(name: &str) -> Option<Ticket> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::any::{TypeId, type_name};
    #[test]
    fn a_constructor_controls_the_boundary() {
        assert_eq!(Ticket::new(" 港 ").unwrap().name(), "港");
        assert!(Ticket::new(" \n").is_none());
        assert!(Ticket::new("").is_none());
        assert_eq!(Ticket::new("\u{3000}a b\t").unwrap().name(), "a b");
    }
    #[test]
    fn b_reexport_and_alias_name_the_same_type() {
        let a: Ticket = issued("ready").unwrap();
        let b: InternalTicket = a;
        assert_eq!(b.name(), "ready");
        assert!(issued("").is_none());
        assert_eq!(issued(" x ").map(|t| t.name().len()), Some(1));
    }

    fn drop<T>(_value: T) -> &'static str {
        "local drop"
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Do the alias and the reexport name one type? What path does
        // type_name report for it?
        let p1: (bool, &str) = guess();
        assert_eq!(
            p1,
            (
                TypeId::of::<Ticket>() == TypeId::of::<InternalTicket>(),
                type_name::<Ticket>()
            ),
            "P1: use adds a name, not a type"
        );

        // P2. A local item named like a prelude function. Which one runs?
        let p2: &str = guess();
        assert_eq!(p2, drop(5), "P2: local items shadow the prelude");
    }
}
