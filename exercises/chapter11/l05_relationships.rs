//! LESSON 05 — Associated types and generic traits
//! Run: cargo test --test chapter11 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can one type implement Source with two
//! different Item types? Can it implement Combine<u8> and Combine<&str> simultaneously?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: When should callers select a trait parameter, and when should the
//! implementation fix an associated type?
//!
//! COMPILER / MANUAL LAB (use examples/ch11_scratch.rs):
//! Write a generic next_pair<S: Source> returning Option<(S::Item,S::Item)>. Add a bound
//! S::Item: Clone in a where clause, then explain why it is a bound on a different type.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

trait Source {
    type Item;
    fn next_item(&mut self) -> Option<Self::Item>;
}
struct Words(std::vec::IntoIter<String>);
impl Source for Words {
    type Item = String;
    /// A. Yield the next owned word, then None permanently after exhaustion.
    fn next_item(&mut self) -> Option<Self::Item> {
        todo!("Task 1: implement the documented contract")
    }
}
trait Combine<Rhs> {
    type Output;
    fn combine(&self, rhs: Rhs) -> Self::Output;
}
struct Prefix(String);
impl Combine<&str> for Prefix {
    type Output = String;
    /// B. Join prefix and suffix with ':'; preserve empty strings.
    fn combine(&self, rhs: &str) -> Self::Output {
        todo!("Task 2: implement the documented contract")
    }
}
impl Combine<u8> for Prefix {
    type Output = String;
    /// C. Format prefix#number using decimal digits.
    fn combine(&self, rhs: u8) -> Self::Output {
        todo!("Task 3: implement the documented contract")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    fn a_associated_item_is_owned_and_exhaustion_is_stable() {
        let mut s = Words(vec!["a".into(), "β".into()].into_iter());
        assert_eq!(s.next_item(), Some("a".into()));
        assert_eq!(s.next_item(), Some("β".into()));
        assert_eq!(s.next_item(), None);
        assert_eq!(s.next_item(), None);
    }
    #[test]
    fn b_string_rhs_selects_one_implementation() {
        assert_eq!(Prefix("api".into()).combine("v1"), "api:v1");
        assert_eq!(Prefix("".into()).combine(""), ":");
    }
    #[test]
    fn c_numeric_rhs_selects_another_implementation() {
        assert_eq!(Prefix("api".into()).combine(255_u8), "api#255");
    }

    #[test]
    fn c_numeric_rhs_formats_zero_and_empty_prefix() {
        assert_eq!(Prefix("".into()).combine(0_u8), "#0");
        assert_eq!(Prefix("a:b".into()).combine(7_u8), "a:b#7");
    }

    struct Countdown(u32);
    impl Iterator for Countdown {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            if self.0 == 0 {
                None
            } else {
                self.0 -= 1;
                Some(self.0)
            }
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. IntoIterator is implemented for both Vec<u8> and &Vec<u8>, each with its
        // own associated Item. Size in bytes of each Item?
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (
                size_of::<<Vec<u8> as IntoIterator>::Item>(),
                size_of::<<&Vec<u8> as IntoIterator>::Item>()
            ),
            "P1: one type, two impls, two Item types"
        );

        // P2. Countdown only writes `next`. What do the provided methods return?
        let p2: (Vec<u32>, u32) = guess();
        assert_eq!(
            p2,
            (Countdown(4).collect(), Countdown(4).sum()),
            "P2: provided methods come from the associated type"
        );
    }
}
