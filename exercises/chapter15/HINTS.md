# Chapter 15 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the chapter reference and compiler diagnostics first. Open one level at a time.

## 01 · Borrowed, mutable, owned, and draining sources

<details><summary>Hint 1</summary>

Choose an iteration source based on whether values should be borrowed, mutably borrowed, or transferred.

</details>
<details><summary>Hint 2</summary>

Use iter().copied() for values, iter_mut() for editing, and drain(range).collect() for removing owned elements.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`iter()` yields `&T`, `iter_mut()` yields `&mut T`, and `into_iter()` on a `Vec<T>` yields `T`. Each is a separate type because each holds a different kind of access: many shared borrows can coexist, a mutable borrow must be unique, and an owning iterator has taken the vector itself. So reading and editing use different iterators, and the borrow checker enforces the rules for each. `drain(range)` borrows the vector mutably and hands out the removed elements *by value*: the caller owns them, with their original heap buffers. When the Drain is dropped, the whole range is removed even if it was not fully consumed, and the tail shifts down.

</details>

## 02 · Generating finite sequences

<details><summary>Hint 1</summary>

from_fn calls a closure for each next request. successors yields a seed then repeatedly applies a function returning Option.

</details>
<details><summary>Hint 2</summary>

Use checked_mul in successors to stop at u32 overflow, then take_while for the limit. For countdown, keep Option state to include zero exactly once.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A generator ends when its closure returns `None`, so termination has to be built into the state: `successors` stops when the step returns `None`, and `from_fn` stops when its closure does. Use checked arithmetic (`checked_mul`, `checked_sub`) so the step that *would* overflow becomes `None` instead of wrapping or panicking. `take_while` then applies the limit. For the countdown, `Option<u8>` state records "nothing left" separately from the value 0, so 0 is yielded once and the iterator stays exhausted. Both generators are lazy: nothing runs until something calls `next`, and `successors` computes each next value when it yields the previous one.

</details>

## 03 · Mapping, filtering, and flattening

<details><summary>Hint 1</summary>

filter changes which items remain; map changes their shape; flat_map turns each item into zero or more items.

</details>
<details><summary>Hint 2</summary>

Parse with .ok(), filter nonnegative values, and split each line with split_whitespace. Flatten the Option layer rather than inventing placeholder values.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`filter` drops items whose predicate is false; `filter_map` drops items whose closure returns `None`, so invalid tokens vanish at the parse step and negatives vanish at the filter. `flat_map` replaces each item with zero or more items, so empty lines contribute nothing. `Option<T>` implements `IntoIterator` with zero or one item, so flattening an Option removes the `None` layer without inventing a placeholder; `Result` works the same way, with `Err` yielding nothing. Every adapter is lazy: its closure runs only when a consumer pulls an item through it.

</details>

## 04 · Stopping, peeking, and resuming

<details><summary>Hint 1</summary>

peek can examine the next item while keeping it available for a later consumer.

</details>
<details><summary>Hint 2</summary>

Use skip_while then take_while for the active region. Use peekable and next explicitly when delimiters must remain available.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Adapters like `take_while` take `self` by value, so calling them on the iterator itself would consume it. `by_ref()` returns `&mut I`, which is also an iterator, so the adapter consumes the borrow and the original iterator remains usable, at whatever position the adapter left it. `take_while` must inspect the first item that fails the predicate, so that item is consumed and lost; `peekable` lets you look at the next item without consuming it. An ordinary iterator may return `Some` again after `None`; `fuse()` guarantees that once it returns `None`, it always will.

</details>

## 05 · Reversal, pairing, inspection, and repetition

<details><summary>Hint 1</summary>

Adapter order changes semantics. zip ends at the shorter input, and cycle repeats only when asked for more items.

</details>
<details><summary>Hint 2</summary>

Enumerate before reversing to keep original indices. Zip with cycle after guarding an empty palette. Chain provides one iterator over compatible Item types.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`rev` and `rposition` need `DoubleEndedIterator`, so they can pull from the back; an unbounded range has no back, so `(1..).rev()` does not compile. `cycle` needs `Clone`, because it restarts from a saved copy of the original iterator. `enumerate` numbers the items it receives, so `enumerate().rev()` keeps original indices while `rev().enumerate()` numbers from the back. `zip` stops when either side ends. `inspect` runs a side effect for each item that actually flows past it, which makes laziness visible: a counter in `inspect` shows how far a consumer pulled.

</details>

## 06 · Accumulators, extrema, and sequence comparisons

<details><summary>Hint 1</summary>

