//! LESSON 02 — Borrowed and owned futures
//! Run: cargo test --test chapter20 l02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
//! 1. Can a future outlive a borrowed input?
//! 2. Does async move automatically make a borrowed reference static?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain the difference between moving a reference and moving ownership of its referent.
//! COMPILER LAB (examples/ch20_scratch.rs): Try returning a future that borrows a local String. Repair by moving the String into async move. Check Send with a generic bound; compare Rc versus Arc.

use std::future::Future;
/// A. Return a future borrowing input; its output is the trimmed subslice.
/// No String allocation and no static lifetime assertion.
#[allow(clippy::manual_async_fn)] // the explicit future type is the point of this lesson
fn borrowed_trim<'a>(input: &'a str) -> impl Future<Output = &'a str> + 'a {
    async move { todo!("A: implement the contract") }
}
/// B. Own the input in a Send + static future, returning the trimmed byte length.
/// It must remain usable after the caller drops every other input value.
#[allow(clippy::manual_async_fn)] // the explicit future type is the point of this lesson
fn owned_length(input: String) -> impl Future<Output = usize> + Send + 'static {
    async move { todo!("B: implement the contract") }
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::any::TypeId;
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
    #[test]
    fn a_future_can_return_a_borrow() {
        let input = String::from("  é  ");
        let trimmed = ready(borrowed_trim(&input));
        assert_eq!(trimmed, "é");
        assert_eq!(trimmed.as_ptr(), input[2..].as_ptr());
        assert_eq!(ready(borrowed_trim("\u{3000}wide\t\n")), "wide");
        assert_eq!(ready(borrowed_trim("   ")), "");
    }
    #[test]
    fn a_borrowed_trim_does_not_allocate() {
        let input = String::from("\t padded \n");
        let (trimmed, allocations) = count(|| ready(borrowed_trim(&input)));
        assert_eq!(trimmed, "padded");
        assert_eq!(allocations, 0, "return a subslice of the input");
    }
    #[test]
    fn b_future_owns_its_input() {
        fn require_send_static<T: Send + 'static>(_: &T) {}
        let f = owned_length("  é  ".into());
        require_send_static(&f);
        assert_eq!(ready(f), 2);
        assert_eq!(ready(owned_length("   ".into())), 0);
        assert_eq!(ready(owned_length("\u{3000}🦀\u{3000}".into())), 4);
    }
    #[test]
    fn b_owned_length_moves_the_string_in() {
        let input = String::from("  moved  ");
        let (length, allocations) = count(|| ready(owned_length(input)));
        assert_eq!(length, 5);
        assert_eq!(
            allocations, 0,
            "move the String into the future; do not copy it"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Does the future's String use the same heap buffer as the original?
        let s = String::from("payload");
        let before = s.as_ptr();
        let f = async move { s.as_ptr() };
        let p1: bool = guess();
        assert_eq!(
            p1,
            ready(f) == before,
            "P1: async move moves, it does not copy"
        );

        // P2. Do two calls of one async fn have the same future type?
        // Do two textually identical async blocks?
        async fn answer() -> u8 {
            42
        }
        fn same_type<A: 'static, B: 'static>(_: &A, _: &B) -> bool {
            TypeId::of::<A>() == TypeId::of::<B>()
        }
        let (c1, c2) = (answer(), answer());
        let (b1, b2) = (async { 42u8 }, async { 42u8 });
        let p2: (bool, bool) = guess();
        assert_eq!(
            p2,
            (same_type(&c1, &c2), same_type(&b1, &b2)),
            "P2: future types"
        );
    }
}
