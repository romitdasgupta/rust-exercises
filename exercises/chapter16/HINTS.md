# Chapter 16 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the chapter reference and compiler diagnostics first. Open one level at a time.

## 01 · Vector access and structural editing

<details><summary>Hint 1</summary>

Choose order-preserving operations when the contract exposes sequence order.

</details>
<details><summary>Hint 2</summary>

Use split_off for ownership transfer, swap for two elements, fill for overwriting initialized elements, and get for checked access.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`len` is how many elements exist; `capacity` is how many fit before the vector must reallocate, and `capacity >= len` always. `reserve` and `with_capacity` change only capacity, never length, and guarantee *at least* the requested room, so tests must not assume an exact number. `split_off(i)` keeps elements `..i` in the original buffer and moves the tail into a newly allocated vector; ownership of each tail element moves with it. `remove` preserves order by shifting the tail (O(n)); `swap_remove` fills the hole with the last element (O(1)) and changes order. `swap` and `fill` edit in place and never change the length.

</details>

## 02 · Growth, removal, and borrow validity

<details><summary>Hint 1</summary>

Borrow splitting exposes disjoint regions with separate mutable references.

</details>
<details><summary>Hint 2</summary>

Use split_at_mut to access opposite sides. For deletion preserving order, use retain; for unordered deletion use swap_remove with a valid index.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A push *may* reallocate, and the borrow checker reasons about types, not about today's capacity, so any `&v[i]` that is still used after `v.push(..)` is rejected (E0502) even if spare capacity exists. The fixes are to copy the element, end the borrow before mutating, or keep an index and reborrow afterward. `split_at_mut` proves disjointness: it returns two non-overlapping `&mut` slices, so two mutable borrows can coexist without `unsafe`. `retain` removes in place and keeps relative order; `swap_remove` is the O(1) unordered removal.

</details>

## 03 · Sorting, binary search, and slices

<details><summary>Hint 1</summary>

Find insertion ranges with partition_point when duplicate boundaries matter.

</details>
<details><summary>Hint 2</summary>

Stable-sort by score only, so equal-score labels keep input order. Two partition_point calls find the lower and upper duplicate bounds.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A stable sort never reorders elements whose keys compare equal, so a secondary order that already exists in the input (arrival time, say) survives sorting by the primary key. An unstable sort may permute equal elements, but it needs no scratch buffer and is often faster, so prefer it when equal elements are indistinguishable or the key is total. `binary_search` on duplicates returns *some* matching index, not necessarily the first, and on unsorted input its result is meaningless. `partition_point` gives exact boundaries: the first index where the predicate turns false. `dedup` removes only *consecutive* repeats, so sort first for global uniqueness.

</details>

## 04 · Queues and ring buffers

<details><summary>Hint 1</summary>

A bounded recent-history queue evicts the oldest item before adding a new one.

</details>
<details><summary>Hint 2</summary>

Handle capacity zero first. Pop from the front while full, then push at the back. Rotate by moving the front to the back.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`VecDeque` is a ring buffer: it tracks a head index inside one allocation, so `pop_front` just moves the head instead of shifting every element like `Vec::remove(0)`. As a result the elements may wrap around the end of the buffer, and `as_slices` can return two pieces; where the split falls is not something to depend on. `make_contiguous` rotates the storage so a single slice holds every element in logical order, which is what you need before calling slice methods such as `sort`. A FIFO queue pushes at the back and pops from the front.

</details>

## 05 · Priority queues and heap ordering

<details><summary>Hint 1</summary>

BinaryHeap uses Ord; Reverse flips the comparison for a min-priority queue.

</details>
<details><summary>Hint 2</summary>

Collect into a heap and pop until empty. Wrap items in Reverse for ascending extraction and limit the count with take-like logic.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`BinaryHeap` is a max-heap that only guarantees the *top* element: `iter()` and `into_vec()` visit the internal array in heap order, which is not sorted. `pop` repeatedly (or `into_sorted_vec`, which is ascending) to get priority order. `Reverse<T>` is a wrapper whose `Ord` is flipped, so a `BinaryHeap<Reverse<T>>` pops the smallest value first without a second collection type. Tuples compare field by field, which is how a heap of `(priority, name)` breaks ties.

