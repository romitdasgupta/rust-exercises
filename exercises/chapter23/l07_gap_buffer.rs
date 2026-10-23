//! LESSON 07 — A fixed-capacity byte gap buffer
//! Run: cargo test --test chapter23 l07_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Why must moving a gap use an overlap-permitting copy? What does start == end mean for insertion?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
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
    use practice::guess;
    #[test]
    fn a_b_c_editing_moves_gap_in_both_directions() {
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
    fn a_b_c_zero_and_full_capacity() {
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
    #[test]
    fn b_c_cursor_moves_through_a_full_buffer() {
        let mut g = ByteGap::new(3);
        for b in b"abc" {
            assert!(g.insert(*b));
        }
        assert!(g.set_position(1), "a full buffer has an empty gap");
        assert_eq!(g.contents(), b"abc");
        assert!(!g.insert(b'!'));
        assert_eq!(g.remove(), Some(b'b'));
        assert!(g.set_position(2));
        assert!(g.insert(b'Z'));
        assert_eq!(g.contents(), b"acZ");
        assert_eq!(g.len(), 3);
    }
    #[test]
    fn b_c_repeated_moves_keep_contents_and_length() {
        let mut g = ByteGap::new(8);
        for b in b"hello" {
            assert!(g.insert(*b));
        }
        for position in [0, 5, 2, 4, 1, 3, 0, 5] {
            assert!(g.set_position(position));
            assert_eq!(g.contents(), b"hello");
            assert_eq!(g.len(), 5);
        }
        assert!(!g.set_position(6), "len + 1 is out of range");
        assert!(g.set_position(0));
        assert_eq!(
            g.remove(),
            Some(b'h'),
            "remove takes the byte after the cursor"
        );
        assert!(g.set_position(4));
        assert_eq!(g.remove(), None);
        assert_eq!(g.contents(), b"ello");
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. ptr::copy is an overlap-permitting move (memmove). Copy 3 bytes
        // from index 0 to index 2. What does the array spell?
        let mut a = *b"abcdef";
        let p = a.as_mut_ptr();
        // SAFETY: both ranges (0..3 and 2..5) lie inside the 6-byte array.
        unsafe { std::ptr::copy(p, p.add(2), 3) };
        let p1: &[u8] = guess();
        assert_eq!(p1, &a, "P1: overlapping copy forward");

        // P2. The same idea in safe code: copy 2..5 down to index 0.
        let mut b = *b"abcdef";
        b.copy_within(2..5, 0);
        let p2: &[u8] = guess();
        assert_eq!(p2, &b, "P2: overlapping copy backward");

        // P3. A gap buffer whose gap holds leftover bytes. Contents and len?
        let g = ByteGap {
            storage: b"abXYZcde".to_vec(),
            start: 2,
            end: 5,
        };
        let p3: (&[u8], usize) = guess();
        assert_eq!(p3, (&g.contents()[..], g.len()), "P3: what the gap hides");
    }
}
