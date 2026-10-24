# Chapter 4 notes — Ownership and Moves

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Ownership scopes

**Predictions I got wrong, and why:**

- 

**Explain:** Draw the owner tree for a vector of strings and explain when destruction happens.

> 

**Edge case I added:**

- 

## l02 · Moves across assignments and fields

**Predictions I got wrong, and why:**

- 

**Explain:** Explain partial moves, the remaining fields, and the difference between moving a tuple and destructuring it.

> 

**Edge case I added:**

- 

## l03 · Moves and control flow

**Predictions I got wrong, and why:**

- 

**Explain:** Explain how definite initialization applies to every path and every loop iteration.

> 

**Edge case I added:**

- 

## l04 · Moving from indexed collections

**Predictions I got wrong, and why:**

- 

**Explain:** Explain why indexed storage must remain initialized and compare order and complexity of extraction operations.

> 

**Edge case I added:**

- 

## l05 · Leaving a replacement behind

**Predictions I got wrong, and why:**

- 

**Explain:** Describe the invariant restored by replace and take, including the state observed by a caller.

> 

**Edge case I added:**

- 

## l06 · Copy and explicit cloning

**Predictions I got wrong, and why:**

- 

**Explain:** Explain value semantics, Copy opt-in, and independent deep data after cloning a String.

> 

**Edge case I added:**

- 

## l07 · Rc and Arc

**Predictions I got wrong, and why:**

- 

**Explain:** Explain strong counts, destruction of the last owner, thread safety costs, and why reference cycles can leak.

> 

**Edge case I added:**

- 

## l08 · Capstone: owned dispatch queue

**Predictions I got wrong, and why:**

- 

**Explain:** Draw every allocation and owner after dispatch. Explain why sharing immutable configuration does not imply sharing mutable job labels.

> 

**Edge case I added:**

- 

## r01 · Spaced retrieval: Chapter 3 inside Chapter 4

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

**Draw stack/heap ownership before and after a move, clone, and drop.**

> 

**Explain which operations invalidate the source binding and why a moved binding can be reassigned.**

> 

**Extract an owned collection element without leaving uninitialized storage.**

> 

**Explain why Copy cannot be derived for String and why cloning Rc differs from cloning its value.**

> 

**Distinguish Rc and Arc, shared ownership and mutable access, and reference cycles.**

> 

**Redo the final capstone after a break without looking at your implementation; invent a changed requirement and explain the tradeoff.**

>
