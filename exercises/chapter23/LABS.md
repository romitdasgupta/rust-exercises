# Unsafe audits and deeper implementations

All experiments use valid allocations and initialized values under stated contracts. Keep intentionally invalid pointer, use-after-free, aliasing, or double-drop scenarios on paper; do not execute undefined behavior as a demonstration.

Labs 2–4 build an isolated package outside this repository. One minimal manifest serves all of them (standard library only):

```toml
[package]
name = "unsafe-lab"
version = "0.1.0"
edition = "2024"

[dependencies]
```

## Lab 1 · Safety proof and UB boundary

Goal: write the safety argument for every unsafe block in your finished lessons.

1. For every unsafe block in l01–l08, list its obligations under these headings: allocation/provenance, in-bounds range, alignment, initialization/validity, aliasing, lifetime, and ownership.
   *Checkpoint:* each heading has an entry for each block, even when it is "not applicable, because ...".
2. Mark which obligations follow from a caller's contract (an `unsafe fn`'s `# Safety` section) and which from a private invariant of a safe type.
3. Compare the three places a contract can live: an `unsafe fn` (caller obligations), an `unsafe trait` (implementor obligations), and a safe API that contains unsafe code (implementation obligations). Use one example of each from this chapter.
4. Distinguish panics and compiler errors from undefined behavior. Explain why debug assertions cannot make a missing safety contract sound.
5. Add a `// SAFETY:` comment at every unsafe call site. Audit the unsafe operations inside every `unsafe fn` explicitly: Rust 2024 requires an explicit `unsafe {}` block for them (`unsafe_op_in_unsafe_fn`).
   *Checkpoint:* `cargo clippy --test chapter23` reports no `unsafe_op_in_unsafe_fn` warnings for your code, and every unsafe block has a SAFETY comment.

Acceptance criteria:

- [ ] Every unsafe block has a written obligation list under all seven headings.
- [ ] Each obligation is attributed to a caller contract or to a private invariant.
- [ ] The unsafe fn / unsafe trait / safe-API comparison is written, with examples.
- [ ] The panic versus UB distinction and the debug-assertion argument are written.
- [ ] SAFETY comments everywhere, and explicit unsafe blocks inside unsafe fns.

## Lab 2 · Extend the gap buffer to non-Copy values

Goal: a `Gap<T>` that owns arbitrary values, built from the byte version in l07.

1. In the isolated package, define `Gap<T>` backed by allocated `MaybeUninit<T>` slots. The initialized regions are `[0, start)` and `[end, capacity)`. Keep it fixed-capacity for now.
   *Checkpoint:* `Gap::<String>::with_capacity(4)` constructs and drops without touching any slot.
2. Implement cursor movement with an overlap-aware `ptr::copy`.
3. Implement insertion. A rejected insertion (buffer full) returns its input to the caller.
4. Implement deletion, which returns ownership with `ptr::read`.
5. Implement iteration over both initialized regions. Never create a `&[T]` that spans the gap.
6. Implement `Drop` so it drops exactly the live regions, once each.
   *Checkpoint:* a per-element Drop counter shows each inserted value dropped exactly once, whether it was deleted, returned, or still inside at drop time.
7. Verify that no user code, panicking indexing, or allocation can run after a move but before the ownership ranges are corrected. Annotate each move site.
8. Only after a complete ownership ledger, add growth.
9. Explain why the copied representation bits that remain inside the gap are not a second live `T`.

Acceptance criteria:

- [ ] A sequence of edits matches a `Vec<String>` model after every step.
- [ ] Empty and full capacity.
- [ ] Cursor at both endpoints.
- [ ] A zero-sized type with Drop.
- [ ] A rejected insertion keeps its input.
- [ ] Per-element Drop counters prove exactly-once drops.
- [ ] Written explanation of stale bits in the gap, and the move-site audit from step 7.

## Lab 3 · Panic safety under user callbacks

Goal: keep the container valid when user code panics in the middle of an operation.

1. Add `try_extend` to your l08 `Slots` (or to `Gap<T>`). Test it with an iterator that panics after yielding two elements.
2. Catch the panic outside the container with `std::panic::catch_unwind`.
   *Checkpoint:* after the panic, the existing prefix is valid, the two inserted values are in the container, and the remaining iterator-owned values follow their own ownership rules.
3. Make this hold by construction: use a guard, or update the initialized length immediately after each successful write.
4. Only after identifying what happens if the predicate panics, add a `retain` (filter) operation.
5. For each operation, state whether it gives the basic or the strong exception guarantee.
6. Consider a panic in a destructor during unwinding. It may leak the remaining elements, but it must never cause a double drop or an invalid read. Do not put TODOs or assertions in Drop scaffolding.

Acceptance criteria:

- [ ] A `catch_unwind` test shows the prefix valid and each successfully inserted value dropped exactly once.
- [ ] `retain` is tested with a panicking predicate, and its behavior is written down.
- [ ] Basic/strong guarantee stated for every operation.
- [ ] Destructor-panic behavior explained: leaks allowed, double drops never.

## Lab 4 · Tagged-pointer transfer exercise

Goal: two tag bits, with a written alignment proof.

1. Extend the l04 flagged borrow to two flag bits, only for an explicitly aligned owned type (`#[repr(align(4))]` or greater).
2. Preserve provenance with `map_addr` and lifetimes with `PhantomData`. Mask *all* tag bits before every access.
   *Checkpoint:* all four flag combinations round-trip, and the masked pointer equals the original address.
3. Write the alignment proof: why the two low bits of every valid pointer to your type are zero.
4. Discuss strict versus exposed provenance. Explain why printing an address differs from reconstructing an arbitrary usable pointer.

Acceptance criteria:

- [ ] A compile-fail check showing the lifetime cannot escape (keep the error; do not use `'static`).
- [ ] All flag combinations tested.
- [ ] Address identity tested.
- [ ] Written alignment proof and provenance discussion.

## Lab 5 · Dynamic checks (optional toolchain component)

Goal: run what tools can check, and be clear about what they cannot.

1. If Miri is available for a suitable nightly toolchain (`rustup +nightly component add miri`), run your completed unsafe core and the isolated gap-container tests under it (`cargo +nightly miri test`).
   *Checkpoint:* Miri reports no undefined behavior for the tested paths, or you have recorded and fixed each report.
2. Also run the ordinary tests in debug and `--release`.
3. Record tool versions and results. Do not claim a proof from a passing test run.
4. List the type, layout, and platform cases you did not test, and explain how each generic safety argument covers them.

Acceptance criteria:

- [ ] Tool versions and Miri, debug, and release results recorded.
- [ ] A list of untested cases, each with the argument that covers it.

References: [Rustonomicon](https://doc.rust-lang.org/nomicon/), [std::ptr and provenance](https://doc.rust-lang.org/std/ptr/), [MaybeUninit](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html), and [Miri](https://github.com/rust-lang/miri).
