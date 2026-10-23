//! LESSON 09 — Capstone: generic ordered tree
//! Run: cargo test --test chapter10 l09_
//!
//! PREDICT before running: Where should a duplicate go? How does sorted insertion change tree depth? Does generic storage require Clone?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain recursive mutable matching, why every child needs indirection, and how inorder traversal proves the ordering invariant.
//! COMPILER / TRANSFER LAB: Add a size method and test duplicate policy. Compare inserting sorted versus mixed keys and predict maximum depth. Discuss balanced trees and why the standard BTreeSet is usually preferable in production. Extend traversal to borrow values with explicit lifetimes rather than cloning them.
//! Use examples/ch10_scratch.rs; repair intentional failures before committing.

enum Tree<T> {
    Empty,
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}
impl<T: Ord> Tree<T> {
    /// A. Insert an owned value, returning true only when a new node is added.
    /// Equal values are ignored (return false). Do not clone or require Copy.
    fn insert(&mut self, new: T) -> bool {
        todo!("Implement this task")
    }
    /// B. Query by shared reference without changing the tree.
    fn contains(&self, needle: &T) -> bool {
        todo!("Implement this task")
    }
    /// C. Append borrowed values in sorted order to out, preserving any
    /// existing prefix. Return borrows tied to this tree; no Clone bound.
    fn inorder<'a>(&'a self, out: &mut Vec<&'a T>) {
        todo!("Implement this task")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    #[test]
    fn a_insert_maintains_unique_nodes() {
        let mut t = Tree::Empty;
        assert!(t.insert(String::from("m")));
        assert!(t.insert("a".into()));
        assert!(t.insert("z".into()));
        assert!(!t.insert("m".into()));
        assert!(t.contains(&"a".into()));
        assert!(!t.contains(&"b".into()));
    }
    #[test]
    fn a_smaller_values_go_left() {
        let mut t = Tree::Empty;
        for n in [4, 2, 7, 3] {
            assert!(t.insert(n));
        }
        let Tree::Node { value, left, right } = &t else {
            panic!("the first value becomes the root");
        };
        assert_eq!(*value, 4);
        assert!(matches!(**right, Tree::Node { value: 7, .. }));
        let Tree::Node { value, left, right } = &**left else {
            panic!("2 goes left of 4");
        };
        assert_eq!(*value, 2);
        assert!(matches!(**left, Tree::Empty));
        assert!(matches!(**right, Tree::Node { value: 3, .. }));
    }
    #[test]
    fn a_duplicates_are_rejected_without_allocating() {
        let mut t = Tree::Empty;
        for s in ["m", "a", "z"] {
            t.insert(s.to_owned());
        }
        let dup = String::from("a");
        let (added, allocations) = count(|| t.insert(dup));
        assert!(!added);
        assert_eq!(allocations, 0, "a rejected value needs no new node");
        let needle = String::from("z");
        let (found, allocations) = count(|| t.contains(&needle));
        assert!(found);
        assert_eq!(allocations, 0, "contains must not clone the tree's values");
    }
    #[test]
    fn b_search_visits_the_ordered_branch() {
        let mut t = Tree::Empty;
        assert!(!t.contains(&4));
        for n in [4, 2, 7, 1, 3, 6, 9] {
            t.insert(n);
        }
        for n in [1, 2, 3, 4, 6, 7, 9] {
            assert!(t.contains(&n));
        }
        for n in [0, 5, 8, 10] {
            assert!(!t.contains(&n));
        }
    }
    #[test]
    fn c_inorder_borrows_sorted_values() {
        let mut t = Tree::Empty;
        for s in ["m", "a", "z", "b", "a"] {
            t.insert(s.to_owned());
        }
        let prefix = String::from("prefix");
        let mut out = vec![&prefix];
        t.inorder(&mut out);
        assert_eq!(
            out.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            ["prefix", "a", "b", "m", "z"]
        );
        let empty: Tree<String> = Tree::Empty;
        let mut none = Vec::new();
        empty.inorder(&mut none);
        assert!(none.is_empty());
    }
    #[test]
    fn c_inorder_borrows_into_the_tree() {
        let mut t = Tree::Empty;
        for n in [50, 20, 80, 10, 30, 70, 90, 60] {
            t.insert(n);
        }
        let mut out = Vec::with_capacity(16);
        let (_, allocations) = count(|| t.inorder(&mut out));
        assert_eq!(
            out.iter().map(|n| **n).collect::<Vec<_>>(),
            [10, 20, 30, 50, 60, 70, 80, 90]
        );
        assert_eq!(
            allocations, 0,
            "push borrows into `out`; no temporary vectors"
        );
        let Tree::Node { value, .. } = &t else {
            unreachable!()
        };
        assert!(std::ptr::eq(out[3], value), "the root's value is borrowed");
    }
}
