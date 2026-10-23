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
    use std::cell::RefCell;
    fn ready<F: std::future::Future>(future: F) -> F::Output {
        let mut f = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        match f.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(x) => x,
            std::task::Poll::Pending => panic!("expected immediately ready future"),
        }
    }
    fn graph(edges: &[(&str, &[&str])]) -> MemoryGraph {
        MemoryGraph(
            edges
                .iter()
                .map(|(n, out)| (n.to_string(), out.iter().map(|s| s.to_string()).collect()))
                .collect(),
        )
    }
    /// Records every node whose neighbors are requested, in order.
    struct Logged<'a> {
        inner: &'a MemoryGraph,
        calls: RefCell<Vec<String>>,
    }
    impl Graph for Logged<'_> {
        async fn neighbors(&self, node: &str) -> Result<Vec<String>, String> {
            self.calls.borrow_mut().push(node.to_owned());
            self.inner.neighbors(node).await
        }
    }
    #[test]
    fn a_neighbors_are_cloned_in_order() {
        let g = graph(&[("a", &["c", "b", "c"]), ("b", &[])]);
        assert_eq!(
            ready(g.neighbors("a")),
            Ok(vec!["c".into(), "b".into(), "c".into()])
        );
        assert_eq!(ready(g.neighbors("b")), Ok(vec![]));
        assert_eq!(ready(g.neighbors("zz")), Err("zz".into()));
        assert_eq!(g.0.len(), 2, "neighbors borrows the graph");
    }
    #[test]
    fn b_traversal_handles_cycles_and_duplicates() {
        let g = MemoryGraph(BTreeMap::from([
            ("a".into(), vec!["b".into(), "b".into()]),
            ("b".into(), vec!["a".into(), "c".into()]),
            ("c".into(), vec![]),
        ]));
        assert_eq!(ready(crawl(&g, "a")).unwrap(), ["a", "b", "c"]);
        assert_eq!(ready(crawl(&g, "c")).unwrap(), ["c"]);
    }
    #[test]
    fn b_missing_node_is_not_silently_discarded() {
        let g = MemoryGraph(BTreeMap::from([("a".into(), vec!["missing".into()])]));
        assert_eq!(ready(crawl(&g, "a")), Err("missing".into()));
        assert_eq!(ready(crawl(&g, "nowhere")), Err("nowhere".into()));
    }
    #[test]
    fn b_each_node_is_looked_up_once_breadth_first() {
        let g = graph(&[
            ("a", &["b", "c", "b"]),
            ("b", &["d", "a"]),
            ("c", &["d", "c"]),
            ("d", &[]),
            ("unreachable", &["a"]),
        ]);
        let logged = Logged {
            inner: &g,
            calls: RefCell::new(Vec::new()),
        };
        assert_eq!(ready(crawl(&logged, "a")).unwrap(), ["a", "b", "c", "d"]);
        assert_eq!(
            *logged.calls.borrow(),
            ["a", "b", "c", "d"],
            "visit in breadth-first order and expand each node once"
        );
    }
    #[test]
    fn b_first_error_follows_breadth_first_order() {
        // A recursive depth-first walk would reach "deep-missing" first.
        let g = graph(&[("a", &["b", "near-missing"]), ("b", &["deep-missing"])]);
        assert_eq!(ready(crawl(&g, "a")), Err("near-missing".into()));
    }
}
