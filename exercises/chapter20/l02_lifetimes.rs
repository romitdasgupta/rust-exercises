//! LESSON 02 — Borrowed and owned futures
//! Run: cargo test --test chapter20 l02_
//!
//! PREDICT before running:
//! 1. Can a future outlive a borrowed input?
//! 2. Does async move automatically make a borrowed reference static?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain the difference between moving a reference and moving ownership of its referent.
//! COMPILER LAB (examples/ch20_scratch.rs): Try returning a future that borrows a local String. Repair by moving the String into async move. Check Send with a generic bound; compare Rc versus Arc.

use std::future::Future;
/// A. Return a future borrowing input; its output is the trimmed subslice.
/// No String allocation and no static lifetime assertion.
fn borrowed_trim<'a>(input: &'a str) -> impl Future<Output = &'a str> + 'a {
    async move { todo!("A: implement the contract") }
}
/// B. Own the input in a Send + static future, returning the trimmed byte length.
/// It must remain usable after the caller drops every other input value.
fn owned_length(input: String) -> impl Future<Output = usize> + Send + 'static {
    async move { todo!("B: implement the contract") }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
    #[test]
    fn future_can_return_a_borrow() {
        let input = String::from("  é  ");
        let trimmed = ready(borrowed_trim(&input));
        assert_eq!(trimmed, "é");
        assert_eq!(trimmed.as_ptr(), input[2..].as_ptr());
    }
    #[test]
    fn future_owns_its_input() {
        fn require_send_static<T: Send + 'static>(_: &T) {}
        let f = owned_length("  é  ".into());
        require_send_static(&f);
        assert_eq!(ready(f), 2);
        assert_eq!(ready(owned_length("   ".into())), 0);
    }
}