</details>

## 06 · Hash maps, tree maps, and entries

<details><summary>Hint 1</summary>

Entry combines lookup and update while representing occupied versus vacant cases.

</details>
<details><summary>Hint 2</summary>

Use entry(...).or_insert(0) for counting. Collect into BTreeMap for stable sorted key output and use range(start..end) for a half-open query.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Use a `BTreeMap` when the keys' order is part of the answer: range queries, sorted output, smallest/largest key. `HashMap` iteration order is unspecified and changes between runs, so tests and output must sort explicitly. `entry` does one lookup and returns `Occupied` or `Vacant`; `or_insert_with(f)` calls `f` only for a vacant entry. Because `String: Borrow<str>`, a `HashMap<String, V>` can be queried with `&str` via `get`/`contains_key`, with no temporary String allocation.

</details>

## 07 · Set algebra and equal-but-distinct values

<details><summary>Hint 1</summary>

Set membership tracks identity, not necessarily every payload field. replace changes the stored representative.

</details>
<details><summary>Hint 2</summary>

Use BTreeSet for deterministic algebra output. Hash User by id only, and use replace to retrieve the prior representative.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Hash collections assume `a == b` implies `hash(a) == hash(b)`; if `Hash` reads a field that `Eq` ignores, two equal values land in different buckets and lookups silently fail. So hash exactly the identity fields that `Eq` compares. `insert` of an equal value keeps the *existing* element and returns false; `replace` swaps in the new representative and returns the old one. Mutating an element's identity while it sits in a set (through interior mutability, for example) is a logic error: it is now stored under the wrong hash or position, and the set's behavior becomes unspecified, though not undefined.

</details>

## 08 · Hashing and custom builders

<details><summary>Hint 1</summary>

Hasher accepts a stream of bytes; a builder constructs fresh hash states for a map.

</details>
<details><summary>Hint 2</summary>

Use wrapping_add so debug and release agree. Use BuildHasherDefault<ByteSum> to connect the algorithm to HashMap.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A hash only picks a bucket; maps still compare keys with `Eq`, so a collision costs time, never correctness. A deliberately weak hasher like a byte sum makes collisions easy to construct ("ab" and "ba" collide), and an attacker who controls keys can force every insert into one bucket and turn O(1) operations into O(n): a denial-of-service. `RandomState` uses SipHash with per-map random keys to resist that, at some speed cost. `DefaultHasher` output is not stable across Rust versions, so never persist it as an identifier. `BuildHasherDefault<H>` is a zero-sized builder that creates a fresh `H::default()` for each hash.

</details>

## 09 · Capstone: a task scheduler and audit

<details><summary>Hint 1</summary>

Use a map to combine duplicate names, then a heap for scheduling; document a total tie-break.

</details>
<details><summary>Hint 2</summary>

Keep the maximum priority per name with Entry. Heap tuples order by priority then name; use Reverse for the name if lexical ascending ties are required.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Merging duplicates is a `HashMap` entry update: O(1) expected per task. A `BinaryHeap` gives O(log n) push and pop, and popping only `limit` items avoids sorting everything. Hash iteration order is unspecified, so the tie-break must be explicit in the heap's ordering: `(priority, Reverse(name))` pops the highest priority first and the lexically smallest name among equals. The output order is then fully determined by the contract, not by the hasher.

</details>

## R01 · Spaced review of Chapters 7, 10, 13–15

<details><summary>Hint 1</summary>

A map can borrow its keys from the input text, so counting words needs no per-word String. Compute a closure's key from `&item` *before* moving the item into the map. Match on the entry to tell a new key from a repeated one.

</details>
<details><summary>Hint 2</summary>

`HashMap<&str, usize>`, then collect and `sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)))`, then `truncate(n)`. For grouping, `map.entry(key(&item)).or_default().push(item)`. For the config, `lines().enumerate()`, `split_once('=')`, `parse::<u32>().map_err(...)?`, and `match map.entry(k) { Entry::Vacant(e) => ..., Entry::Occupied(_) => return Err(...) }`.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one function lets the hasher choose the output order, one returns one item too many. Most `.to_string()` and `.clone()` calls in this file are unnecessary.

