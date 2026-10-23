//! LESSON 06 — Lifetimes and const parameters
//! Run: cargo test --test chapter09 l06_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Which parameter describes a type, which describes an owner relationship, and which describes a length? Are Window<T,3> and Window<T,4> the same type?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain what the borrowed window owns and why its output reference can outlive the short borrow of the wrapper.
//! COMPILER / TRANSFER LAB: Instantiate windows over arrays of lengths 0, 2, and 4. Try passing one length where another is expected. Try returning a window of a local array; inspect the diagnostic. Explain why &[T] has runtime length while &[T; N] carries a compile-time length.
//! Use examples/ch09_scratch.rs; repair intentional failures before committing.

struct Window<'a, T, const N: usize> {
    items: &'a [T; N],
}
impl<'a, T, const N: usize> Window<'a, T, N> {
    /// A. Construct a borrowed fixed-length window; do not copy elements.
    fn new(items: &'a [T; N]) -> Self {
        todo!("Implement this task")
    }
    /// B. Borrow an element with the original array lifetime; out-of-bounds None.
    fn get(&self, index: usize) -> Option<&'a T> {
        todo!("Implement this task")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_length_is_a_const_parameter() {
        let a = [String::from("one"), String::from("two")];
        let w = Window::new(&a);
        assert!(std::ptr::eq(w.items, &a));
        let empty: [u8; 0] = [];
        assert!(Window::new(&empty).get(0).is_none());
    }
    #[test]
    fn b_element_can_outlive_wrapper() {
        let a = [4, 9];
        let out;
        {
            let w = Window::new(&a);
            out = w.get(1).unwrap();
            assert_eq!(w.get(2), None);
        }
        assert!(std::ptr::eq(out, &a[1]));
        let w = Window::new(&a);
        assert!(std::ptr::eq(w.get(0).unwrap(), &a[0]));
        assert_eq!(w.get(usize::MAX), None);
    }

    fn outer_len<T, const N: usize>(_: &[T; N]) -> usize {
        N
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. In machine words: a Window over 1000 u64s, and a &[u64] slice.
        let p1: (usize, usize) = guess();
        let word = size_of::<usize>();
        assert_eq!(
            (p1.0 * word, p1.1 * word),
            (size_of::<Window<'_, u64, 1000>>(), size_of::<&[u64]>()),
            "P1: where the length lives"
        );

        // P2. For this nested array: the N that outer_len sees, and the total
        // size in bytes.
        let grid = [[0u8; 2]; 3];
        let p2: (usize, usize) = guess();
        assert_eq!(
            p2,
            (outer_len(&grid), size_of_val(&grid)),
            "P2: [[T; 2]; 3]"
        );
    }
}
