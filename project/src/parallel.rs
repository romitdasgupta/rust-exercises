//! Milestone m19 (Chapter 19): many files on worker threads.
//!
//! The contract that makes parallelism safe to adopt: **the results are
//! exactly the sequential results**. Workers may finish in any order; the
//! code puts results back in input order and merges them in that order.

use crate::error::Error;
use crate::filter::Filter;
use crate::pipeline::Tally;
use crate::report::{Group, Summary};
use std::path::Path;

/// m19 A. `items.iter().map(f).collect()`, computed by up to `jobs` threads.
///
/// - `jobs == 0` means `std::thread::available_parallelism()` (1 if unknown).
/// - Use at most `min(jobs, items.len())` workers. With one worker (or no
///   items), run everything on the calling thread and spawn nothing.
/// - Otherwise use `std::thread::scope`, so workers can borrow `items` and
///   `f`. Hand out work dynamically: a worker takes the next unclaimed item
///   when it finishes one (an `AtomicUsize` index, or a channel of indexes).
///   Send `(index, result)` pairs back over an `mpsc` channel and put each
///   result in its slot.
/// - If `f` panics, the panic propagates to the caller (the scope does this).
pub fn map_ordered<T, R, F>(items: &[T], jobs: usize, f: F) -> Vec<R>
where
    T: Sync,
    R: Send,
    F: Fn(&T) -> R + Sync,
{
    todo!("m19 A: ordered parallel map")
}

/// m19 B. Filter and summarize several files with up to `jobs` threads.
///
/// Each file is read with [`crate::pipeline::process_reader`] into its own
/// [`Summary`]; malformed lines are only counted. Merge the per-file tallies
/// and summaries in path order. If a file cannot be opened, return
/// [`Error::Open`] for the *first* such path in path order (the other files
/// may still have been read). The result is the same for every `jobs`.
pub fn summarize_files<P>(
    paths: &[P],
    filter: &Filter,
    group: &Group,
    jobs: usize,
) -> Result<(Tally, Summary), Error>
where
    P: AsRef<Path> + Sync,
{
    todo!("m19 B: per-file work, ordered merge")
}
