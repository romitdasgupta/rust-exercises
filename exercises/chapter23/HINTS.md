# Chapter 23 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the first hint before opening the second. The implementation constraints in each task are part of the exercise.

## l01 · A safe invariant boundary

<details><summary>Hint 1</summary>

Unsafe code relies on an invariant that safe callers cannot break through the public API.

</details>

<details><summary>Hint 2</summary>

Validate with is_ascii; borrow with from_utf8_unchecked only after explaining why every stored byte is guaranteed ASCII.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

The private invariant is that every byte in `Ascii.0` is at most 0x7f. `new` establishes it by checking `is_ascii()` before constructing, and gives the caller's Vec back unchanged if the check fails. The field is private, so code outside the module cannot break it. `lowercase` keeps it because ASCII lowercasing maps ASCII to ASCII. The only unsafe operation is `str::from_utf8_unchecked` in `as_str`: its precondition, valid UTF-8, follows from the invariant because every ASCII sequence is valid UTF-8, including `\0`. `unsafe` does not switch off the borrow checker. It only permits the few extra operations listed in the book, so the returned `&str` still borrows `self` as usual.

</details>

## l02 · Raw pointer reads with contracts

<details><summary>Hint 1</summary>

The unsafe function cannot validate arbitrary pointer provenance. Its safe caller should derive the pointer from a valid borrow.

</details>

<details><summary>Hint 2</summary>

Bounds-check before add, then place only the read in a justified unsafe block. Copying u32 does not transfer allocation ownership.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

`read_u32`'s caller must guarantee that the pointer is non-null, aligned for u32, and points to an initialized u32 inside a live allocation it may read from, with no conflicting writer during the read. Non-null alone proves nothing: a dangling pointer is non-null. `at` discharges every obligation. It checks `index < values.len()` first, so `values.as_ptr().add(index)` stays in bounds of the slice it came from (provenance). Slice elements are aligned and initialized. The shared borrow `&[u32]` keeps the allocation alive and excludes writers for the duration of the call. Creating and offsetting a raw pointer is safe; only reading through it is unsafe.

</details>

## l03 · Raw slice reconstruction

<details><summary>Hint 1</summary>

The safety argument concerns reference creation, not just later use. Empty regions still need valid aligned non-null pointers.

</details>

<details><summary>Hint 2</summary>

Get len and as_mut_ptr before constructing either slice. ptr::swap permits identical pointers, unlike copy_nonoverlapping-style assumptions.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

Bounds: `mid <= len` is checked, so `[0, mid)` and `[mid, len)` lie inside the slice. Provenance: both pointers derive from `values.as_mut_ptr()`, so they may access the whole slice. Alignment and initialization follow from the original `&mut [T]`. Lifetime: both outputs borrow from the input's unique borrow, which the signature ties to the result. Non-overlap: the two ranges are disjoint, so the two `&mut` slices never alias. Merely *creating* overlapping `&mut` references is already undefined behavior, even if neither is used. A zero-length slice still needs a non-null, aligned pointer, which `as_mut_ptr().add(len)` provides. `ptr::swap` explicitly allows both pointers to be equal; `copy_nonoverlapping` does not.

</details>

## l04 · A flagged borrow

<details><summary>Hint 1</summary>

Keep the original pointer provenance rather than converting to an integer and inventing a new pointer.

</details>

<details><summary>Hint 2</summary>

map_addr transforms only address bits. Mask the tag before dereferencing; PhantomData makes the lifetime relation visible to borrow checking.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

A raw pointer carries no lifetime, so `Flagged` stores `PhantomData<&'a u32>`. To the borrow checker the struct holds a `&'a u32`, so it cannot outlive the borrowed value, and it costs zero bytes. u32 is 4-aligned, so the two low address bits of a valid pointer are always zero, and one of them can hold the flag. `map_addr` changes only the address bits and keeps the pointer's provenance, so after masking the tag you have the *same* pointer again, valid to dereference. Casting to `usize` and back with `as` would create a pointer without provenance under strict provenance, and clearing the tag later cannot restore it.

</details>

## l05 · Layout, unsafe traits, unions

<details><summary>Hint 1</summary>

An unsafe trait puts obligations on every unsafe impl; generic unsafe code may rely on those obligations.

</details>

<details><summary>Hint 2</summary>

Build a shared byte slice using a cast pointer plus size_of. Union reads are unsafe even when your particular field pair has a valid-bits argument.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

