# Unsafe audits and deeper implementations

All experiments use valid allocations and initialized values under stated contracts. Keep intentionally invalid pointer, use-after-free, aliasing, or double-drop scenarios on paper; do not execute undefined behavior as a demonstration.

## Safety proof and UB boundary

For every unsafe block list allocation/provenance, in-bounds range, alignment, initialization/validity, aliasing, lifetime, and ownership obligations. Identify which follow from the caller contract and which from a private invariant. Compare an unsafe fn (caller obligations), unsafe trait (implementor obligations), and a safe API containing unsafe code (implementation obligations). Distinguish panics/compiler errors from undefined behavior and explain why debug assertions cannot make a missing safety contract sound. Add SAFETY comments at every call site and audit unsafe operations inside unsafe fn explicitly (Rust 2024 requires explicit blocks).

## Extend the gap buffer to non-Copy values

In an isolated package build `Gap<T>` backed by allocated `MaybeUninit<T>` slots, with initialized regions `[0,start)` and `[end,capacity)`. Implement cursor movement with overlap-aware ptr::copy, insertion, deletion returning ownership with ptr::read, iteration over both regions, and Drop of exactly the live regions. Never create &[T] spanning the gap. Keep the implementation fixed-capacity first; add growth only after a complete ownership ledger. Acceptance: compare a sequence of edits with a Vec<String> model, include empty/full capacity, cursor endpoints, zero-sized types with Drop, rejected insertion retaining its input, and per-element Drop counters. Explain why copied representation bits remaining inside the gap are not a second live T. Verify no user code/panicking indexing/allocation can run after a move but before the ownership ranges are corrected.

## Panic safety under user callbacks

Add `try_extend` that accepts an iterator which panics after yielding two elements. Catch the panic outside the container, then verify the existing prefix is valid, the successfully inserted values are dropped once, and the remaining iterator-owned values follow their own ownership rules. Use a guard or update initialized length immediately after each successful write. Add a retain/filter operation only after identifying what happens if the predicate panics. State whether each operation gives the basic or strong exception guarantee. A panic in a destructor may leak remaining elements during unwinding; it must never cause a double drop or invalid read. Do not put TODOs or assertions in Drop scaffolding.

## Tagged-pointer transfer exercise

Extend the flagged borrow with two flag bits only for an explicitly aligned owned type (repr(align(4)) or greater). Preserve provenance with map_addr and lifetimes with PhantomData; mask all tag bits before access. Acceptance: compile-fail lifetime escape, all flag combinations, address identity, and written alignment proof. Discuss strict versus exposed provenance and why printing an address is different from reconstructing an arbitrary usable pointer.

## Dynamic checks (optional toolchain component)

If Miri is available for a suitable nightly toolchain, run your completed unsafe core and isolated gap-container tests under it. Also run ordinary debug/release tests. Record tool versions and results; do not claim a proof from a passing test run. Explain untested type/layout/platform cases and how each generic safety argument covers them.

References: [Rustonomicon](https://doc.rust-lang.org/nomicon/), [std::ptr and provenance](https://doc.rust-lang.org/std/ptr/), [MaybeUninit](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html), and [Miri](https://github.com/rust-lang/miri).
