//! LESSON 03 — Self, supertraits, and associated items
//! Run: cargo test --test chapter11 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Is an associated constant stored in every
//! value? Can a default method return Self without a Sized bound?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why does Resettable require both the interface of Named and a sized
//! constructor? Distinguish an associated function from a method.
//!
//! COMPILER / MANUAL LAB (use examples/ch11_scratch.rs):
//! Try calling zero() as an instance method. Then attempt dyn Resettable and explain which
//! associated items prevent dyn compatibility. Add where Self: Sized to a factory-only
//! trait method and retry.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

trait Named {
    fn name(&self) -> &'static str;
}
trait Resettable: Named + Sized {
    const LIMIT: u16;
    fn zero() -> Self;
    fn reset(&mut self);
}
#[derive(Debug, PartialEq)]
struct Count(u16);
impl Named for Count {
    fn name(&self) -> &'static str {
        "count"
    }
}
impl Resettable for Count {
    const LIMIT: u16 = 100;
    /// A. Construct a zero count.
    fn zero() -> Self {
        todo!("Task 1: implement the documented contract")
    }
    /// B. Reset this value using the associated factory, not a magic value.
    fn reset(&mut self) {
        todo!("Task 2: implement the documented contract")
    }
}
/// C. Return a fresh T and a label "NAME (limit LIMIT)" using only trait items.
fn fresh<T: Resettable>() -> (T, String) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_factory_constructs_self() {
        assert_eq!(Count::zero(), Count(0));
    }
    #[test]
    fn b_mutating_method_resets_state() {
        let mut c = Count(89);
        c.reset();
        assert_eq!(c, Count(0));
    }
    #[test]
    fn c_supertrait_and_associated_items_compose() {
        assert_eq!(fresh::<Count>(), (Count(0), "count (limit 100)".into()));
        struct Other;
        impl Named for Other {
            fn name(&self) -> &'static str {
                "other"
            }
        }
        impl Resettable for Other {
            const LIMIT: u16 = 7;
            fn zero() -> Self {
                Other
            }
            fn reset(&mut self) {}
        }
        assert_eq!(fresh::<Other>().1, "other (limit 7)");
    }

    trait Capped {
        const MAX: u32 = 10;
        fn cap(&self, n: u32) -> u32 {
            n.min(Self::MAX)
        }
    }
    struct Loose;
    impl Capped for Loose {}
    struct Tight;
    impl Capped for Tight {
        const MAX: u32 = 3;
    }

    trait Tick {
        fn tick(&mut self);
        fn tick_n(&mut self, n: usize) {
            for _ in 0..n {
                self.tick();
            }
        }
    }
    struct Dial(u8);
    impl Tick for Dial {
        fn tick(&mut self) {
            self.0 = self.0.wrapping_add(100);
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Count has `const LIMIT: u16 = 100`. Size of one Count value in bytes?
        let p1: usize = guess();
        assert_eq!(
            p1,
            size_of::<Count>(),
            "P1: associated consts are not fields"
        );

        // P2. The default `cap` reads Self::MAX. Tight overrides only the constant.
        let p2: (u32, u32) = guess();
        assert_eq!(
            p2,
            (Loose.cap(7), Tight.cap(7)),
            "P2: Self::MAX per implementor"
        );

        // P3. A default method calls the required one three times.
        let mut dial = Dial(0);
        dial.tick_n(3);
        let p3: u8 = guess();
        assert_eq!(p3, dial.0, "P3: default method driving a required one");
    }
}
