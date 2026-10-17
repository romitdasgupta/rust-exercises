//! LESSON 04 — AsRef and AsMut views
//! Run: cargo test --test chapter13 l04_
//!
//! PREDICT before executing: Does AsRef promise that two values have the same equality and
//! hash identity? Does calling as_ref allocate?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why can an API accept multiple owned and borrowed input shapes
//! through one cheap conversion?
//!
//! COMPILER / MANUAL LAB (use examples/ch13_scratch.rs):
//! Write fn path_label<P:AsRef<std::path::Path>>(p:P) and call it with &str, String, Path,
//! and PathBuf. Explain why valid OS paths need not be valid Unicode.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

/// A. Sum bytes into u64. Accept Vec<u8>, &[u8], arrays, and byte strings.
fn byte_total<B: AsRef<[u8]>>(bytes: B) -> u64 {
    todo!("Task 1: implement the documented contract")
}
/// B. Replace each ASCII lowercase byte by its uppercase value; leave all other bytes.
fn uppercase_ascii<B: AsMut<[u8]>>(mut bytes: B) {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_views_accept_owned_and_borrowed_inputs() {
        assert_eq!(byte_total(vec![255, 1]), 256);
        assert_eq!(byte_total([1_u8, 2]), 3);
        assert_eq!(byte_total(b"AZ"), 155);
        assert_eq!(byte_total(&[] as &[u8]), 0);
    }
    #[test]
    fn b_mutable_view_changes_only_ascii_lowercase() {
        let mut v = vec![b'a', b'Z', 0xff, b'z'];
        uppercase_ascii(&mut v);
        assert_eq!(v, [b'A', b'Z', 0xff, b'Z']);
        let mut a = *b"hi!";
        uppercase_ascii(&mut a);
        assert_eq!(&a, b"HI!");
    }
}
