//! LESSON 04 — AsRef and AsMut views
//! Run: cargo test --test chapter13 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Does AsRef promise that two values have the
//! same equality and hash identity? Does calling as_ref allocate?
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
/// View the bytes in place; do not copy them.
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
    use practice::{alloc::count, guess};
    use std::path::Path;
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

    #[test]
    fn a_views_do_not_copy() {
        let v: Vec<u8> = (0..=255).collect();
        let (total, allocations) = count(|| byte_total(&v));
        assert_eq!(total, 32_640);
        assert_eq!(allocations, 0, "as_ref borrows; it never copies");
        assert_eq!(byte_total(vec![255; 70_000]), 17_850_000);
    }

    #[test]
    fn b_mutable_view_edits_only_the_borrowed_part() {
        let mut v = *b"abc";
        uppercase_ascii(&mut v[1..]);
        assert_eq!(&v, b"aBC");
        uppercase_ascii(&mut [] as &mut [u8]);
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. AsRef conversions from a str: as bytes, and as a filesystem path.
        let bytes: &[u8] = "é".as_ref();
        let path: &Path = "logs/today.txt".as_ref();
        let p1: (usize, Option<&str>) = guess();
        assert_eq!(
            p1,
            (bytes.len(), path.file_stem().and_then(|s| s.to_str())),
            "P1: one str, two views"
        );

        // P2. Does as_ref on a Vec<u8> and on a String point at the same bytes as the owner?
        let v = vec![1_u8, 2];
        let s = String::from("hi");
        let (vb, sb): (&[u8], &[u8]) = (v.as_ref(), s.as_ref());
        let p2: (bool, bool) = guess();
        assert_eq!(
            p2,
            (vb.as_ptr() == v.as_ptr(), sb.as_ptr() == s.as_ptr()),
            "P2: views are not copies"
        );
    }
}
