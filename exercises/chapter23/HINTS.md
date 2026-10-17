# Chapter 23 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try hint 1 before opening hint 2.

## l01 · A safe invariant boundary

<details><summary>Hint 1</summary>

Unsafe code relies on an invariant that safe callers cannot break through the public API.

</details>

<details><summary>Hint 2</summary>

Validate with is_ascii; borrow with from_utf8_unchecked only after explaining why every stored byte is guaranteed ASCII.

</details>

## l02 · Raw pointer reads with contracts

<details><summary>Hint 1</summary>

The unsafe function cannot validate arbitrary pointer provenance. Its safe caller should derive the pointer from a valid borrow.

</details>

<details><summary>Hint 2</summary>

Bounds-check before add, then place only the read in a justified unsafe block. Copying u32 does not transfer allocation ownership.

</details>

## l03 · Raw slice reconstruction

<details><summary>Hint 1</summary>

The safety argument concerns reference creation, not just later use. Empty regions still need valid aligned non-null pointers.

</details>

<details><summary>Hint 2</summary>

Get len and as_mut_ptr before constructing either slice. ptr::swap permits identical pointers, unlike copy_nonoverlapping-style assumptions.

</details>

## l04 · A flagged borrow

<details><summary>Hint 1</summary>

Keep the original pointer provenance rather than converting to an integer and inventing a new pointer.

</details>

<details><summary>Hint 2</summary>

map_addr transforms only address bits. Mask the tag before dereferencing; PhantomData makes the lifetime relation visible to borrow checking.

</details>

## l05 · Layout, unsafe traits, unions

<details><summary>Hint 1</summary>

An unsafe trait puts obligations on every unsafe impl; generic unsafe code may rely on those obligations.

</details>

<details><summary>Hint 2</summary>

Build a shared byte slice using a cast pointer plus size_of. Union reads are unsafe even when your particular field pair has a valid-bits argument.

</details>

## l06 · Moving values and initialization

<details><summary>Hint 1</summary>

ptr::read copies representation bits while transferring ownership logically. The old owner must stop considering the slot initialized.

</details>

<details><summary>Hint 2</summary>

Set the Vec length smaller before reading its old final slot. MaybeUninit::write initializes a slot; assume_init is justified only after every returned slot was written.

</details>

## l07 · A fixed-capacity byte gap buffer

<details><summary>Hint 1</summary>

The logical cursor is start. Moving left shifts the preceding bytes to just before end; moving right shifts following bytes to start.

</details>

<details><summary>Hint 2</summary>

Left distance = start-position; copy position..start to end-distance. Right distance = position-start; copy end..end+distance to start, then move both bounds together.

</details>

## l08 · Capstone: initialized-prefix storage

<details><summary>Hint 1</summary>

MaybeUninit does not drop its inner T automatically. The initialized-length invariant must drive both raw reads and cleanup.

</details>

<details><summary>Hint 2</summary>

Use write for push, assume_init_read for pop after shrinking len, and from_raw_parts for a shared view of only the prefix. No potentially panicking operation belongs after moving a value but before excluding its old slot.

</details>

