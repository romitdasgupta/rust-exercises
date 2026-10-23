//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter09 c01_
//! Then: cargo clippy --test chapter09
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and some behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every unnecessary allocation, and every receiver or
//!    return type that forces callers to copy data.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. You MAY change receivers, return types, derives and impls where
//!    the doc comment says so; the tests are written to accept the better design.
//! 4. Run clippy and resolve its warnings for this file.

use std::cell::RefCell;

/// A shipment's id is trimmed and nonempty. `views` counts how often the
/// summary was rendered; it is a statistic, not part of the shipment's
/// identity. You may change the derives and add impls.
#[derive(Debug, Clone, PartialEq)]
struct Shipment {
    id: String,
    items: Vec<String>,
    grams: u32,
    views: RefCell<u32>,
}

impl Shipment {
    /// A. Store the trimmed id; None if it is empty after trimming. Keep this
    /// signature.
    fn new(id: &str, items: Vec<String>, grams: u32) -> Option<Shipment> {
        if id.is_empty() {
            return None;
        }
        Some(Shipment {
            id: id.trim().to_string().clone(),
            items: items.clone(),
            grams,
            views: RefCell::new(0),
        })
    }

    /// B. The shipment id. You may change the return type.
    fn id(&self) -> String {
        self.id.clone()
    }

    /// C. The same shipment with a new weight. You may change the receiver.
    fn with_grams(&self, grams: u32) -> Shipment {
        Shipment {
            grams,
            ..self.clone()
        }
    }

    /// D. "ID: N item(s), G g", or "ID: empty, G g" with no items. Counts one
    /// view, saturating. Keep this signature.
    fn summary(&self) -> String {
        let n = *self.views.borrow();
        *self.views.borrow_mut() = n.saturating_add(1);
        let mut out = self.id.clone();
        out = out + ": ";
        if self.items.len() == 0 {
            out = out + "empty";
        } else if self.items.len() == 1 {
            out = out + "1 item";
        } else {
            out = out + &format!("{} items", self.items.len().to_string());
        }
        out + &format!(", {} g", self.grams)
    }

    /// D. How many times the summary was rendered. Keep this signature.
    fn views(&self) -> u32 {
        *self.views.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;

    fn items(names: &[&str]) -> Vec<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn a_new_trims_before_validating() {
        let s = Shipment::new("  box-7 ", items(&["a"]), 10).unwrap();
        assert_eq!(s.id(), "box-7");
        assert!(Shipment::new("", vec![], 1).is_none());
        assert!(Shipment::new(" \t\n", vec![], 1).is_none());
    }

    #[test]
    fn a_new_moves_the_items_and_allocates_once() {
        let list = items(&["a", "b"]);
        let p = list.as_ptr();
        let (s, allocations) = count(|| Shipment::new(" box ", list, 1));
        let s = s.unwrap();
        assert_eq!(s.items.as_ptr(), p, "the item vector should move in");
        assert!(allocations <= 1, "new made {allocations} allocations");
    }

    #[test]
    fn b_id_borrows() {
        let s = Shipment::new("box-7", vec![], 1).unwrap();
        let (len, allocations) = count(|| s.id().len());
        assert_eq!(len, 5);
        assert_eq!(allocations, 0, "a getter can return a borrow");
    }

    #[test]
    fn c_with_grams_reuses_the_shipment() {
        let s = Shipment::new("box", items(&["a", "b"]), 1).unwrap();
        let p = s.items.as_ptr();
        let (heavier, allocations) = count(|| s.with_grams(900));
        assert_eq!(heavier.grams, 900);
        assert_eq!(heavier.items.as_ptr(), p, "the items should move, not copy");
        assert_eq!(allocations, 0, "with_grams made {allocations} allocations");
    }

    #[test]
    fn d_summary_formats_every_count() {
        let empty = Shipment::new("e", vec![], 0).unwrap();
        assert_eq!(empty.summary(), "e: empty, 0 g");
        let one = Shipment::new("o", items(&["x"]), 5).unwrap();
        assert_eq!(one.summary(), "o: 1 item, 5 g");
        let two = Shipment::new("t", items(&["x", "y"]), 1500).unwrap();
        assert_eq!(two.summary(), "t: 2 items, 1500 g");
        assert_eq!(two.summary(), "t: 2 items, 1500 g");
        assert_eq!(two.views(), 2);
    }

    #[test]
    fn d_views_are_not_part_of_equality() {
        let a = Shipment::new("box", items(&["a"]), 3).unwrap();
        let b = Shipment::new("box", items(&["a"]), 3).unwrap();
        assert_eq!(a, b);
        a.summary();
        assert_eq!(a.views(), 1);
        assert_eq!(a, b, "reading a shipment must not change what it equals");
        assert_ne!(a, Shipment::new("box", items(&["a"]), 4).unwrap());
        assert_ne!(a, Shipment::new("box", items(&["b"]), 3).unwrap());
    }
}
