# Chapter 5 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the first hint before opening the second. The implementation constraints in each task are part of the exercise.

## 01 · Borrowing and dereferencing

<details><summary>Hint 1</summary>

A shared borrow reads; an exclusive borrow can write its referent.

</details>
<details><summary>Hint 2</summary>

Use len on the borrowed value; use *slot on the left side of assignment.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

The owner (the `String` or integer binding in the caller) keeps its storage; a reference only points at it, so passing `&text` neither moves nor copies anything, and the owner is usable after the call. The `.` operator dereferences automatically as many times as needed to find a method or field, which is why `r.len()` works on `&&&Vec`. Assignment has no such rule: `slot = value` would rebind the local reference, so writing to the referent needs an explicit `*slot = value`. Arithmetic operators like `+` also see through one level of reference, but `*` makes the target explicit.

</details>

## 02 · Reference bindings and identity

<details><summary>Hint 1</summary>

Reference equality usually compares values; pointer identity is a separate question.

</details>
<details><summary>Hint 2</summary>

Reassign a mutable &i32 binding for selection. Use std::ptr::eq for identity and == for content.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`let mut r = &x` makes the *binding* mutable: `r` can later point elsewhere, but neither integer can be changed through it. `&mut x` is the opposite: the referent can change, and the binding may be immutable. With a reference to a reference (`&mut &i32`), `*rr = &y` changes where the inner reference points. `==` on references compares the values they point at; `std::ptr::eq` compares addresses (and, for slices, lengths). Two different String objects are never the same object even when their buffers happen to compare equal, for example two empty Strings that have no heap buffer at all.

</details>

## 03 · Optional references and fat pointers

<details><summary>Hint 1</summary>

Checked access naturally represents a missing borrow as None.

</details>
<details><summary>Hint 2</summary>

Use get to return Option<&T>; return a borrowed prefix and compute length from the slice metadata.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

References are never null, so absence is `Option<&T>`; because the null bit pattern is unused, `Option<&T>` is the same size as `&T`. `&[T]` and `&str` are fat pointers: an address plus a length, two machine words, so a subslice can describe any region without copying. Borrowing a temporary in a `let` initializer, as in `let r = &format!(..)`, extends that temporary's life to the end of the block; returning the borrow from a function is still rejected, because the temporary would die with the function. A borrow never owns its referent, so dropping the reference drops nothing.

</details>

## 04 · Lifetimes through functions

<details><summary>Hint 1</summary>

Return a subslice of input, whose owner is controlled by the caller.

</details>
<details><summary>Hint 2</summary>

split_once can return two borrows of the original str. A maximum scan can keep a borrowed candidate.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

In `fn greatest<'a>(values: &'a [i32]) -> Option<&'a i32>`, the signature promises that the result points into `values`, so the caller may use the result only while that slice is still borrowed. The annotation describes a relationship the compiler checks; it allocates nothing and cannot make an owner live longer. A function cannot return a reference to its own local, because the local is dropped at the end of the call; returning a borrow of an input or an owned value are the two fixes. Adding `'static` where it isn't needed only forbids callers from passing ordinary borrowed data.

</details>

## 05 · Structs that borrow

<details><summary>Hint 1</summary>

The view stores references into one source; it does not copy text.

</details>
<details><summary>Hint 2</summary>

