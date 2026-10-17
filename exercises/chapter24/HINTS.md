# Chapter 24 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try hint 1 before opening hint 2.

## l01 · C representations

<details><summary>Hint 1</summary>

Use std::ffi aliases and repr(C) to express the platform C ABI. They do not serialize pointers or Rust-owned collections.

</details>

<details><summary>Hint 2</summary>

Measure layout with size_of, align_of, and offset_of rather than hardcoding this machine’s sizes.

</details>

## l02 · C strings and ownership

<details><summary>Hint 1</summary>

CStr validates termination, not UTF-8. Owning the text requires an actual copy.

</details>

<details><summary>Hint 2</summary>

Use CString::new, CStr::to_str, and CStr::from_bytes_with_nul. No arbitrary-pointer scan is needed for the bounded-buffer task.

</details>

## l03 · ABI calls and out-parameters

<details><summary>Hint 1</summary>

MaybeUninit lets you allocate output storage without inventing an initialized value. Its contents become valid only after the documented success status.

</details>

<details><summary>Hint 2</summary>

Call with out.as_mut_ptr, inspect status, then use assume_init only in the success branch. Preserve the extern C function-pointer type.

</details>

## l04 · Callbacks and userdata

<details><summary>Hint 1</summary>

The pointer is borrowed, not transferred. This mock API promises callbacks finish before foreign_map returns.

</details>

<details><summary>Hint 2</summary>

Create a Vec of initialized output slots, pass as_ptr/as_mut_ptr and a c_void cast of a local factor, then return the Vec after the call.

</details>

## l05 · Opaque handle ownership

<details><summary>Hint 1</summary>

The owner has one non-null pointer and no Clone. Drop is supplied to preserve safety even while exercise functions are unfinished.

</details>

<details><summary>Hint 2</summary>

Create CString before raw_open. Borrow the Resource only while &self exists, then allocate owned output with to_owned.

</details>

## l06 · Owner-bound foreign borrows

<details><summary>Hint 1</summary>

The lifetime must appear in the view’s type even though the raw pointer does not carry a Rust lifetime itself.

</details>

<details><summary>Hint 2</summary>

Use PhantomData<&Repository>. Tie construction to &self, then justify from_ptr from the immutable CString that remains live.

</details>

## l07 · Capstone: checked foreign transaction

<details><summary>Hint 1</summary>

The foreign contract tells you which output is initialized on each status path. Reading the other one is never justified.

</details>

<details><summary>Hint 2</summary>

Allocate two MaybeUninit slots, branch on status, and copy CStr to String before returning the error. Add index context in batch using enumerate.

</details>