The `Plain` contract says the type is Copy, has no padding, and every bit pattern of its size is a valid value. Generic unsafe code such as `bytes` relies on it to read the value as initialized bytes. Implementing it is an `unsafe impl` because the compiler cannot check the promise. bool would break it: only 0 and 1 are valid, so treating arbitrary bytes as a bool is undefined behavior. The same invalid patterns are why `Option<bool>` fits in one byte. Padding bytes are uninitialized, so a struct with padding cannot be viewed as `&[u8]` either. Reading a union field reinterprets the stored bits without validating them, so it is only sound when every pattern is valid for the field you read, as with u32. Layout facts such as size, alignment, and offset should be measured with `size_of`, `align_of`, and `offset_of!`, never assumed.

</details>

## l06 · Moving values and initialization

<details><summary>Hint 1</summary>

ptr::read copies representation bits while transferring ownership logically. The old owner must stop considering the slot initialized.

</details>

<details><summary>Hint 2</summary>

Set the Vec length smaller before reading its old final slot. MaybeUninit::write initializes a slot; assume_init is justified only after every returned slot was written.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

`ptr::read` copies the bits and transfers ownership, but the compiler still thinks the source is initialized. The bookkeeping must change so the old location is no longer treated as an owner. In `raw_pop`, `set_len(len - 1)` excludes the slot before or immediately after the read, with nothing in between that could panic. If a panic unwound between the read and the length update, the Vec would drop the slot again: a double drop. Other mistakes leak instead (a missed drop), which is safe but wrong. `MaybeUninit` never drops its contents, so `assume_init` (or `assume_init_read`) is justified only once every slot has been written, and the result then becomes the sole owner.

</details>

## l07 · A fixed-capacity byte gap buffer

<details><summary>Hint 1</summary>

The logical cursor is start. Moving left shifts the preceding bytes to just before end; moving right shifts following bytes to start.

</details>

<details><summary>Hint 2</summary>

Left distance = start-position; copy position..start to end-distance. Right distance = position-start; copy end..end+distance to start, then move both bounds together.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

The buffer holds `storage.len()` bytes. The text is `storage[..start]` followed by `storage[end..]`, the gap is `[start, end)`, and the cursor is `start`. The invariant is `0 <= start <= end <= storage.len()`, and the logical length is `storage.len() - (end - start)`. Moving the cursor left by d copies `storage[pos..start]` to `end - d`. Moving right copies `storage[end..end + d]` to `start`. In either case the source and destination can overlap, so the copy must be `ptr::copy` (memmove), not `copy_nonoverlapping`. Insertion writes at `start` when `start < end`; `start == end` means the buffer is full. Every byte stays initialized (the gap holds stale but valid u8s), so no ownership is at stake. A generic `Gap<T>` would need `MaybeUninit` and careful drops.

</details>

## l08 · Capstone: initialized-prefix storage

<details><summary>Hint 1</summary>

MaybeUninit does not drop its inner T automatically. The initialized-length invariant must drive both raw reads and cleanup.

</details>

<details><summary>Hint 2</summary>

Use write for push, assume_init_read for pop after shrinking len, and from_raw_parts for a shared view of only the prefix. No potentially panicking operation belongs after moving a value but before excluding its old slot.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

The invariant is `len <= N`, with exactly `data[..len]` initialized and owned by the container. `push` checks capacity, writes the slot, then increments `len`, with no user code or panicking operation in between. A failed push returns the value untouched. `pop` decrements `len` *first*, so the slot is no longer counted, then `assume_init_read`s it. The caller owns the result, and Drop will not touch it again. `as_slice` builds `&[T]` from `data.as_ptr().cast()` and `len`, and never covers unused slots, even if they still hold old bits. The supplied Drop removes each element from the bookkeeping before dropping it. A panicking destructor can therefore leak the rest, which is safe, but never cause a double drop.

</details>

## R01 · Spaced review of Chapters 17, 21, and 22

<details><summary>Hint 1</summary>

A: bind `$slice` and then `$index` to locals before anything else, exactly as in Chapter 22's evaluate-once macros. B: a Waker is a data pointer plus a vtable, and the data pointer here is an `Arc` turned into a raw pointer. C: a byte below 0x80 never appears inside a multi-byte UTF-8 sequence.

</details>
<details><summary>Hint 2</summary>

