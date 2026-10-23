//! LESSON 03 — Methods and associated items
//! Run: cargo test --test chapter09 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: When does a method take self by value versus &self or &mut self? Does an associated constructor require a receiver?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain the invariant shared by the constructor and mutation method, and how an associated constant belongs to the type.
//! COMPILER / TRANSFER LAB: Try calling a non-method associated function with dot syntax. Move a value into a self-consuming method and try using it afterward. Add two impl blocks and explain why they belong to the same type.
//! Use examples/ch09_scratch.rs; repair intentional failures before committing.

#[derive(Debug, PartialEq)]
struct Gauge {
    reading: u8,
}
impl Gauge {
    const MAX: u8 = 100;
    /// A. Associated constructor: accept only readings <= Self::MAX.
    fn new(reading: u8) -> Option<Self> {
        todo!("Implement this task")
    }
    /// B. Replace the reading if valid; return false without mutation otherwise.
    fn set(&mut self, reading: u8) -> bool {
        todo!("Implement this task")
    }
    fn reading(&self) -> u8 {
        self.reading
    }
    /// C. Consume the gauge and return the reading.
    fn into_reading(self) -> u8 {
        todo!("Implement this task")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    use std::cell::RefCell;
    #[test]
    fn a_associated_constructor_enforces_range() {
        assert_eq!(Gauge::new(100), Some(Gauge { reading: 100 }));
        assert!(Gauge::new(101).is_none());
        assert_eq!(Gauge::new(0), Some(Gauge { reading: 0 }));
        assert!(Gauge::new(u8::MAX).is_none());
    }
    #[test]
    fn b_exclusive_method_preserves_invariant() {
        let mut g = Gauge::new(20).unwrap();
        assert!(g.set(99));
        assert!(!g.set(255));
        assert_eq!(g.reading(), 99);
        assert!(!g.set(101));
        assert_eq!(g.reading(), 99, "a rejected value must not be stored");
        assert!(g.set(100));
        assert_eq!(g.reading(), 100);
    }
    #[test]
    fn c_owned_receiver_finishes_the_value() {
        assert_eq!(Gauge::new(0).unwrap().into_reading(), 0);
        assert_eq!(Gauge::new(42).unwrap().into_reading(), 42);
    }

    trait Describe {
        fn reading(&self) -> u8 {
            0
        }
    }
    impl Describe for Gauge {}

    struct Ticket<'a>(&'a RefCell<Vec<&'static str>>);
    impl Ticket<'_> {
        fn peek(&self) {}
        fn finish(self) {}
    }
    impl Drop for Ticket<'_> {
        fn drop(&mut self) {
            self.0.borrow_mut().push("dropped");
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Gauge has an inherent `reading` method AND a trait method with
        // the same name. Which one does each call run?
        let g = Gauge { reading: 7 };
        let p1: (u8, u8) = guess();
        assert_eq!(
            p1,
            (g.reading(), Describe::reading(&g)),
            "P1: inherent methods win"
        );

        // P2. `peek` borrows, `finish` consumes. When is the ticket dropped?
        let log = RefCell::new(Vec::new());
        let ticket = Ticket(&log);
        ticket.peek();
        log.borrow_mut().push("peeked");
        ticket.finish();
        log.borrow_mut().push("finished");
        let p2: Vec<&str> = guess();
        assert_eq!(p2, *log.borrow(), "P2: a self method owns its receiver");
    }
}
