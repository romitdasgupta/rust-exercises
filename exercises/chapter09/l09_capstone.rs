//! LESSON 09 — Capstone: validated inventory
//! Run: cargo test --test chapter09 l09_
//!
//! PREDICT before running: Which fields should be private, which methods mutate inventory, and which statistic may change through a shared borrow?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain the constructor invariant, atomic failure of restock, and why Cell is appropriate for a single-threaded read statistic.
//! COMPILER / TRANSFER LAB: Move the type into a disposable library module and attempt direct field construction from an external binary. Add a maximum stock invariant and revise every constructor/mutator. Discuss the changes needed for sharing read statistics across threads.
//! Use examples/ch09_scratch.rs; repair intentional failures before committing.

use std::cell::Cell;
struct Inventory {
    name: String,
    stock: u16,
    reads: Cell<u32>,
}
impl Inventory {
    /// A. Trim and own a nonempty name; initialize read count to zero.
    fn new(name: &str, stock: u16) -> Option<Self> {
        todo!("Implement this task")
    }
    /// B. Add stock if it fits u16; return false without mutation on overflow.
    fn restock(&mut self, amount: u16) -> bool {
        todo!("Implement this task")
    }
    /// C. Return (borrowed_name,stock) and saturating-increment reads through
    /// &self. Do not clone the name or increment reads in other methods.
    fn inspect(&self) -> (&str, u16) {
        todo!("Implement this task")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_constructor_establishes_invariants() {
        let i = Inventory::new(" 港 ", 3).unwrap();
        assert_eq!(i.name, "港");
        assert_eq!(i.reads.get(), 0);
        assert!(Inventory::new(" \n", 0).is_none());
    }
    #[test]
    fn b_failed_mutation_leaves_stock_unchanged() {
        let mut i = Inventory::new("x", 65530).unwrap();
        assert!(i.restock(5));
        assert!(!i.restock(1));
        assert_eq!(i.stock, 65535);
        assert_eq!(i.reads.get(), 0);
    }
    #[test]
    fn c_observation_updates_only_the_statistic() {
        let i = Inventory::new("x", 8).unwrap();
        assert_eq!(i.inspect(), ("x", 8));
        assert_eq!(i.inspect(), ("x", 8));
        assert_eq!(i.reads.get(), 2);
        assert_eq!(i.inspect().0.as_ptr(), i.name.as_ptr());
        i.reads.set(u32::MAX);
        i.inspect();
        assert_eq!(i.reads.get(), u32::MAX);
    }
}
