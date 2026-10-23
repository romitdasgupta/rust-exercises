//! LESSON 07 — Capstone: a generic validation pipeline
//! Run: cargo test --test chapter11 l07_
//!
//! PREDICT before running: Design which values are borrowed, owned, statically dispatched,
//! and dynamically dispatched before coding. Predict empty-source and empty-rule behavior.
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why can rules be heterogeneous while the Source associated type stays
//! fixed? Could Rule include a generic method and still support these trait objects?
//!
//! COMPILER / MANUAL LAB (use examples/ch11_scratch.rs):
//! Create a second source backed by an array iterator and a new rule without changing run.
//! Then deliberately change Source::Item to u32 and inspect the bound diagnostic.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

trait Source {
    type Item;
    fn pull(&mut self) -> Option<Self::Item>;
}
trait Rule {
    fn accepts(&self, text: &str) -> bool;
}
struct MinLength(usize);
impl Rule for MinLength {
    fn accepts(&self, s: &str) -> bool {
        s.chars().count() >= self.0
    }
}
struct Forbid(char);
impl Rule for Forbid {
    fn accepts(&self, s: &str) -> bool {
        !s.contains(self.0)
    }
}
/// A. Test every rule; an empty rule list accepts any text.
fn accepted(text: &str, rules: &[&dyn Rule]) -> bool {
    todo!("Task 1: implement the documented contract")
}
/// B. Drain an arbitrary Source of owned strings into a vector, preserving order.
fn collect_source<S: Source<Item = String>>(source: &mut S) -> Vec<String> {
    todo!("Task 2: implement the documented contract")
}
/// C. Consume the source once, returning accepted strings and rejected count.
fn run<S: Source<Item = String>>(source: &mut S, rules: &[&dyn Rule]) -> (Vec<String>, usize) {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    struct Input(std::vec::IntoIter<String>);
    impl Source for Input {
        type Item = String;
        fn pull(&mut self) -> Option<String> {
            self.0.next()
        }
    }
    fn input(words: &[&str]) -> Input {
        Input(
            words
                .iter()
                .map(|s| (*s).into())
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }
    #[test]
    fn a_rules_compose_and_empty_rules_accept() {
        let min = MinLength(2);
        let no = Forbid('!');
        assert!(accepted("🦀a", &[&min, &no]));
        assert!(!accepted("a!", &[&min, &no]));
        assert!(!accepted("🦀", &[&min]));
        assert!(accepted("", &[]));
    }
    #[test]
    fn b_associated_type_source_drains_once() {
        let mut s = input(&["b", "a", "b"]);
        assert_eq!(collect_source(&mut s), ["b", "a", "b"]);
        assert!(collect_source(&mut s).is_empty());
    }
    #[test]
    fn c_pipeline_keeps_order_and_counts_rejections() {
        let mut s = input(&["ok", "!bad", "x", "🦀a", "ok"]);
        let min = MinLength(2);
        let no = Forbid('!');
        assert_eq!(
            run(&mut s, &[&min, &no]),
            (vec!["ok".into(), "🦀a".into(), "ok".into()], 2)
        );
        assert_eq!(run(&mut s, &[]), (vec![], 0));
        assert_eq!(run(&mut input(&[""]), &[]), (vec!["".into()], 0));
    }

    #[test]
    fn c_pipeline_moves_strings_instead_of_copying_them() {
        let words: Vec<String> = (0..200).map(|i| format!("word{i}")).collect();
        let first = words[0].as_ptr();
        let mut s = Input(words.into_iter());
        let min = MinLength(1);
        let ((kept, rejected), allocations) = count(|| run(&mut s, &[&min]));
        assert_eq!((kept.len(), rejected), (200, 0));
        assert_eq!(kept[0].as_ptr(), first, "accepted strings should be moved");
        assert!(
            allocations <= 12,
            "200 accepted strings cost {allocations} allocations"
        );
    }

    #[test]
    fn b_collect_source_moves_strings() {
        let words: Vec<String> = (0..200).map(|i| format!("w{i}")).collect();
        let mut s = Input(words.into_iter());
        let (all, allocations) = count(|| collect_source(&mut s));
        assert_eq!(all.len(), 200);
        assert!(
            allocations <= 12,
            "200 strings cost {allocations} allocations"
        );
    }
}
