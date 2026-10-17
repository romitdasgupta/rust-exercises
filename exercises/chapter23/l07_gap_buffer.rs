//! LESSON 07 — A fixed-capacity byte gap buffer
//! Run: cargo test --test chapter23 l07_
//!
//! PREDICT before running:
//! 1. Why must moving a gap use an overlap-permitting copy?
//! 2. What does start == end mean for insertion?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: State logical length, cursor position, and the gap invariant. Explain why all storage remains initialized in this byte-only version.
//! COMPILER LAB (examples/ch23_scratch.rs): Simulate moving the gap left and right on an eight-byte array. Identify copy source/destination ranges before executing.

struct ByteGap {
    storage: Vec<u8>,
    start: usize,
    end: usize,
}
impl ByteGap {
    fn new(capacity: usize) -> Self {
        Self {
            storage: vec![0; capacity],
            start: 0,
            end: capacity,
        }
    }
    fn len(&self) -> usize {
        self.storage.len() - (self.end - self.start)
    }
    fn contents(&self) -> Vec<u8> {
        self.storage[..self.start]
            .iter()
            .chain(&self.storage[self.end..])
            .copied()
            .collect()
    }
    /// A. Insert at cursor if there is gap capacity. Advance cursor on success.
    fn insert(&mut self, byte: u8) -> bool {
        todo!("A: implement the contract")
    }
    /// B. Move cursor to a logical position 0..=len, using ptr::copy to shift
    /// bytes over the gap. Invalid position returns false with no mutation.
    /// SAFETY: storage is initialized bytes; source/destination must stay in it,
    /// copies may overlap, and bookkeeping must preserve 0<=start<=end<=storage.len().
    fn set_position(&mut self, position: usize) -> bool {
        todo!("B: implement the contract")
    }
    /// C. Delete the byte AFTER the cursor, enlarging the gap. Return None at end.
    fn remove(&mut self) -> Option<u8> {
        todo!("C: implement the contract")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn editing_moves_gap_in_both_directions() {
        let mut g = ByteGap::new(6);
        for b in b"abcd" {
            assert!(g.insert(*b));
        }
        assert!(g.set_position(1));
        assert!(g.insert(b'X'));
        assert_eq!(g.contents(), b"aXbcd");
        assert_eq!(g.remove(), Some(b'b'));
        assert_eq!(g.contents(), b"aXcd");
        assert!(g.set_position(4));
        assert!(g.insert(b'!'));
        assert_eq!(g.contents(), b"aXcd!");
        assert_eq!(g.remove(), None);
        let before = g.contents();
        assert!(!g.set_position(99));
        assert_eq!(g.contents(), before);
    }
    #[test]
    fn zero_and_full_capacity() {
        let mut zero = ByteGap::new(0);
        assert!(zero.set_position(0));
        assert!(!zero.insert(1));
        assert_eq!(zero.remove(), None);
        let mut full = ByteGap::new(1);
        assert!(full.insert(7));
        assert!(!full.insert(8));
        assert!(full.set_position(0));
        assert_eq!(full.remove(), Some(7));
        assert!(full.insert(9));
        assert_eq!(full.contents(), [9]);
    }
}