A: `if index < slice.len() { Some(unsafe { slice.get_unchecked(index) }) } else { None }`. B: `Arc::into_raw(wakes).cast()` and `Waker::from_raw(RawWaker::new(data, &VTABLE))`. Clone calls `Arc::increment_strong_count`, `wake` rebuilds the Arc with `Arc::from_raw` (and drops it), and `wake_by_ref` only borrows `&*data.cast::<AtomicUsize>()`. C: `text.bytes().position(|b| b == sep)?`, then `get_unchecked(..i)` and `get_unchecked(i + 1..)`.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one returns the right elements in the wrong order, and one accepts coordinates that are outside the grid. Then, for every `unsafe`, ask what it buys compared with a safe std method, and what happens if anything between a `ptr::read` and `set_len` panics.

</details>
<details><summary>Hint 2</summary>

`labels.split_off(labels.len().saturating_sub(n))` moves the tail in order with one allocation. In `get`, reject `col >= cols` and use `checked_mul`/`checked_add` before a plain `get(i)`. `copy_prefix` has no caller obligations, so it should be a safe fn using `copy_from_slice`.

</details>
<details><summary>Check your review</summary>

`take_last` reads elements back to front, so the output is reversed. It also grows `out` from empty, and `push` can allocate or panic after some elements have been `ptr::read` but before `set_len`. A panic there would leave each moved String owned twice, a double drop. `split_off` does the same work safely. `Grid::get` checks only the flat index, so `(0, 3)` silently returns cell `(1, 0)`. `row * cols` can overflow: that panics in debug and wraps in release. The `get_unchecked` saves nothing over `get`. `copy_prefix` is marked `unsafe fn` although callers have nothing to uphold. Its body calls an unsafe function without an `unsafe {}` block, which Rust 2024 warns about (`unsafe_op_in_unsafe_fn`), and there is no SAFETY comment. `copy_from_slice` is the safe equivalent.

</details>

## Drills

Run `cargo drill ch23`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Dereferencing a raw pointer</summary>E0133. Creating `p` is safe; reading through it is one of the operations `unsafe` unlocks, because the compiler cannot prove `p` is valid. Wrap only the read: `let y = unsafe { *p };` with a SAFETY comment saying `p` comes from a live reference.</details>
<details><summary>d02 · Making pointers is safe</summary>Compiles and runs. `wrapping_add` and comparisons never touch memory, so they are safe even far outside the array. Only dereferencing (or `add`, which requires staying in bounds) needs `unsafe`. An address is not permission to read.</details>
<details><summary>d03 · Writing through *const</summary>E0594. A `*const i32` does not permit writes, even inside `unsafe`. Declare `let p: *mut i32 = &mut x;`. `unsafe` grants extra operations; it does not change what a type allows.</details>
<details><summary>d04 · Implementing an unsafe trait</summary>E0200. An `unsafe trait` puts a promise on implementors, so the impl must say `unsafe impl Plain for u32 {}`, ideally with a SAFETY comment explaining why every bit pattern is a valid u32.</details>
<details><summary>d05 · unsafe impl of a safe trait</summary>E0199. `Clone` has no safety contract, so there is nothing to promise. Remove `unsafe`. Marking code unsafe without a contract only hides real unsafety.</details>
<details><summary>d06 · Raw pointers across threads</summary>E0277: `*const i32` is not `Send`. The compiler has no idea who owns the pointee or how long it lives. Move the owner instead: `thread::spawn(move || *data)`. Wrapping the pointer in a type with `unsafe impl Send` would need a real proof.</details>
<details><summary>d07 · transmute between sizes</summary>E0512: f32 is 4 bytes and u64 is 8, and `transmute` requires equal sizes. Use the safe API: `1.5_f32.to_bits() as u64`.</details>
<details><summary>d08 · A mutable static</summary>E0133: any access to a `static mut` is unsafe, because another thread could be using it at the same moment (a data race). `unsafe { HITS += 1 }` compiles, but the idiomatic fix is `static HITS: AtomicU32`, with `fetch_add` and `load`, and no unsafe at all.</details>
<details><summary>d09 · Checked layouts</summary>Panics: `Layout::from_size_align` returns `Err` because an alignment must be a power of two, and `unwrap` panics. Use 4 (or `Layout::new::<[u8; 12]>()`). Safe constructors check layout rules at run time; unchecked ones make them your obligation.</details>
<details><summary>d10 · Moving out through a raw pointer</summary>E0507. `*p` is a place you may read, but moving the String out would leave `name` and `copy` both owning one buffer. Clone it (`unsafe { (*p).clone() }`), or use `ptr::read` only when the original is then forgotten.</details>
