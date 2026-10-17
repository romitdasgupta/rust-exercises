//! LESSON 03 — Async behavior in traits
//! Run: cargo test --test chapter20 l03_
//!
//! PREDICT before running:
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
async fn total<L: Lookup>(table: &L, keys: &[&str]) -> u64 {
    todo!("B: implement the contract")
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
    fn generic_async_dispatch() {
        let t = Table(vec![("a".into(), 4), ("b".into(), 7)]);
        assert_eq!(ready(total(&t, &["b", "missing", "a", "b"])), 18);
        assert_eq!(ready(t.lookup("x")), None);
        assert_eq!(ready(total(&t, &[])), 0);
    }
}
