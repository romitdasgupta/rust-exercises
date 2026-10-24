//! LESSON 02 — Recursive data with enums
//! Run: cargo test --test chapter10 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Why does a recursive enum need Box or another indirection? Is the number of nodes determined by the enum size?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain finite sized representation, ownership of child nodes, and structural recursion over a tagged tree.
//! COMPILER / TRANSFER LAB: Remove Box from a recursive child and inspect the infinite-size error. Draw heap ownership of a nested expression. Consider deeply skewed input and discuss recursion depth; avoid constructing unbounded input just to crash the stack.
//! Use examples/ch10_scratch.rs; repair intentional failures before committing.

enum Expr {
    Literal(i16),
    Add(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}
/// A. Evaluate as i64. Inputs in tests are small enough that i64 cannot overflow.
fn evaluate(expr: &Expr) -> i64 {
    todo!("Implement this task")
}
/// B. Count the maximum number of nodes from root to leaf. A literal has depth 1.
fn depth(expr: &Expr) -> usize {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::cell::RefCell;
    use std::hint::black_box;
    fn sample() -> Expr {
        Expr::Add(
            Box::new(Expr::Literal(7)),
            Box::new(Expr::Neg(Box::new(Expr::Literal(2)))),
        )
    }
    #[test]
    fn a_tree_evaluates_by_variant() {
        assert_eq!(evaluate(&sample()), 5);
        assert_eq!(evaluate(&Expr::Neg(Box::new(Expr::Literal(-3)))), 3);
        let min = Expr::Neg(Box::new(Expr::Literal(i16::MIN)));
        assert_eq!(evaluate(&min), 32768, "negate after widening");
        let big = Expr::Add(
            Box::new(Expr::Literal(i16::MAX)),
            Box::new(Expr::Literal(i16::MAX)),
        );
        assert_eq!(evaluate(&big), 65534, "add after widening");
    }
    #[test]
    fn b_depth_tracks_structure() {
        assert_eq!(depth(&Expr::Literal(0)), 1);
        assert_eq!(depth(&sample()), 3);
        let left_heavy = Expr::Add(
            Box::new(Expr::Neg(Box::new(Expr::Neg(Box::new(Expr::Literal(1)))))),
            Box::new(Expr::Literal(2)),
        );
        assert_eq!(depth(&left_heavy), 4, "follow the deeper child");
    }

    struct Noisy<'a>(&'static str, &'a RefCell<Vec<&'static str>>);
    impl Drop for Noisy<'_> {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }
    enum Chain<'a> {
        End(Noisy<'a>),
        Link(Noisy<'a>, Box<Chain<'a>>),
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. How many machine words (usize) does a Box<Expr> take?
        let p1: usize = guess();
        assert_eq!(
            p1 * size_of::<usize>(),
            size_of::<Box<Expr>>(),
            "P1: a Box is a pointer"
        );

        // P2. Dropping a recursive value: in what order are the payloads dropped?
        let log = RefCell::new(Vec::new());
        drop(Chain::Link(
            Noisy("outer", &log),
            Box::new(Chain::End(Noisy("inner", &log))),
        ));
        let p2: Vec<&str> = guess();
        assert_eq!(p2, *log.borrow(), "P2: fields drop in declaration order");

        // P3. How many heap allocations does building sample() make?
        let (tree, allocations) = count(|| black_box(sample()));
        let p3: usize = guess();
        assert_eq!(p3, allocations, "P3: one allocation per Box");
    }
}
