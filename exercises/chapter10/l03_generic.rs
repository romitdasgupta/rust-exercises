//! LESSON 03 — Generic enums
//! Run: cargo test --test chapter10 l03_
//!
//! PREDICT before running: Can a generic enum carry unrelated types in different variants? Must either type implement Clone for a consuming swap?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain which type arguments change when variants are exchanged and why matching can transfer ownership without bounds.
//! COMPILER / TRANSFER LAB: Instantiate Either with String/u32 and with two borrowed values. Try adding an unnecessary Copy bound and explain the excluded valid cases. Compare Either with Result: which application meaning does each name communicate?
//! Use examples/ch10_scratch.rs; repair intentional failures before committing.

#[derive(Debug, PartialEq)]
enum Either<L, R> {
    Left(L),
    Right(R),
}
/// A. Consume and swap alternatives: Left(l) becomes Right(l), and vice versa.
fn swap<L, R>(value: Either<L, R>) -> Either<R, L> {
    todo!("Implement this task")
}
/// B. Borrow the payload while retaining its variant and without allocating.
fn borrowed<L, R>(value: &Either<L, R>) -> Either<&L, &R> {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_swap_moves_noncopy_payloads() {
        assert_eq!(
            swap::<String, u32>(Either::Left("x".into())),
            Either::Right("x".into())
        );
        assert_eq!(swap::<String, u32>(Either::Right(9)), Either::Left(9));
    }
    #[test]
    fn b_payload_borrow_keeps_identity() {
        let v: Either<String, u32> = Either::Left("x".into());
        match (borrowed(&v), &v) {
            (Either::Left(a), Either::Left(b)) => assert!(std::ptr::eq(a, b)),
            _ => panic!("wrong variant"),
        };
        assert_eq!(
            borrowed(&Either::<String, u32>::Right(4)),
            Either::Right(&4)
        );
    }
}
