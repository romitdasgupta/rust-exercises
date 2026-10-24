//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter20 c01_
//! Then: cargo clippy --test chapter20
//!
//! This is the kind of code you will be asked to review at work. It compiles
//! (with a warning worth reading), and some behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every await that holds something it should not, and every
//!    place where cancellation could leave state half-updated.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. Keep all three signatures; the tests depend on them.
//! 4. Run clippy and resolve its warnings for this file.

use std::{cell::RefCell, collections::BTreeMap, future::Future, sync::Mutex};

/// A. Return the cached value for `key`. On a miss, await `load`, cache its
/// result, and return it. Other tasks polled while `load` is pending must be
/// able to use the cache.
async fn get_or_load(
    cache: &Mutex<BTreeMap<String, String>>,
    key: &str,
    load: impl Future<Output = String>,
) -> String {
    let mut map = cache.lock().unwrap();
    if let Some(value) = map.get(key) {
        return value.clone();
    }
    let value = load.await;
    map.insert(key.to_string(), value.clone());
    value
}

/// B. Move `amount` from `from` to `to` once `approve` resolves to true, and
/// return whether money moved. A rejected transfer, or one whose future is
/// dropped before approval, changes nothing. Inputs guarantee `*from >= amount`
/// and no overflow.
async fn transfer(
    from: &mut u64,
    to: &mut u64,
    amount: u64,
    approve: impl Future<Output = bool>,
) -> bool {
    *from -= amount;
    if approve.await {
        *to += amount;
        true
    } else {
        false
    }
}

async fn save(store: &RefCell<Vec<String>>, item: &str) {
    store.borrow_mut().push(item.to_owned());
}

/// C. Save every item, in order, and return how many were saved.
async fn save_all(store: &RefCell<Vec<String>>, items: &[&str]) -> usize {
    let mut saved = 0;
    for item in items {
        save(store, item);
        saved += 1;
    }
    saved
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        pin::{Pin, pin},
        task::{Context, Poll, Waker},
    };

    fn ready<F: Future>(future: F) -> F::Output {
        let mut f = pin!(future);
        match f.as_mut().poll(&mut Context::from_waker(Waker::noop())) {
            Poll::Ready(x) => x,
            Poll::Pending => panic!("expected immediately ready future"),
        }
    }

    /// Returns Pending once, then Ready.
    struct YieldOnce(bool);
    impl Future for YieldOnce {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if self.0 {
                return Poll::Ready(());
            }
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }

    fn cache_of(entries: &[(&str, &str)]) -> Mutex<BTreeMap<String, String>> {
        Mutex::new(
            entries
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        )
    }

    #[test]
    fn a_hit_returns_the_cached_value() {
        let cache = cache_of(&[("k", "cached")]);
        let value = ready(get_or_load(&cache, "k", async { "loaded".to_string() }));
        assert_eq!(value, "cached");
    }

    #[test]
    fn a_miss_loads_and_caches() {
        let cache = cache_of(&[]);
        let value = ready(get_or_load(&cache, "k", async { "loaded".to_string() }));
        assert_eq!(value, "loaded");
        assert_eq!(
            cache.lock().unwrap().get("k").map(String::as_str),
            Some("loaded")
        );
    }

    #[test]
    fn a_cache_is_unlocked_while_loading() {
        let cache = cache_of(&[("other", "x")]);
        let mut f = pin!(get_or_load(&cache, "k", async {
            YieldOnce(false).await;
            "slow".to_string()
        }));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(f.as_mut().poll(&mut cx).is_pending());
        assert!(
            cache.try_lock().is_ok(),
            "the lock was held across `.await`: every other task would block"
        );
        assert_eq!(f.as_mut().poll(&mut cx), Poll::Ready("slow".to_string()));
        assert_eq!(cache.lock().unwrap().len(), 2);
    }

    #[test]
    fn b_approved_transfer_moves_money() {
        let (mut a, mut b) = (100, 5);
        assert!(ready(transfer(&mut a, &mut b, 30, async { true })));
        assert_eq!((a, b), (70, 35));
    }

    #[test]
    fn b_rejected_transfer_changes_nothing() {
        let (mut a, mut b) = (100, 5);
        assert!(!ready(transfer(&mut a, &mut b, 30, async { false })));
        assert_eq!((a, b), (100, 5));
    }

    #[test]
    fn b_cancelled_transfer_changes_nothing() {
        let (mut a, mut b) = (100, 5);
        {
            let mut f = Box::pin(transfer(&mut a, &mut b, 30, std::future::pending()));
            assert!(
                f.as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
        }
        assert_eq!((a, b), (100, 5), "dropping a pending transfer lost money");
    }

    #[test]
    fn c_every_item_is_saved_in_order() {
        let store = RefCell::new(Vec::new());
        assert_eq!(ready(save_all(&store, &["a", "b", "c"])), 3);
        assert_eq!(*store.borrow(), ["a", "b", "c"]);
        assert_eq!(ready(save_all(&store, &[])), 0);
    }
}