Consumers drive the iterator to exhaustion or until their answer is known. Empty cases often follow algebraic identities.

</details>
<details><summary>Hint 2</summary>

Use explicit result types for sum/product. Choose the first shortest word via min_by_key, and compare values lexicographically with Iterator::cmp.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

With ties, more than one item is "the minimum", so the result depends on a rule the caller must be able to rely on. The standard library documents it: `min_by_key` and `min_by` return the *first* of equal elements, `max_by_key` and `max_by` the *last*. `max_by_key` compares a key extracted from each item, using `Ord` on the key; `max_by` takes a comparison closure, which lets you compare in ways no key expresses, such as `f64::total_cmp` or case-insensitive text. Empty input gives `None` for extrema, while `sum` and `product` return their identities, 0 and 1.

</details>

## 07 · Searching and iterator positions

<details><summary>Hint 1</summary>

Position methods consume inspected items. ExactSizeIterator supports translating reverse positions to forward indices.

</details>
<details><summary>Hint 2</summary>

Use position/rposition separately on fresh iterators; use find_map to stop at the first successful parse. nth and nth_back skip relative to current ends.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Indexing reads a position without changing anything. Searching methods (`position`, `find`, `nth`) *consume* every item they inspect, so later calls start from where the last one stopped: `nth(2)` twice yields the 3rd and 6th items. `rposition` needs `ExactSizeIterator` as well as `DoubleEndedIterator`, so it can convert "k items from the back" into an index from the front; reporting original indices means the result can be used to index the slice. `find_map` combines search and transformation, stopping at the first `Some`.

</details>

## 08 · Folds, short circuits, and side effects

<details><summary>Hint 1</summary>

fold threads an accumulator through each item; try_fold additionally permits early return through Result or Option.

</details>
<details><summary>Hint 2</summary>

Use checked_add with try_fold for overflow. For audit, push before the predicate so the rejected input is included, then stop with Err.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A fallible consumer stops at the first failure because the answer is already known and later work is wasted, or harmful when the closure has side effects. `try_fold` and `try_for_each` return the first `Err` or `None`, and they consume the failing item but leave the rest in the iterator. `fold` combines from the front, `rfold` from the back; for non-commutative operations such as digit building or string nesting, the direction changes the result. `try_rfold` short-circuits from the back, so the items nearest the right end run first.

</details>

## 09 · Collections, extension, and partition

<details><summary>Hint 1</summary>

The destination type chooses FromIterator. Extend modifies an existing collection instead of creating a new one.

</details>
<details><summary>Hint 2</summary>

Use Result collection for all-or-nothing parsing. Partition copies into even/odd vectors. Extend the existing vector with the supplied iterator.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`collect` creates a new collection through `FromIterator`, and the destination type (named in an annotation or the turbofish) decides which one: Vec, String, BTreeSet, or `Result<Vec<_>, E>`, which stops at the first `Err`. `extend` adds to a collection that already exists through `Extend`, keeping its contents, and can reserve space from the source's `size_hint`. `partition` sends each item to one of two collections by a predicate, so both outputs must be the same type, and each keeps the input order.

</details>

## 10 · Implementing a double-ended iterator

<details><summary>Hint 1</summary>

Use a half-open interval [front,back), where back is excluded and length is back-front.

</details>
<details><summary>Hint 2</summary>

For next, return front then increment. For next_back, decrement back then return it. Both first check front<back.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`size_hint` may be loose: `(0, None)` is always a valid hint, so callers can use it only for optimizations such as pre-allocation. `ExactSizeIterator` promises the exact count, and `len()` and `rposition` rely on that, so it must stay exact after every `next` and `next_back`. A half-open interval `[front, back)` makes both ends share one remaining range: the iterator is empty exactly when `front == back`, and the two ends cannot yield the same value twice. Implementing `FusedIterator` promises that after `None`, every later call also returns `None`; callers and adapters such as `fuse` may rely on it.

</details>

## 11 · Capstone: a lazy measurement report

<details><summary>Hint 1</summary>

Compose one pipeline whose stages match the contract in order.

</details>
<details><summary>Hint 2</summary>

Enumerate original lines before filtering so diagnostic numbers stay original. Parse with filter_map; compute an average only after checking for empty input.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`enumerate` numbers the items it receives, so it must come first, while every source line is still present; numbering after filtering would number the survivors instead. The rest of the pipeline only describes work: trim, skip comments, parse with `filter_map`, keep values at or above the minimum, then `take(limit)`. Nothing runs until `collect` starts pulling, and `take` stops pulling after `limit` rows, so later lines are never parsed. `report` then runs its consumers over the collected rows: `sum` over widened i64 values, and the mean only when there is at least one row.

