//! LESSON 04 — Extension traits and qualified calls
//! Run: cargo test --test chapter11 l04_
//!
//! PREDICT before executing: Can this crate implement its own trait for [i32]? Can it
//! implement Display for Vec<i32>?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: How do coherence and the orphan rule preserve unambiguous
//! implementations across crates? Why can importing two traits make method syntax
//! ambiguous?
//!
//! COMPILER / MANUAL LAB (use examples/ch11_scratch.rs):
//! In scratch, define two traits with the same method name and implement both for one type.
//! Trigger ambiguity and resolve it with <Type as Trait>::method. Try impl Display for
//! Vec<i32> and then repair it with a newtype.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

trait SliceStats {
    fn span(&self) -> Option<i64>;
}
impl SliceStats for [i32] {
    /// A. max - min in i64; return None for an empty slice. Never overflow i32.
    fn span(&self) -> Option<i64> {
        todo!("Task 1: implement the documented contract")
    }
}
trait Warm {
    fn tone(&self) -> &'static str;
}
trait Cool {
    fn tone(&self) -> &'static str;
}
struct Palette;
impl Warm for Palette {
    fn tone(&self) -> &'static str {
        "amber"
    }
}
impl Cool for Palette {
    fn tone(&self) -> &'static str {
        "blue"
    }
}
/// B. Call each trait explicitly and return warm/cool separated by a literal slash.
fn tones(value: &Palette) -> String {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_extension_accepts_arrays_vectors_and_extremes() {
        assert_eq!([].as_slice().span(), None);
        assert_eq!([5, 5].span(), Some(0));
        assert_eq!(vec![-3, 8, 1].span(), Some(11));
        assert_eq!([i32::MIN, i32::MAX].span(), Some(4_294_967_295));
    }
    #[test]
    fn b_qualified_calls_disambiguate() {
        assert_eq!(tones(&Palette), "amber/blue");
    }
}