</details>
<details><summary>Hint 2</summary>

Count into a `HashMap<&str, usize>` with `entry`, and add `.then(a.0.cmp(b.0))` to the sort. Track seen events in a `HashSet<&str>`, check the limit *before* pushing, and return `Vec<&str>` from `&[String]`. `users.into_iter().collect()` builds the index by moving each name.

</details>
<details><summary>Check your review</summary>

`word_counts` sorts by count only, so equal counts come out in hash iteration order, which is different on every run; it also allocates two Strings per word just to look one up (`get`/`contains_key` accept `&str`), does a double lookup that `entry` replaces, and clones every key again when collecting. `recent_unique` checks `out.len() > limit` after pushing, so it returns `limit + 1` items (and one item for a limit of 0); `out.contains` makes it O(n·k) where a `HashSet` is O(n); it clones every event and takes `&Vec<String>` where `&[String]` will do. `index_by_id` owns the vector yet clones each name; iterate by value or `collect()` directly. Clippy flags the `contains_key`/`insert` pair, the needless `to_string` calls, the hand-written `sort_by`, and the `&Vec` parameter.

</details>

## Drills

Run `cargo drill ch16`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Holding a reference across push</summary>E0502. `first` borrows the vector immutably and is used after `push`, which needs `&mut`. Spare capacity doesn't matter: the borrow checker only sees that `push` *may* reallocate. Move the push below the first assertion so the borrow ends first, or copy the value out (`let first = scores[0];`) and compare it directly.</details>
<details><summary>d02 · Updating a map through an index</summary>E0594. `HashMap` implements `Index` but not `IndexMut`, so `counts["a"]` is read-only (and would panic for a missing key). Use `*counts.entry("a").or_insert(0) += 1`, or `if let Some(n) = counts.get_mut("a") { *n += 1 }`.</details>
<details><summary>d03 · Sorting floats</summary>E0277: `f64` is only `PartialOrd`, because NaN is not comparable with anything, so `sort` (which needs `Ord`) is unavailable. Use `readings.sort_by(f64::total_cmp)`, which defines a total order that includes NaN.</details>
<details><summary>d04 · A key type without Hash</summary>E0277: `HashMap::insert` requires `K: Eq + Hash`, and `Cell` doesn't implement `Hash`. Add `Hash` to the derive; derived `Hash` and `Eq` use the same fields, so they stay consistent. (A `BTreeMap` would need `Ord` instead.)</details>
<details><summary>d05 · Two mutable indices</summary>E0499. Each `&mut slots[i]` borrows the whole vector mutably; the compiler doesn't reason about index values. Use `slots.swap(0, 2)`, or `split_at_mut` to get two disjoint `&mut` slices.</details>
<details><summary>d06 · Sorting by a borrowed key</summary>error (no code): "lifetime may not live long enough". `sort_by_key`'s key type can't borrow from the element it was computed from, because the sort moves elements around while keys are alive. Use `sort_by(|a, b| a.name.cmp(&b.name))`, or `sort_by_cached_key(|t| t.name.clone())` when the key is expensive.</details>
<details><summary>d07 · The heap's default order</summary>Panics: `BinaryHeap` is a max-heap, so `pop` returns 30. Store `Reverse(deadline)` values (the import is already there) and unwrap with `let Reverse(soonest) = deadlines.pop().unwrap();`.</details>
<details><summary>d08 · dedup on unsorted data</summary>Panics: `dedup` removes only *consecutive* duplicates, and no two equal tags are adjacent, so all four remain. Sort first (`tags.sort(); tags.dedup();`), or use a set if you need to keep first-seen order.</details>
<details><summary>d09 · contains on a Vec<String></summary>E0308: `Vec<String>::contains` takes `&String`, and `&str` doesn't coerce to it. Use `names.iter().any(|n| n == "grace")`, which compares `String` with `&str` directly.</details>
<details><summary>d10 · Looking up a String key with &str</summary>Compiles and runs. `get` and `contains_key` accept any `&Q` where `String: Borrow<Q>`, and `String: Borrow<str>`, so no temporary `String` is needed for a lookup.</details>
