# Authoring guide

How the exercises are built, for anyone adding or changing a chapter. Learners don't need this file. [Chapter 4](exercises/chapter04/) is the reference implementation of every convention below; copy its patterns.

## Learning design

Each element exists to build long-term, usable skill rather than short-term recognition:

| Element | Why |
| --- | --- |
| `p_predictions` tests | Retrieval and prediction before feedback. The learner writes a value, and the test checks it immediately. |
| Compiler drills (`cargo drill`) | Day-to-day Rust is reading diagnostics. Learners predict the outcome, see the verdict, then fix the code. |
| Technique tests | Green tests should mean the intended technique was used, not just that the output is right. |
| `r01_review.rs` | Spaced, interleaved retrieval of earlier chapters, done from memory. |
| `c01_code_review.rs` | Working-but-wrong code: reviewing, refactoring, and judging API design are core work skills. |
| `NOTES.md` | Written explanations are the main retention artifact and the basis for spaced review. |
| *Check your explanation* hints | Feedback on explanations, revealed only after the learner writes their own. |
| `cargo progress` | Status plus a spaced-review schedule (1, 3, 7, 21, 60 days). |

## Lesson files (`lNN_*.rs`)

- Keep the existing contracts and tests. Add tests; don't weaken them.
- Header: the `PREDICT before running:` line starts with ``first answer `p_predictions` at the bottom (replace each `guess()` with your answer). Then consider:``.
- **`p_predictions`**, at the end of `mod tests`: 2–4 predictions about *observable runtime behavior* related to the lesson (values, drop order via a logging `Drop` type, pointer equality, allocation counts, `size_of`, iteration order, `Rc::strong_count`, overflow results...).
  - Pattern: show the code, ask in a `// P1.` comment, `let p1: T = guess();`, then `assert_eq!(p1, <actual expression>, "P1: short topic");`.
  - The correct answer must not be readable from the source; it is computed by the expression the learner must mentally evaluate.
  - Every prediction must be deterministic across platforms (64-bit Linux/macOS/Windows) and both profiles. Avoid hash iteration order, capacities after growth, and thread interleavings, unless the prediction is "it is unspecified", which a test cannot check.
  - Conceptual or "does it compile" questions belong in drills or NOTES, not here.
- **Technique tests** where the contract states a technique ("no clones", "borrow", "in place", "one allocation", "use threads"). Add `use practice::alloc::count;` and assert upper bounds: `assert!(allocations <= n, "...")` or `== 0`. Never assert an exact non-zero count in technique tests; release builds can elide allocations. Other tools: `as_ptr()` and `ptr::eq` identity, `Rc::ptr_eq`, counting distinct `thread::current().id()`, `size_of`, drop logs.
- A chapter that uses `count` needs this once in its `mod.rs`:

  ```rust
  #[global_allocator]
  static ALLOC: practice::alloc::Counting = practice::alloc::Counting;
  ```

## Review lesson (`r01_review.rs`)

- 3 tasks, plus a `p_predictions` test of earlier-chapter facts. Each task combines one earlier idea with this chapter's material. Draw from chapter N−1, from N−2 or N−3, and from one chapter at least four back when one exists. Label each task with its chapters, e.g. `/// A. (Ch5 borrows + Ch9 structs)`.
- Header opens with the "do this WITHOUT reopening earlier chapters" instruction from Chapter 4.
- Chapter 3 has no review lesson.

## Code review lesson (`c01_code_review.rs`)

- Ships *working-looking* code (no `todo!()`) written the way a hurried teammate might: 2–3 functions, 2 real bugs, plus avoidable clones, needless allocation, wrong ownership in signatures, `unwrap()` where errors should propagate, non-idiomatic loops, missed std APIs, or concurrency/unsafe hazards suited to the chapter.
- Tests: behavior tests that expose the bugs (some pass at the start, which is realistic) and technique tests that expose the smells.
- If a better signature is part of the fix, write the tests so they compile with both the starting and the improved signature (for example `.as_deref()` on the result, or passing `&vec` where `&[T]` will be accepted), and say in the doc comment that the type may change.
- Some clippy lints are deny-level (for example `derived_hash_with_manual_eq`, `approx_constant`, `erasing_op`). Seeded smells must stay at warning level so `cargo clippy` still builds; use a manual impl or a targeted `#[allow]` with a comment.
- The header follows Chapter 4: review in NOTES.md first, then run the tests, fix the code, and clear clippy warnings.

## Drills (`drills/chNN/dNN_name.rs`)

- 8–10 standalone programs per chapter (`fn main`, std only, edition 2024). Mix the outcomes: mostly compile errors with error codes, at least one that compiles (`ok`), and a run-time `panic` where the topic allows. Pick the misconceptions people actually hit at work.
- Use the exact header from `drills/ch04/d01_use_after_move.rs`, with a one-line "Ask:" nudge.
- `main` asserts the intended behavior. Lines that must survive the fix end with `// KEEP`, so the drill can't be "fixed" by deleting the problem.
- Verify each drill's real outcome with `rustc --edition 2024 -A warnings`, and check that a reasonable fix compiles and runs. Keep the error-code set stable and small, ideally exactly one code. Errors without a code (some lifetime errors) are allowed; the expected answer is then `error`.
- Add a `## Drills` section to the chapter's HINTS.md: one collapsed `<details>` per drill giving the outcome, why it happens, and the idiomatic fix.

## HINTS.md

For every lesson, keep the two hints and add a third block, `<details><summary>Check your explanation (after writing your own)</summary>`: a model answer of 3–6 sentences to the lesson's EXPLAIN prompt. Then add sections for R01 and C01 (two hints each; C01 also gets *Check your review*), and the Drills section.

## Chapter README

Replace "Work through the chapter" (or its equivalent) with the Chapter 4 version, adjusted for the chapter number. Add rows for R01, C01, and the drills to the lesson table, and `cargo clippy --test chapterNN` and `cargo drill chNN` to the command block. Keep the coverage map and the mastery check.

## NOTES.md

Generated by `python3 scripts/make_notes.py NN --force` from lesson headers and the README's mastery check. Regenerate after adding lessons. Never use `--force` on a learner's filled-in notes.

## Validation (required before committing)

1. `cargo fmt --all --check` and `cargo clippy --all-targets --all-features` pass with no errors. Warnings caused by `todo!()` bodies are acceptable.
2. The starters compile: `cargo test --locked --offline --all-targets --no-run`, in debug and `--release`.
3. **Reference check:** in a scratch copy, implement every `todo!()` correctly, replace every `guess()` with the right answer, and fix the c01 code. Then run `cargo test --test chapterNN` **in both profiles**; everything must pass. Also confirm that at least one plausible wrong technique (an extra clone, a sequential version, and so on) fails the technique tests. Do not commit the reference solutions.
4. Starting state: in the committed starter, every `p_predictions` test fails at `guess()`, the c01 tests expose its bugs and smells, and other tests fail at `todo!()` as before.
5. `cargo drill chNN` works on the chapter's drills: predict one drill, fix it, and confirm the ✓. Then delete `.progress/` and restore the drill file.
