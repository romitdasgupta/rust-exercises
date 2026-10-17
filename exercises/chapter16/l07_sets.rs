//! LESSON 07 — Set algebra and equal-but-distinct values
//! Run: cargo test --test chapter16 l07_
//!
//! PREDICT before executing: If two set values compare equal but have different metadata,
//! which value does insert retain? How does replace differ?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: Why must equality and hashing use the same identity fields? Why is
//! mutating an element’s identity while stored a logic error?
//!
//! COMPILER / MANUAL LAB (use examples/ch16_scratch.rs):
//! Use BTreeSet union, intersection, difference, and symmetric_difference on {1,2,3} and
//! {3,4}; predict all ordered results. Compare is_subset and is_disjoint, and do the
//! equivalent HashSet operations without assuming iteration order.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

use std::collections::{BTreeSet, HashSet};
use std::hash::{Hash, Hasher};
/// A. Return sorted (union,intersection,left-only,symmetric-difference).
fn algebra(left: &[i32], right: &[i32]) -> (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) {
    todo!("Task 1: implement the documented contract")
}
#[derive(Debug)]
struct User {
    id: u64,
    display: String,
}
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for User {}
impl Hash for User {
    /// B. Hash exactly the identity used by Eq; exclude display metadata.
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!("Task 2: implement the documented contract")
    }
}
/// C. Replace an equal-id representative, returning its old display if present.
fn refresh(users: &mut HashSet<User>, new: User) -> Option<String> {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_set_algebra_deduplicates_and_sorts() {
        assert_eq!(
            algebra(&[3, 1, 2, 2], &[4, 3]),
            (vec![1, 2, 3, 4], vec![3], vec![1, 2], vec![1, 2, 4])
        );
    }
    #[test]
    fn b_equal_values_have_equal_hashes() {
        use std::collections::hash_map::DefaultHasher;
        fn hash(u: &User) -> u64 {
            let mut h = DefaultHasher::new();
            u.hash(&mut h);
            h.finish()
        }
        assert_eq!(
            hash(&User {
                id: 1,
                display: "old".into()
            }),
            hash(&User {
                id: 1,
                display: "new".into()
            })
        );
    }
    #[test]
    fn c_replace_updates_the_representative() {
        let mut s = HashSet::new();
        assert_eq!(
            refresh(
                &mut s,
                User {
                    id: 1,
                    display: "old".into()
                }
            ),
            None
        );
        assert_eq!(
            refresh(
                &mut s,
                User {
                    id: 1,
                    display: "new".into()
                }
            ),
            Some("old".into())
        );
        assert_eq!(s.len(), 1);
        assert_eq!(s.iter().next().unwrap().display, "new");
    }
}
