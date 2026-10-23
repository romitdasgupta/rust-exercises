//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter08 c01_
//! Then: cargo clippy --test chapter08
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and most behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every invariant the module boundary fails to protect,
//!    and every unnecessary allocation or `unsafe`.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. You MAY change field visibility, return types, and how the
//!    counter is stored where the doc comments say so; the tests are written
//!    to accept the better design.
//! 4. Run clippy and resolve its warnings for this file.

mod inventory {
    /// Maximum items per bin.
    pub const CAPACITY: usize = 4;

    /// A bin that never holds more than CAPACITY items.
    /// You may change the field's visibility.
    #[derive(Debug, Default)]
    pub struct Bin {
        pub items: Vec<String>,
    }

    impl Bin {
        /// Add an item. Return false, changing nothing, when the bin is full.
        pub fn add(&mut self, item: &str) -> bool {
            if self.items.len() > CAPACITY {
                return false;
            }
            self.items.push(item.to_string());
            true
        }

        /// The items, in insertion order. You may change the return type.
        pub fn items(&self) -> Vec<String> {
            self.items.clone()
        }
    }

    static mut NEXT_ID: u32 = 0;

    /// Return a new id: 1, 2, 3, ... Never returns the same id twice, even
    /// when called from many threads. You may change how the counter is stored.
    pub fn next_id() -> u32 {
        unsafe {
            NEXT_ID += 1;
            NEXT_ID
        }
    }
}

#[cfg(test)]
mod tests {
    use super::inventory::{Bin, CAPACITY, next_id};
    use practice::alloc::count;
    use std::collections::HashSet;
    use std::thread;

    #[test]
    fn a_bin_never_exceeds_capacity() {
        let mut bin = Bin::default();
        for i in 0..CAPACITY {
            assert!(bin.add(&format!("item{i}")), "item {i} should fit");
        }
        assert!(!bin.add("one too many"));
        assert_eq!(bin.items().len(), CAPACITY);
        assert_eq!(bin.items().first().map(String::as_str), Some("item0"));
        assert_eq!(bin.items().last().map(String::as_str), Some("item3"));
    }

    #[test]
    fn a_items_borrows_instead_of_copying() {
        let mut bin = Bin::default();
        bin.add("bolt");
        bin.add("nut");
        let (len, allocations) = count(|| bin.items().len());
        assert_eq!(len, 2);
        assert_eq!(allocations, 0, "reading the items should not copy them");
    }

    #[test]
    fn b_ids_are_unique_across_threads() {
        let ids: Vec<Vec<u32>> = thread::scope(|s| {
            let workers: Vec<_> = (0..4)
                .map(|_| s.spawn(|| (0..50_000).map(|_| next_id()).collect::<Vec<u32>>()))
                .collect();
            workers.into_iter().map(|w| w.join().unwrap()).collect()
        });
        for per_thread in &ids {
            assert!(per_thread.windows(2).all(|w| w[0] < w[1]), "ids increase");
        }
        let all: HashSet<u32> = ids.iter().flatten().copied().collect();
        assert_eq!(all.len(), 200_000, "every id must be unique");
        assert_eq!(all.iter().max(), Some(&200_000));
    }
}
