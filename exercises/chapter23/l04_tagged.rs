//! LESSON 04 — A flagged borrow
//! Run: cargo test --test chapter23 l04_
//!
//! PREDICT before running:
//! 1. Why is an aligned u32 pointer low bit available on this platform?
//! 2. Does stripping a tag restore validity if provenance was already lost?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain lifetime tracking with PhantomData and preserving provenance with pointer map_addr.
//! COMPILER LAB (examples/ch23_scratch.rs): Try returning a Flagged referring to a local value; retain the compiler lifetime error. Do not replace the lifetime with static.

use std::marker::PhantomData;
struct Flagged<'a> {
    tagged: *const u32,
    borrow: PhantomData<&'a u32>,
}
impl<'a> Flagged<'a> {
    /// A. Borrow value and store flag in its pointer's low bit using map_addr.
    /// Check align_of::<u32>() >= 2. Never dereference the tagged pointer.
    fn new(value: &'a u32, flag: bool) -> Self {
        todo!("A: implement the contract")
    }
    fn flag(&self) -> bool {
        self.tagged.addr() & 1 != 0
    }
    /// B. Clear the tag with map_addr, then return the original shared borrow.
    /// SAFETY invariant: pointer comes from a live &'a u32, with only low tag bit
    /// changed, and PhantomData retains that borrow. No mutation is permitted.
    fn value(&self) -> &'a u32 {
        todo!("B: implement the contract")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tag_and_borrow_are_independent() {
        let value = 123;
        for flag in [false, true] {
            let f = Flagged::new(&value, flag);
            assert_eq!(f.flag(), flag);
            assert_eq!(*f.value(), 123);
            assert!(std::ptr::eq(f.value(), &value));
        }
    }
}
