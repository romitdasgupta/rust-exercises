//! LESSON 08 — Growable owned sequences
//! Run: cargo test --test chapter03 l08_
//!
//! PREDICT: first answer `p_predictions` at the bottom (replace each `guess()`
//! with your answer). Then consider: What are the length and minimum capacity
//! of `Vec::<u8>::with_capacity(8)`? Can you immediately index its first
//! element? Which three pieces of metadata describe a Vec? What can happen when
//! a push exceeds its capacity?
//!
//! EXPLAIN: Why is capacity not the same as length? Why avoid asserting a
//! particular allocation growth factor? How do insert/remove differ from pop
//! in the number of elements they may need to shift?
//!
//! COMPILER LAB: compare `vec![5; n]` with `Vec::<u8>::with_capacity(n)` by
//! printing length and capacity. Try popping each. Predict before running.

/// A. Collect the integers in 0..end into a vector. end may be zero or negative.
fn collected_range(end: i32) -> Vec<i32> {
    todo!("A: let the return type select the collection")
}

/// B. The input has at least two elements. In this order:
/// insert 99 at index 1, remove index 0, then push 7. Return the edited vector.
fn edit_queue(mut queue: Vec<i32>) -> Vec<i32> {
    todo!("B: practice insert, remove, and push on the same vector")
}

/// C. Allocate with capacity for count elements. Record initial length and
/// capacity, then push the byte 42 exactly count times into that vector.
/// Return (filled vector, INITIAL length, INITIAL capacity).
fn reserved_bytes(count: usize) -> (Vec<u8>, usize, usize) {
    todo!("C: reserving memory is different from creating elements")
}

/// D. Remove and return the last element, or None if there is none.
fn pop_last(values: &mut Vec<i32>) -> Option<i32> {
    todo!("D: the vector's pop operation already has the needed return type")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::hint::black_box;

    #[test]
    fn a_collect_can_infer_vec_from_the_return_type() {
        assert_eq!(collected_range(5), [0, 1, 2, 3, 4]);
        assert!(collected_range(0).is_empty());
        assert!(collected_range(-3).is_empty());
    }

    #[test]
    fn b_edits_use_the_current_indices() {
        assert_eq!(edit_queue(vec![10, 20, 30]), [99, 20, 30, 7]);
        assert_eq!(edit_queue(vec![1, 2]), [99, 2, 7]);
    }

    #[test]
    fn c_capacity_reservation_does_not_set_length() {
        for count in [0, 1, 17, 100] {
            let (values, initial_len, initial_capacity) = reserved_bytes(count);
            assert_eq!(initial_len, 0);
            assert!(initial_capacity >= count);
            assert_eq!(values, vec![42; count]);
            assert_eq!(values.capacity(), initial_capacity);
        }
    }

    #[test]
    fn d_pop_tracks_the_changing_length() {
        let mut values = vec![3, 8];
        assert_eq!(pop_last(&mut values), Some(8));
        assert_eq!(values, [3]);
        assert_eq!(pop_last(&mut values), Some(3));
        assert_eq!(pop_last(&mut values), None);
        assert!(values.is_empty());
    }

    #[test]
    fn c_reserving_then_pushing_allocates_once() {
        let ((values, _, _), allocations) = count(|| reserved_bytes(100));
        assert_eq!(values.len(), 100);
        assert!(
            allocations <= 1,
            "reserve once, then push into that buffer; got {allocations} allocations"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Length, and whether capacity is at least 8.
        let reserved = Vec::<u8>::with_capacity(8);
        let p1: (usize, bool) = guess();
        assert_eq!(
            p1,
            (reserved.len(), reserved.capacity() >= 8),
            "P1: capacity is not length"
        );

        // P2. Contents after these edits, in this order.
        let mut v = vec![1, 2, 3];
        v.insert(1, 9);
        v.remove(0);
        v.pop();
        let p2: Vec<i32> = guess();
        assert_eq!(p2, v, "P2: insert, remove, pop");

        // P3. Heap allocations made by Vec::new() and by with_capacity(4).
        let (_, empty) = count(|| black_box(Vec::<u8>::new()));
        let (_, reserved) = count(|| black_box(Vec::<u8>::with_capacity(4)));
        let p3: (usize, usize) = guess();
        assert_eq!(p3, (empty, reserved), "P3: an empty Vec does not allocate");
    }
}
