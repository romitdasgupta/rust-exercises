//! LESSON 01 — A safe invariant boundary
//! Run: cargo test --test chapter23 l01_
//!
//! PREDICT before running:
//! 1. Does unsafe disable borrow checking outside the unsafe operation?
//! 2. Is every ASCII byte sequence valid UTF-8, including zero?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: State the private invariant, identify each unsafe operation, and explain why all public operations preserve it.
//! COMPILER LAB (examples/ch23_scratch.rs): Try constructing the wrapper with its private field from another module. Compare checked String::from_utf8 with unchecked conversion; never call unchecked on invalid data.

#[derive(Debug)]
struct Ascii(Vec<u8>); // invariant: every byte <= 0x7f
impl Ascii {
    /// A. Validate once; return the original bytes unchanged on rejection.
    fn new(bytes: Vec<u8>) -> Result<Self, Vec<u8>> {
        todo!("A: implement the contract")
    }
    /// B. Borrow the text without allocation. Practice a minimal unsafe block
    /// with a SAFETY comment deriving UTF-8 validity from the private invariant.
    fn as_str(&self) -> &str {
        todo!("B: implement the contract")
    }
    fn lowercase(&mut self) {
        self.0.make_ascii_lowercase();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validation_and_mutation_preserve_invariant() {
        let mut s = Ascii::new(b"A\0Z".to_vec()).unwrap();
        assert_eq!(s.as_str(), "A\0Z");
        s.lowercase();
        assert_eq!(s.as_str(), "a\0z");
        assert_eq!(s.as_str().as_ptr(), s.0.as_ptr());
        assert_eq!(Ascii::new(vec![255]).unwrap_err(), [255]);
        assert_eq!(Ascii::new(vec![]).unwrap().as_str(), "");
    }
}
