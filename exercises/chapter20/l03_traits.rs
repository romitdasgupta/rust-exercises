//! LESSON 03 — Async behavior in traits
//! Run: cargo test --test chapter20 l03_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider:
//! 1. Can a trait containing async fn automatically be used as dyn Trait?
//! 2. Does a trait async method promise its returned future is Send?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Explain static dispatch for async trait methods and when explicit boxed futures are useful.
//! COMPILER LAB (examples/ch20_scratch.rs): Try Box<dyn Lookup> for this trait. Then write a separate object-safe trait returning Pin<Box<dyn Future<Output=Option<u32>> + Send + 'a>>.

trait Lookup {
    async fn lookup(&self, key: &str) -> Option<u32>;
}
struct Table(Vec<(String, u32)>);
impl Lookup for Table {
    /// A. Return the first matching entry without consuming the table.
    async fn lookup(&self, key: &str) -> Option<u32> {
        todo!("A: implement the contract")
    }
}
/// B. Use the async trait through a generic bound. Missing keys contribute zero.
/// Await each lookup in place: no boxing and no allocation.
async fn total<L: Lookup>(table: &L, keys: &[&str]) -> u64 {
    todo!("B: implement the contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::{alloc::count, guess};
    use std::{future::Future, pin::Pin};
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
    #[test]
    fn a_lookup_returns_the_first_match() {
        let t = Table(vec![("a".into(), 4), ("b".into(), 7), ("a".into(), 9)]);
        assert_eq!(ready(t.lookup("a")), Some(4));
        assert_eq!(ready(t.lookup("b")), Some(7));
        assert_eq!(ready(t.lookup("x")), None);
        assert_eq!(ready(t.lookup("")), None);
        assert_eq!(t.0.len(), 3, "lookup borrows the table");
    }
    #[test]
    fn b_generic_async_dispatch() {
        let t = Table(vec![("a".into(), 4), ("b".into(), 7)]);
        assert_eq!(ready(total(&t, &["b", "missing", "a", "b"])), 18);
        assert_eq!(ready(t.lookup("x")), None);
        assert_eq!(ready(total(&t, &[])), 0);
    }
    #[test]
    fn b_total_widens_before_adding() {
        let t = Table(vec![("big".into(), u32::MAX)]);
        assert_eq!(
            ready(total(&t, &["big", "big", "big"])),
            3 * u64::from(u32::MAX)
        );
    }
    #[test]
    fn b_total_does_not_box_or_allocate() {
        let t = Table(vec![("a".into(), 4), ("b".into(), 7)]);
        let keys = ["a", "b", "a", "zzz"];
        let (sum, allocations) = count(|| ready(total(&t, &keys)));
        assert_eq!(sum, 15);
        assert_eq!(allocations, 0, "static dispatch needs no Box per lookup");
    }

    trait Ping {
        async fn ping(&self) -> u8;
    }
    struct Pinger;
    impl Ping for Pinger {
        async fn ping(&self) -> u8 {
            7
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Sizes on a 64-bit target: a boxed trait-object future, and a Box<u32>.
        let p1: (usize, usize) = guess();
        assert_eq!(
            p1,
            (
                size_of::<Pin<Box<dyn Future<Output = u32>>>>(),
                size_of::<Box<u32>>()
            ),
            "P1: thin and fat pointers"
        );

        // P2. Heap allocations made by creating and polling an async trait method.
        let (value, allocations) = count(|| ready(Pinger.ping()));
        let p2: usize = guess();
        assert_eq!(
            p2, allocations,
            "P2: async fn in traits is statically dispatched"
        );
        assert_eq!(value, 7);
    }
}