Use split_once and store its borrowed parts with the same lifetime. Return a field directly.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A struct that stores references needs a lifetime parameter, because the compiler must know how long the referents live in order to prevent a dangling field. `Record<'a>` says both fields borrow data that outlives `'a`; every `Record` built from one line is tied to that line's owner, so the owner cannot be dropped or mutated while the record is used. Moving the record copies two fat pointers and leaves the source alone. A function like `value<'a>(record: &Record<'a>) -> &'a str` returns the field with the *source's* lifetime, so the result can outlive the short borrow of the wrapper.

</details>

## 06 · Independent lifetimes and elision

<details><summary>Hint 1</summary>

Use separate lifetime parameters when output depends on only one independently borrowed input.

</details>
<details><summary>Hint 2</summary>

Give left and right separate lifetimes. The output of left_text needs only the left lifetime.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Elision rules: each reference parameter gets its own lifetime; if there is exactly one input lifetime, it is used for every elided output; if a method takes `&self` or `&mut self`, outputs borrow from `self`. Otherwise an output reference must be annotated. When a struct's fields borrow different owners, separate parameters (`Pair<'left, 'right>`) let the compiler track them independently, so `left_text` can return a `&'left str` that stays valid after the right-hand owner is dropped. With a single shared lifetime, the result would be limited by the shorter-lived owner. Lifetimes exist only during type checking; at run time a `Pair` is just two fat pointers.

</details>

## 07 · Sharing versus mutation

<details><summary>Hint 1</summary>

End read access before requesting exclusive mutation.

</details>
<details><summary>Hint 2</summary>

Calculate values before the mutation, then update via &mut. Keep no element borrow alive across push.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A borrow lasts until its last use, not to the end of the block, so you can read a length, stop using the reference, and then mutate. While a shared borrow of an element is alive, the vector cannot be mutated: a push may reallocate the buffer and leave the reference dangling. The compiler enforces this without looking at the run-time capacity, because the rule must hold for every execution, not only the ones where capacity happens to suffice. `&mut` means exclusive: no other reference to the same data can be used while it is live. A reborrow `&mut *r` temporarily suspends `r`, which becomes usable again after the reborrow's last use.

</details>

## 08 · Disjoint mutable borrows

<details><summary>Hint 1</summary>

Use a library operation whose returned references describe nonoverlapping storage.

</details>
<details><summary>Hint 2</summary>

Check bounds and equality first, split at the larger index, and update one element from each half.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`split_at_mut(mid)` returns `(&mut [..mid], &mut [mid..])`, two mutable slices whose types promise they never overlap, so the borrow checker can let both live at once. Its signature encodes that both halves borrow the original slice for the same lifetime, so the original is unusable until both halves are done. The library implements it with a small amount of `unsafe` and a bounds check (it panics if mid > len), and the safe signature means callers need no `unsafe`. The compiler cannot prove `values[0]` and `values[1]` disjoint because indices are run-time values, but it can for distinct struct or tuple fields, which are separate places.

</details>

## 09 · Capstone: borrowed roster queries

<details><summary>Hint 1</summary>

A returned borrow ties up the relevant owner; an index carries no borrow but can become stale.

</details>
<details><summary>Hint 2</summary>

Scan indices and keep the first longest match. Return a subslice from the chosen String only while borrowing the roster.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Returning a borrow (`Option<&str>`) is cheap and always valid, but it keeps the roster borrowed, so the caller cannot edit it while holding the result. Returning an index carries no borrow, so the roster can change, but the index silently goes stale when elements are removed, inserted, or sorted, and the caller must re-check it. Cloning an owner gives the caller independent data at the cost of an allocation and of not seeing later edits. A stable ID stored inside each entry survives reordering; looking it up is slower than indexing, but it never points at the wrong element.

</details>

## R01 · Spaced review of Chapters 3 and 4

<details><summary>Hint 1</summary>

Find the index first, through a shared borrow; once you have a plain `usize`, the borrow is over and you may call a method that moves the String out while keeping order. For the counters, decide the overflow policy per element with a method that *reports* overflow. For truncation, a byte budget can land inside a scalar.

</details>
<details><summary>Hint 2</summary>

Track the best index and its `chars().count()` in a loop, replacing it only on a strictly greater count; then `names.remove(i)`. `counter.checked_add(amount)` returns `None` exactly when the sum does not fit: store 255 and count it. For truncation, start at `max_bytes.min(text.len())` and step down until `text.is_char_boundary(end)`, then return `&text[..end]`.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one function panics on input it is documented to skip, and one matches more labels than it should. Every `.clone()` in this file is unnecessary, including the one that "works around" the borrow checker.

</details>
<details><summary>Hint 2</summary>

`config_value` can take `&[String]` and `&str` and return `Option<&str>` borrowed from the lines; use `if let Some((k, v)) = line.split_once('=')`. Rename with `for label in labels.iter_mut()`, compare with `==`, and reuse the buffer with `label.clear(); label.push_str(new)`. For `bump_to_end`, find the index with `position`, which ends the borrow, then `remove` and `push` the same String.

</details>
<details><summary>Check your review</summary>

`config_value` unwraps `split_once`, so any comment or blank line before the match panics; it also clones every line, demands a `&String` key where any `&str` would do, and copies the value it could borrow. `rename_all` uses `contains`, so "older" and "bold" are renamed too; it clones each label just to compare it and allocates a new String per match. `bump_to_end` behaves correctly but clones the whole vector to avoid holding a borrow while mutating; computing the index first ends the borrow, so no snapshot is needed. Clippy also flags the `&Vec<String>` and `&String` parameters and the index loop.

</details>

## Drills

Run `cargo drill ch05`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Writing through a shared reference</summary>E0596: `&String` promises the owner that nothing changes while it is borrowed. Take `text: &mut String`; the caller already passes `&mut greeting`.</details>
<details><summary>d02 · Returning a reference to a local</summary>E0515: `upper` is dropped when `shout` returns, so a reference to it would dangle. Return the owned `String` instead; `String == &str` comparisons still work.</details>
<details><summary>d03 · Holding a borrow across a push</summary>E0502: `push` needs `&mut scores` while `first` (a shared borrow) is still used, and a push can reallocate the buffer `first` points into. Read `*first` before pushing, or copy the value out with `let first = scores[0];`.</details>
<details><summary>d04 · Two mutable borrows into one vector</summary>E0499: indexing borrows the whole vector, and the compiler cannot prove that two run-time indices differ. Use `pair.swap(0, 1)`, or `split_at_mut(1)` to get two provably disjoint halves.</details>
<details><summary>d05 · A borrow ends at its last use</summary>Compiles. `view` is last used on the `len()` line, so the shared borrow is over before `push_str` (non-lexical lifetimes).</details>
<details><summary>d06 · A reference that outlives its owner</summary>E0597: `name` is dropped at the end of the inner block while `label` is still used afterward. Declare the owner in the outer scope so it lives as long as the reference.</details>
<details><summary>d07 · Which input does the result borrow?</summary>E0106: with two reference parameters, elision cannot choose the output's lifetime. Write `fn longer<'a>(a: &'a str, b: &'a str) -> &'a str`: the result is valid while both inputs are.</details>
<details><summary>d08 · Moving an owner while it is borrowed</summary>E0505: `consume(source)` moves the String (and may free it) while `view` still points into it. Finish using the view before the move, or pass `&source` to a function that only borrows.</details>
<details><summary>d09 · Assigning to a borrowed value</summary>E0506: assigning to `limit` while `r` is still used would change what a shared reference sees. Use `r` before the assignment, or copy the value out with `let old = limit;`.</details>
<details><summary>d10 · Splitting past the end</summary>Panics at run time: `split_at_mut` checks `mid <= len` when it runs, because `mid` is a run-time value. Valid split points are `0..=len`; here `values.len()` gives an empty right half.</details>
