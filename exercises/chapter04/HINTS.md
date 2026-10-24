# Chapter 4 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the first hint before opening the second. The implementation constraints in each task are part of the exercise.

## 01 · Ownership scopes

<details><summary>Hint 1</summary>

A String owns its buffer; a vector owns its elements. The returned owner can survive the inner block.

</details>
<details><summary>Hint 2</summary>

Build the vector in a block and return its final expression; moving a String does not clone its bytes.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A `Vec<String>` owns a heap buffer of String headers, and each header owns its own UTF-8 buffer: a tree with the binding at the root. When the root's owner goes out of scope, Rust drops it, and dropping a vector drops its elements in order, then frees the buffer. Locals are dropped in reverse order of declaration. A move transfers the root without touching the heap: only the header (pointer, capacity, length) is copied, and the old binding becomes unusable, so the tree still has exactly one owner.

</details>

## 02 · Moves across assignments and fields

<details><summary>Hint 1</summary>

Ownership can move through function parameters, results, assignments, and collection insertion.

</details>
<details><summary>Hint 2</summary>

Destructure the pair into owned bindings. Push consumes its argument; returning a vector moves its ownership.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Moving a field out of a tuple or struct is a *partial move*. The moved field is uninitialized; the other fields remain owned and usable individually, but the whole value can no longer be used, because part of it is missing. Destructuring with `let (a, b) = pair;` moves every field into new bindings at once, so nothing is left behind. `_` in a pattern does not bind, so it does not move that field.

</details>

## 03 · Moves and control flow

<details><summary>Hint 1</summary>

Every path reaching a use needs an initialized owner.

</details>
<details><summary>Hint 2</summary>

Move through a helper and assign its return value back. The selected if branch can return the chosen String.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

The compiler checks that a variable is definitely initialized on every path that reaches a use. A move inside one `if` branch makes the variable *maybe-moved* afterward, so later uses are rejected. A move inside a loop body would move the value on the second iteration again, so it is rejected unless the loop reassigns the variable before the next iteration. At run time, drop flags remember which conditionally moved variables still own something, which is why the drop log in P1/P2 differs by branch.

</details>

## 04 · Moving from indexed collections

<details><summary>Hint 1</summary>

Use methods that maintain a valid collection after ownership leaves it.

</details>
<details><summary>Hint 2</summary>

Guard invalid indices before remove or swap_remove; pop already returns Option.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`let x = v[i];` would leave a hole in the vector: a slot that the vector still believes is initialized. Rust forbids it (E0507) for non-Copy types. Methods that move out also repair the collection: `pop` shortens it, `remove` shifts the tail left (O(n), order kept), `swap_remove` moves the last element into the hole (O(1), order changed). All of them return the element by value, so no clone is needed.

</details>

## 05 · Leaving a replacement behind

<details><summary>Hint 1</summary>

An owned extraction through a mutable reference must install a valid replacement.

</details>
<details><summary>Hint 2</summary>

std::mem::replace returns the old value. std::mem::take installs Default; Option::take leaves None.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Through `&mut T` you cannot leave the referent uninitialized, because the owner will still use and drop it. `mem::replace` swaps in a value you supply and returns the old one; `mem::take` does the same using `Default::default()` (an empty String with no allocation); `Option::take` leaves `None`. The caller always observes a valid value, and the old value's buffer moves out intact.

</details>

## 06 · Copy and explicit cloning

<details><summary>Hint 1</summary>

Copy types have no ownership-bearing resources that need a unique destructor.

</details>
<details><summary>Hint 2</summary>

Use assignment for the integer array. Clone the String, then modify only the clone.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Copy types are plain bits with no owned resources, so assignment duplicates them and the source stays usable. Copy is opt-in and requires every field to be Copy; a type with a `Drop` impl or a `String` field cannot be Copy, because two bitwise copies would both free the same buffer. `clone()` is explicit and may be expensive: cloning a `Vec<String>` allocates a new buffer and a new buffer for every string.

</details>

## 07 · Rc and Arc

<details><summary>Hint 1</summary>

Clone the pointer to share its referent; strong_count reports the number of owners.

</details>
<details><summary>Hint 2</summary>

