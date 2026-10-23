//! LESSON 07 — Set algebra and equal-but-distinct values
//! Run: cargo test --test chapter16 l07_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: If two set values compare equal but have different metadata,
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
    use practice::guess;
    use std::cmp::Ordering;
    #[test]
    fn a_set_algebra_deduplicates_and_sorts() {
        assert_eq!(
            algebra(&[3, 1, 2, 2], &[4, 3]),
            (vec![1, 2, 3, 4], vec![3], vec![1, 2], vec![1, 2, 4])
        );
        assert_eq!(algebra(&[], &[2, 2]), (vec![2], vec![], vec![], vec![2]));
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
        assert_ne!(
            hash(&User {
                id: 1,
                display: "same".into()
            }),
            hash(&User {
                id: 2,
                display: "same".into()
            }),
            "different ids should feed different data to the hasher"
        );
    }
    #[test]
    fn b_c_set_lookup_ignores_display() {
        let mut s = HashSet::new();
        s.insert(User {
            id: 7,
            display: "a".into(),
        });
        s.insert(User {
            id: 8,
            display: "a".into(),
        });
        assert_eq!(s.len(), 2);
        assert!(s.contains(&User {
            id: 7,
            display: "other".into()
        }));
        assert_eq!(
            refresh(
                &mut s,
                User {
                    id: 9,
                    display: "new".into()
                }
            ),
            None
        );
        assert_eq!(s.len(), 3);
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

    /// Equal by `id` only, like `User`, but ordered so it fits in a BTreeSet.
    #[derive(Debug)]
    struct Tag(u32, &'static str);
    impl PartialEq for Tag {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl Eq for Tag {}
    impl PartialOrd for Tag {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Tag {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Inserting an equal-but-distinct value: the return value, and
        // which label the set keeps.
        let mut set = BTreeSet::new();
        set.insert(Tag(1, "old"));
        let inserted = set.insert(Tag(1, "new"));
        let p1: (bool, &str) = guess();
        assert_eq!(p1, (inserted, set.first().unwrap().1), "P1: insert");

        // P2. replace instead: the label it returns, and the label now stored.
        let returned = set.replace(Tag(1, "new")).map(|t| t.1);
        let p2: (Option<&str>, &str) = guess();
        assert_eq!(p2, (returned, set.first().unwrap().1), "P2: replace");

        // P3. Subset and disjointness.
        let small = BTreeSet::from([1, 2]);
        let big = BTreeSet::from([1, 2, 3]);
        let p3: (bool, bool, bool) = guess();
        assert_eq!(
            p3,
            (
                small.is_subset(&big),
                big.is_subset(&small),
                small.is_disjoint(&BTreeSet::from([3]))
            ),
            "P3: set relations"
        );
    }
}
