# Chapter 7 notes — Error Handling

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Panics and invariants

**Predictions I got wrong, and why:**

- 

**Explain:** Explain why panic is appropriate for the stated invariant, and why ordinary missing input should use Result instead.

> 

**Edge case I added:**

- 

## l02 · Catching Result errors and printing context

**Predictions I got wrong, and why:**

- 

**Explain:** Explain recovery policy and how a user-facing message differs from the underlying parse error.

> 

**Edge case I added:**

- 

## l03 · Propagating failures with question mark

**Predictions I got wrong, and why:**

- 

**Explain:** Explain successful extraction, early return, and why parse errors are preserved rather than converted into a success value.

> 

**Edge case I added:**

- 

## l04 · Result methods and aliases

**Predictions I got wrong, and why:**

- 

**Explain:** Explain how combinators transform either the success or error side while retaining explicit failure.

> 

**Edge case I added:**

- 

## l05 · Multiple error types at a boundary

**Predictions I got wrong, and why:**

- 

**Explain:** Explain ? conversion into a boxed error and compare a concrete error enum with a universal application-boundary error.

> 

**Edge case I added:**

- 

## l06 · Custom errors with a source chain

**Predictions I got wrong, and why:**

- 

**Explain:** Explain the roles of Debug, Display, Error::source, and typed variants for caller recovery.

> 

**Edge case I added:**

- 

## l07 · Impossible errors and deliberate recovery

**Predictions I got wrong, and why:**

- 

**Explain:** Explain the documented proof behind expect and the information intentionally discarded by a recovery policy.

> 

**Edge case I added:**

- 

## l08 · Capstone: fallible batch report

**Predictions I got wrong, and why:**

- 

**Explain:** Explain how typed errors support recovery, why the first failed row wins, and how a CLI main should convert this API into an exit status.

> 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**Choose Result for expected failures and panic for a documented broken invariant.**

> 

**Explain unwinding versus aborting and why catch_unwind is not general error handling.**

> 

**Propagate errors with ? and preserve contextual information and source chains.**

> 

**Use typed errors, boxed trait-object errors, or anyhow according to the boundary and recovery needs.**

> 

**Explain what ignoring an error means and make main return a useful exit status.**

> 

**Redo the final capstone after a break without looking at your implementation; invent a changed requirement and explain the tradeoff.**

>
