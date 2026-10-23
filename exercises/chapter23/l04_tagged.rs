//! LESSON 04 — A flagged borrow
//! Run: cargo test --test chapter23 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Why is an aligned u32 pointer low bit available on this platform? Does stripping a tag restore validity if provenance was already lost?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
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
    use practice::guess;
    #[test]
    fn a_b_tag_and_borrow_are_independent() {
        let value = 123;
        for flag in [false, true] {
            let f = Flagged::new(&value, flag);
            assert_eq!(f.flag(), flag);
            assert_eq!(*f.value(), 123);
            assert!(std::ptr::eq(f.value(), &value));
        }
    }
    #[test]
    fn a_only_the_low_bit_changes() {
        let value = 7_u32;
        let original = (&value as *const u32).addr();
        assert_eq!(Flagged::new(&value, false).tagged.addr(), original);
        assert_eq!(Flagged::new(&value, true).tagged.addr(), original | 1);
    }
    #[test]
    fn b_each_element_keeps_its_own_address() {
        let values = [10_u32, 20, 30, 40];
        let flags: Vec<Flagged<'_>> = values
            .iter()
            .enumerate()
            .map(|(i, v)| Flagged::new(v, i % 2 == 1))
            .collect();
        for (i, f) in flags.iter().enumerate() {
            assert_eq!(f.flag(), i % 2 == 1);
            assert!(std::ptr::eq(f.value(), &values[i]));
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Alignment of u32 and of u8 (how many low address bits are free?).
        let p1: (usize, usize) = guess();
        assert_eq!(p1, (align_of::<u32>(), align_of::<u8>()), "P1: alignment");

        // P2. Tag a pointer, then clear the tag. Low bit while tagged, and is
        // the cleared pointer equal to the original?
        let value = 5_u32;
        let original = &value as *const u32;
        let tagged = original.map_addr(|a| a | 1);
        let cleared = tagged.map_addr(|a| a & !1);
        let p2: (usize, bool) = guess();
        assert_eq!(p2, (tagged.addr() & 1, cleared == original), "P2: map_addr");

        // P3. Size of a Flagged, and of its PhantomData field, on a 64-bit target.
        let p3: (usize, usize) = guess();
        assert_eq!(
            p3,
            (size_of::<Flagged<'_>>(), size_of::<PhantomData<&u32>>()),
            "P3: what PhantomData costs"
        );
    }
}
