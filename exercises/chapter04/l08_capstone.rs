//! LESSON 08 — Capstone: owned dispatch queue
//! Run: cargo test --test chapter04 l08_
//!
//! PREDICT before running: Which objects should be moved, cloned, or shared in a batch of independently editable jobs?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Draw every allocation and owner after dispatch. Explain why sharing immutable configuration does not imply sharing mutable job labels.
//! COMPILER / TRANSFER LAB: Change the design to return jobs one at a time with Option::take. Then discuss whether Rc could be replaced by Arc without changing the ownership story.
//! Use examples/ch04_scratch.rs; repair intentional failures before committing.

use std::rc::Rc;
/// A. Consume labels. Return (label, shared_config) jobs in original order.
/// All jobs must point at ONE allocation containing config. No label clones.
fn dispatch(labels: Vec<String>, config: String) -> Vec<(String, Rc<String>)> {
    todo!("Implement this task")
}
/// B. Consume one job, append its config as "label@config", and return the
/// owned label. The String allocation may grow; do not clone either input.
fn finish(job: (String, Rc<String>)) -> String {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    #[test]
    fn a_dispatch_moves_labels_and_shares_config() {
        let label = String::from("one");
        let p = label.as_ptr();
        let jobs = dispatch(vec![label, "two".into()], "zone".into());
        assert_eq!(jobs[0].0.as_ptr(), p);
        assert!(Rc::ptr_eq(&jobs[0].1, &jobs[1].1));
        assert_eq!(Rc::strong_count(&jobs[0].1), 2);
        assert_eq!(jobs[1].0, "two");
        assert!(dispatch(vec![], "unused".into()).is_empty());
    }
    #[test]
    fn b_finish_consumes_a_job() {
        let mut jobs = dispatch(vec!["one".into(), "two".into()], "cfg".into());
        assert_eq!(finish(jobs.remove(0)), "one@cfg");
        assert_eq!(Rc::strong_count(&jobs[0].1), 1);
        assert_eq!(finish((String::new(), Rc::new(String::new()))), "@");
    }

    #[test]
    fn a_dispatch_never_clones_labels_or_config() {
        let labels: Vec<String> = (0..200).map(|i| i.to_string()).collect();
        let (jobs, allocations) = count(|| dispatch(labels, "zone".into()));
        assert_eq!(jobs.len(), 200);
        assert!(
            allocations < 20,
            "200 labels moved should not cost {allocations} allocations"
        );
    }
}
