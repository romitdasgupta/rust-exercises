# Chapter 14 notes — Closures

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Borrowed captures and sorting

**Predictions I got wrong, and why:**

- 

**Explain:** Why is the captured prefix shared rather than mutable? When is the borrow released?

> 

**Edge case I added:**

- 

## l02 · Mutation, move, and consuming captures

**Predictions I got wrong, and why:**

- 

**Explain:** Which closures implement Fn, FnMut, and FnOnce? Explain why every Fn closure also supports the weaker call traits.

> 

**Edge case I added:**

- 

## l03 · Choosing callable bounds

**Predictions I got wrong, and why:**

- 

**Explain:** Explain why apply_once should not require Fn and why invoke_twice cannot accept a closure that consumes a captured String.

> 

**Edge case I added:**

- 

## l04 · Returning borrowed and erased closures

**Predictions I got wrong, and why:**

- 

**Explain:** Compare the allocation and dispatch choices of impl Fn and Box<dyn Fn>. What is the captured-data lifetime of each API?

> 

**Edge case I added:**

- 

## l05 · Copy and Clone for captures

**Predictions I got wrong, and why:**

- 

**Explain:** How do the capabilities of captured fields determine a closure’s Clone and Copy implementations?

> 

**Edge case I added:**

- 

## l06 · Callback registration and invocation

**Predictions I got wrong, and why:**

- 

**Explain:** Why is a callback list homogeneous in its erased interface but heterogeneous in concrete types? Why does dispatch need &mut self?

> 

**Edge case I added:**

- 

## l07 · Capstone: configurable text processing

**Predictions I got wrong, and why:**

- 

**Explain:** Explain how the bounds express policy without hard-coding the policy. Which callback must not be invoked for rejected input?

> 

**Edge case I added:**

- 

## r01 · Spaced retrieval: Chapters 10–13 inside Chapter 14

**Predictions I got wrong, and why:**

- 

**Edge case I added:**

- 

## c01 · A teammate's pull request

**My review, written before running the tests:**

- 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**Determine whether a closure captures each variable by shared borrow, mutable borrow, or move.**

> 

**Choose the weakest useful Fn/FnMut/FnOnce bound.**

> 

**Explain why move does not imply FnOnce-only.**

> 

**Return closures with clear lifetimes and distinguish generic from boxed dispatch.**

> 

**Register and invoke callbacks without invalidating captured references.**

> 

**Redo the capstone from its contract after a few days without consulting your earlier implementation.**

> 

**Present your predictions, failed compiler experiments, and one new edge case for review.**

>
