//! LESSON 07 — Capstone: asynchronous graph traversal
//! Run: cargo test --test chapter20 l07_
//!
//! PREDICT before running:
//! 1. Can a graph traversal terminate without tracking visited nodes?
//! 2. Does using await alone make this traversal concurrent?
//!
//! IMPLEMENT the tasks; keep the tests.
//! EXPLAIN: Describe the sequential core honestly. Design the bounded spawned-task version from LABS.md, with task results, cancellation, and shutdown accounted for.
//! COMPILER LAB (examples/ch20_scratch.rs): Change the Lookup trait future to borrow returned neighbors. Which lifetimes become tied to the graph, and what changes if tasks own graph Arc handles?

use std::collections::{BTreeMap, BTreeSet, VecDeque};
trait Graph {
    async fn neighbors(&self, node: &str) -> Result<Vec<String>, String>;
}
struct MemoryGraph(BTreeMap<String, Vec<String>>);
impl Graph for MemoryGraph {
    /// A. Return cloned neighbors; a missing node is Err(node.to_owned()).
    async fn neighbors(&self, node: &str) -> Result<Vec<String>, String> {
        todo!("A: implement the contract")
    }
}
/// B. Traverse breadth-first, awaiting one node at a time. Visit each reachable
/// node once; on the first lookup error return it. Return visited names sorted.
/// This deliberately models async composition, not concurrent scheduling.
async fn crawl<G: Graph>(graph: &G, start: &str) -> Result<Vec<String>, String> {
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
    fn traversal_handles_cycles_and_duplicates() {
        let g = MemoryGraph(BTreeMap::from([
            ("a".into(), vec!["b".into(), "b".into()]),
            ("b".into(), vec!["a".into(), "c".into()]),
            ("c".into(), vec![]),
        ]));
        assert_eq!(ready(crawl(&g, "a")).unwrap(), ["a", "b", "c"]);
        assert_eq!(ready(crawl(&g, "c")).unwrap(), ["c"]);
    }
    #[test]
    fn missing_node_is_not_silently_discarded() {
        let g = MemoryGraph(BTreeMap::from([("a".into(), vec!["missing".into()])]));
        assert_eq!(ready(crawl(&g, "a")), Err("missing".into()));
    }
}
