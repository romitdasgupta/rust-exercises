//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter19 c01_
//! Then: cargo clippy --test chapter19
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and some behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every place a lock is held too long, and every panic
//!    that should have been an error.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. Keep both signatures; the tests depend on them.
//! 4. Run clippy and resolve its warnings for this file.

use std::{collections::BTreeMap, sync::RwLock, thread};

/// Apply `f` to every item using at most `workers` threads (treat 0 as 1),
/// returning the results in input order. If `f` panics on any item, return
/// Err("worker panicked") instead of panicking.
fn map_parallel<T, R, F>(items: &[T], workers: usize, f: F) -> Result<Vec<R>, &'static str>
where
    T: Sync,
    R: Send,
    F: Fn(&T) -> R + Sync,
{
    let f = &f;
    thread::scope(|s| {
        let mut handles = Vec::new();
        for item in items {
            handles.push(s.spawn(move || f(item)));
        }
        let mut results = Vec::new();
        while let Some(handle) = handles.pop() {
            results.push(handle.join().unwrap());
        }
        Ok(results)
    })
}

/// Fetch a fresh value for every key and make the cache hold exactly those
/// entries (keys not in `keys` are stale and must go). Fetching is slow, so
/// readers of the cache must not be blocked while `fetch` runs.
fn refresh(cache: &RwLock<BTreeMap<String, u32>>, keys: &[&str], fetch: impl Fn(&str) -> u32) {
    let mut map = cache.write().unwrap();
    for key in keys {
        let value = fetch(key);
        map.insert(key.to_string(), value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, collections::HashSet, sync::Mutex, thread::ThreadId};

    #[test]
    fn a_results_keep_input_order() {
        let items = [1, 2, 3, 4, 5];
        assert_eq!(
            map_parallel(&items, 2, |x| x * 10),
            Ok(vec![10, 20, 30, 40, 50])
        );
        assert_eq!(map_parallel(&[] as &[u8], 3, |x| *x), Ok(vec![]));
    }

    #[test]
    fn a_zero_workers_means_one() {
        assert_eq!(map_parallel(&["a", "bb"], 0, |s| s.len()), Ok(vec![1, 2]));
    }

    #[test]
    fn a_worker_panic_becomes_an_error() {
        let result = map_parallel(&[1, 2, 3], 2, |&x| {
            if x == 2 {
                panic!("fixture: bad item");
            }
            x
        });
        assert_eq!(result, Err("worker panicked"));
    }

    #[test]
    fn a_respects_the_worker_limit() {
        let items: Vec<u32> = (0..40).collect();
        let seen: Mutex<HashSet<ThreadId>> = Mutex::new(HashSet::new());
        let out = map_parallel(&items, 3, |x| {
            seen.lock().unwrap().insert(thread::current().id());
            x + 1
        })
        .unwrap();
        assert_eq!(out, (1..41).collect::<Vec<_>>());
        let threads = seen.into_inner().unwrap().len();
        assert!(
            threads <= 3,
            "40 items ran on {threads} threads; spawn at most `workers` threads"
        );
    }

    #[test]
    fn b_refresh_replaces_stale_entries() {
        let cache = RwLock::new(BTreeMap::from([
            ("old".to_string(), 1),
            ("a".to_string(), 2),
        ]));
        refresh(&cache, &["a", "b"], |key| key.len() as u32 * 100);
        let map = cache.into_inner().unwrap();
        assert_eq!(
            map,
            BTreeMap::from([("a".to_string(), 100), ("b".to_string(), 100)])
        );
    }

    #[test]
    fn b_readers_are_not_blocked_while_fetching() {
        let cache = RwLock::new(BTreeMap::new());
        let blocked = Cell::new(0);
        refresh(&cache, &["x", "y", "z"], |key| {
            if cache.try_read().is_err() {
                blocked.set(blocked.get() + 1);
            }
            key.len() as u32
        });
        assert_eq!(cache.read().unwrap().len(), 3);
        assert_eq!(
            blocked.get(),
            0,
            "the write lock was held during fetch; fetch first, then lock briefly"
        );
    }
}
