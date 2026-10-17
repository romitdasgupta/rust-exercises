//! LESSON 02 — Recursive data with enums
//! Run: cargo test --test chapter10 l02_
//!
//! PREDICT before running: Why does a recursive enum need Box or another indirection? Is the number of nodes determined by the enum size?
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
    }
    #[test]
    fn b_depth_tracks_structure() {
        assert_eq!(depth(&Expr::Literal(0)), 1);
        assert_eq!(depth(&sample()), 3);
    }
}