Use Rc::clone and Arc::clone. Join the Arc-reading thread so the count after it ends is predictable.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Rc::clone` increments a strong count and returns another pointer to the same allocation; it does not copy the value. The value is dropped when the last strong owner is dropped. `Arc` uses atomic increments so counts stay correct across threads, which costs more per clone; `Rc` is not `Send`. Both give shared *read* access only. Two Rcs that point at each other never reach a zero count, so they leak; break cycles with `Weak`.

</details>

## 08 · Capstone: owned dispatch queue

<details><summary>Hint 1</summary>

Separate immutable shared configuration from owned editable labels.

</details>
<details><summary>Hint 2</summary>

Create one Rc, clone it for each tuple, and move each String from the input vector through a for loop.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

After dispatch, each job tuple owns its label String uniquely (moved out of the input vector, no clone) and holds one strong pointer to a single shared `Rc<String>` config allocation. The count equals the number of live jobs. Sharing the config is safe because nobody mutates it; labels are mutated independently, so they must have unique owners. `finish` consumes a job: it moves the label out, appends the config, and dropping the job's Rc decrements the count.

</details>

## R01 · Spaced review of Chapter 3

<details><summary>Hint 1</summary>

Borrow the vector to compute the total, then return the same vector: a borrow ends before the move. Strings are walked by `chars()`, never by byte index.

</details>
<details><summary>Hint 2</summary>

`try_fold` with `checked_add` gives `Option<u16>`. `chars().take(n).collect()` builds the prefix. `len().saturating_sub(count)` plus `Vec::split_off` moves the tail without cloning.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one counts the wrong unit, one leaves a separator behind. Every `.clone()` in this file is unnecessary.

</details>
<details><summary>Hint 2</summary>

Return `Option<&str>` borrowed from `&[String]`. `[String]::join` sizes its buffer once. A stable sort keyed on "is not urgent" reorders by moving Strings, not copying them.

</details>
<details><summary>Check your review</summary>

`longest` compares byte lengths instead of scalar counts, clones the whole vector, then clones each winner; it can borrow instead. `join_labels` leaves a trailing ", " and rebuilds the string on every iteration (quadratic copying). `urgent_first` clones every job twice when it could reorder in place. Clippy also flags `&Vec<String>` and `&mut Vec<String>` parameters: accept slices when you never change the length.

</details>

## Drills

Run `cargo drill ch04`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Use after move</summary>E0382. `let saved = route;` moves the String; `route` is uninitialized afterward. Fix with `route.clone()` (two buffers) or `let saved = &route;` (one buffer, borrowed).</details>
<details><summary>d02 · Moving out of an index</summary>E0507. Indexing yields a place inside the vector; moving from it would leave a hole. Borrow (`&names[0]`), clone, or use a method that repairs the vector (`remove`, `swap_remove`).</details>
<details><summary>d03 · Reassigning a moved binding</summary>Compiles. A moved-from variable is uninitialized, not dead; assigning a new value makes it usable again.</details>
<details><summary>d04 · Moves inside a loop</summary>E0382: the second iteration would move an already-moved String. Clone inside the loop, or build each value inside the loop.</details>
<details><summary>d05 · Partial moves</summary>Compiles. `pair.0` is moved, `pair.1` is still owned. Using `pair` as a whole afterward would be E0382.</details>
<details><summary>d06 · Copy is all-or-nothing</summary>E0204: String is not Copy, so the struct cannot be. Remove `Copy` from the derive; keep `Clone`.</details>
<details><summary>d07 · Rc is shared, not mutable</summary>E0596: `Rc` only hands out shared references, because other owners may be reading. One fix is `Rc::make_mut(&mut config)`: it clones the value first when it is shared (clone-on-write), so `other` still sees the old text. Interior mutability with RefCell comes in Chapter 9.</details>
<details><summary>d08 · Removing past the end</summary>Panics at run time: an index is a run-time value, so the compiler cannot reject it. Use index 0, or check with `if index < queue.len()`.</details>
<details><summary>d09 · Rc across threads</summary>E0277: `Rc<Vec<i32>>` is not `Send`, because its non-atomic count could be corrupted by two threads. Replace Rc with `std::sync::Arc`.</details>
