# Chapter 5 notes — References

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Borrowing and dereferencing

**Predictions I got wrong, and why:**

- 

**Explain:** Explain who owns storage and why a method receiver can be dereferenced automatically while assignment through &mut needs *.

> 

**Edge case I added:**

- 

## l02 · Reference bindings and identity

**Predictions I got wrong, and why:**

- 

**Explain:** Explain reference binding mutability, referent mutability, nested references, and std::ptr::eq.

> 

**Edge case I added:**

- 

## l03 · Optional references and fat pointers

**Predictions I got wrong, and why:**

- 

**Explain:** Explain borrowed temporaries, Option<&T>, slice length metadata, and why a borrow does not own the referent.

> 

**Edge case I added:**

- 

## l04 · Lifetimes through functions

**Predictions I got wrong, and why:**

- 

**Explain:** Describe the input-to-output lifetime relationship and why it does not allocate or extend any lifetime.

> 

**Edge case I added:**

- 

## l05 · Structs that borrow

**Predictions I got wrong, and why:**

- 

**Explain:** Explain why the struct has a lifetime parameter and how fields remain tied to one owner.

> 

**Edge case I added:**

- 

## l06 · Independent lifetimes and elision

**Predictions I got wrong, and why:**

- 

**Explain:** State the function lifetime elision rules and explain when independent lifetimes let one result outlive another input.

> 

**Edge case I added:**

- 

## l07 · Sharing versus mutation

**Predictions I got wrong, and why:**

- 

**Explain:** Explain last use, reallocation invalidation, exclusive access, and why runtime capacity is not enough to permit an overlapping borrow.

> 

**Edge case I added:**

- 

## l08 · Disjoint mutable borrows

**Predictions I got wrong, and why:**

- 

**Explain:** Explain which facts are encoded by split_at_mut and why it does not require unsafe in caller code.

> 

**Edge case I added:**

- 

## l09 · Capstone: borrowed roster queries

**Predictions I got wrong, and why:**

- 

**Explain:** Compare returning a borrow, returning an index, and cloning an owner. Explain invalidation of indices on removal and the caller obligations of each API.

> 

**Edge case I added:**

- 

## r01 · Spaced retrieval: Chapters 3 and 4 inside Chapter 5

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

**Distinguish changing a reference from changing its referent.**

> 

**Explain automatic dereferencing, value equality versus pointer identity, and the metadata in slice references.**

> 

**Reject references that can outlive their owner and annotate relationships rather than extending lifetimes.**

> 

**Use distinct lifetimes for independently borrowed fields and explain the elision rules.**

> 

**Resolve borrow conflicts with last-use boundaries or disjoint access, without unnecessary cloning.**

> 

**Redo the final capstone after a break without looking at your implementation; invent a changed requirement and explain the tradeoff.**

>
