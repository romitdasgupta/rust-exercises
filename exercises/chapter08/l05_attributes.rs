//! LESSON 05 — Attributes, cfg and included files
//! Run: cargo test --test chapter08 l05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Is code behind #[cfg(FALSE)] type-checked? Does cfg!(FALSE) remove the unselected branch from type checking?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain conditional compilation versus runtime branching, environment capture at compilation, and file inclusion paths.
//! COMPILER / TRANSFER LAB: Compare #[cfg(any())] on a function with if cfg!(any()) around an invalid expression. Add a feature in a disposable package and compile both ways. Add build.rs that prints cargo::rustc-env=GUIDED_MODE=lab, read it using env!, then explain OUT_DIR and rerun-if-changed. Generate a Rust constant file into OUT_DIR and include! it; compare include!, include_str!, include_bytes!. Never edit this project manifest for the experiment.
//! Use examples/ch08_scratch.rs; repair intentional failures before committing.

static EMBEDDED: &str = include_str!("included_labels.txt");
/// A. Return whether this target belongs to the Unix family using cfg!.
fn unix_target() -> bool {
    todo!("Implement this task")
}
/// B. Borrow nonempty trimmed lines from the embedded file, in order. No
/// per-line String allocation; returned text has static lifetime.
fn embedded_labels() -> Vec<&'static str> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    #[test]
    fn a_target_attribute_is_compile_time_information() {
        assert_eq!(unix_target(), cfg!(target_family = "unix"));
    }
    #[test]
    fn b_file_is_embedded_and_borrowed() {
        assert_eq!(embedded_labels(), ["north", "港", "south"]);
        let start = EMBEDDED.as_ptr() as usize;
        for label in embedded_labels() {
            let p = label.as_ptr() as usize;
            assert!(
                p >= start && p + label.len() <= start + EMBEDDED.len(),
                "{label:?} must be a slice of EMBEDDED, not a copy or a literal"
            );
        }
    }
    #[test]
    fn b_labels_need_no_per_line_allocation() {
        let (labels, allocations) = count(embedded_labels);
        assert_eq!(labels.len(), 3);
        assert!(
            allocations <= 2,
            "only the Vec may allocate; got {allocations}"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// Open included_labels.txt if you like.
    #[test]
    fn p_predictions() {
        // P1. How many lines does str::lines() report for the embedded file?
        let p1: usize = guess();
        assert_eq!(p1, EMBEDDED.lines().count(), "P1: blank lines count too");

        // P2. Empty all() and any() predicates.
        let p2: (bool, bool, bool) = guess();
        assert_eq!(
            p2,
            (cfg!(all()), cfg!(any()), cfg!(not(any()))),
            "P2: all() is true, any() is false"
        );

        // P3. #[cfg] on statements removes them before compilation.
        let mut kept = String::new();
        #[cfg(not(test))]
        kept.push('a');
        kept.push('b');
        #[cfg(test)]
        kept.push('c');
        #[cfg(target_pointer_width = "64")]
        kept.push('d');
        let p3: &str = guess();
        assert_eq!(p3, kept, "P3: attributes on statements");
    }
}