</details>

## R01 · Spaced review of Chapters 10–14

<details><summary>Hint 1</summary>

A `move` closure owns its captured `best`, and `filter` calls it once per item, so it can update that state on every call. `TryFrom` returns `Result`, and collecting an iterator of Results into `Result<Vec<_>, _>` stops at the first Err. `?` works inside a `try_fold` closure that returns Option.

</details>
<details><summary>Hint 2</summary>

Start `best` as `None`, not `i32::MIN`, or a leading `i32::MIN` would be rejected; `Option::is_none_or` reads well here. `(0..=100).contains(&value)` checks the range before the `as u8` cast. For eval: `try_fold(Vec::new(), |mut stack, op| { match *op { ... } Some(stack) })`, pop with `stack.pop()?`, then accept only `[v]` with a slice pattern.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one function invents a result for empty input, one indexes past the end of a slice. The other problems are intermediate collections: two functions build a Vec they don't need.

</details>
<details><summary>Hint 2</summary>

`filter_map(|t| t.trim().parse().ok())` replaces filter + unwrap; fold a `(sum, count)` pair and return `None` when the count is 0. `zip` stops at the shorter slice. For distinct words, `HashSet::insert` returns false for duplicates, `take(n)` stops early, and returning `Vec<&str>` avoids cloning.

</details>
<details><summary>Check your review</summary>

`average` returns `Some(NaN)` for empty input (0.0 / 0.0) instead of None, filters on `is_ok()` and then `unwrap`s (clippy suggests `filter_map`), and collects a Vec only to sum it and take its length; one fold over `(sum, count)` needs no allocation. `first_distinct` clones the whole input, scans `seen` linearly for every word (quadratic), and reads all the words even when the first `n` are already known; it can take `&[String]`, return borrowed `&str`s, and stop with `take(n)`. `scoreboard` indexes `scores[i]` for every name and panics when scores are shorter; `names.iter().zip(scores).map(...).collect()` follows the contract and allocates the output once.

</details>

## Drills

Run `cargo drill ch15`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · An adapter nobody consumes</summary>Panics at the assertion: `map` only builds a lazy adapter, and nothing pulls items through it, so the closure never runs (rustc warns that the iterator is unused). Use a consumer: `for_each`, or a plain `for` loop for side effects.</details>
<details><summary>d02 · Looping over a vector by value</summary>E0382. `for name in names` calls `names.into_iter()`, which consumes the vector. Loop over `&names` to borrow it.</details>
<details><summary>d03 · collect without a destination</summary>E0283. `collect` can build any `FromIterator` type, and nothing here says which one. Annotate the binding (`let doubled: Vec<i32>`) or use the turbofish, `collect::<Vec<_>>()`.</details>
<details><summary>d04 · What filter's closure receives</summary>E0308. `iter()` yields `&i32`, and `filter` passes a reference to each item, so `s` is `&&i32` and cannot be compared with an integer. Dereference (`**s > 5`) or destructure in the pattern: `|&&s| s > 5`.</details>
<details><summary>d05 · Reversing an endless range</summary>E0277. `rev` needs `DoubleEndedIterator`, and `1..` has no last element to start from. Give the range an end: `(1..=10).rev()`.</details>
<details><summary>d06 · The length of a chain</summary>E0599. `Chain` does not implement `ExactSizeIterator`, because the sum of two lengths could overflow `usize`, so there is no `len()`. Use `count()`, which consumes the iterator, or `size_hint()`, which here is `(5, Some(5))`.</details>
<details><summary>d07 · Growing a vector while iterating it</summary>E0502. The loop holds a shared borrow of `queue` for its whole duration, and `push` needs a mutable one. Pushing could reallocate the buffer the iterator points into. Decide first, then push after the loop (or collect the additions and `extend` afterward).</details>
<details><summary>d08 · Collecting borrowed items into owned ones</summary>E0277. `iter()` over a `Vec<String>` yields `&String`, and a `Vec<String>` cannot be built from references. Use `.cloned()` to get owned Strings, or collect into `Vec<&String>` if borrowing is enough.</details>
<details><summary>d09 · Resuming after by_ref</summary>Compiles. `by_ref()` lends the iterator to `take(1)`, so `lines` is still usable afterward and continues from the second item.</details>
<details><summary>d10 · A computed step of zero</summary>Panics: integer division gives `3 / 4 == 0`, and `step_by(0)` panics because the iterator would never advance. Guard the computed value, for example `(values.len() / 4).max(1)`.</details>
