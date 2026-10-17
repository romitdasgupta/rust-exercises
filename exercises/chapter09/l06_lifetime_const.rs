//! LESSON 06 — Lifetimes and const parameters
//! Run: cargo test --test chapter09 l06_
//!
//! PREDICT before running: Which parameter describes a type, which describes an owner relationship, and which describes a length? Are Window<T,3> and Window<T,4> the same type?
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
    }
}
