# Chapter 19 notes — Threads

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Fork, join, and worker failures

**Predictions I got wrong, and why:**

- 

**Explain:** Explain result nesting and ownership crossing a move closure.

> 

**Edge case I added:**

- 

## l02 · Borrowed parallel work

**Predictions I got wrong, and why:**

- 

**Explain:** Explain the relationship between scope lifetime, disjoint borrows, and joining.

> 

**Edge case I added:**

- 

## l03 · Ownership and channel closure

**Predictions I got wrong, and why:**

- 

**Explain:** Explain FIFO per sender, nondeterministic ordering across senders, and bounded-channel backpressure.

> 

**Edge case I added:**

- 

## l04 · Send and Sync contracts

**Predictions I got wrong, and why:**

- 

**Explain:** Explain auto traits, ownership transfer, and why a mutex does not make every inner value Send.

> 

**Edge case I added:**

- 

## l05 · Mutex invariants and poisoning

**Predictions I got wrong, and why:**

- 

**Explain:** State the counter invariant. Explain guard lifetimes, panic poisoning, lock order, and why recursive locking can deadlock.

> 

**Edge case I added:**

- 

## l06 · RwLock and condition predicates

**Predictions I got wrong, and why:**

- 

**Explain:** Explain the predicate loop, spurious wakeups, and releasing a mutex while waiting.

> 

**Edge case I added:**

- 

## l07 · Atomic counters and local state

**Predictions I got wrong, and why:**

- 

**Explain:** Distinguish atomicity, ordering, and per-thread storage. Explain why a mutable static would need an invariant beyond these counters.

> 

**Edge case I added:**

- 

## l08 · Capstone: bounded parallel pipeline

**Predictions I got wrong, and why:**

- 

**Explain:** Trace ownership and closure of every sender. Explain when sorted output is a contract versus hiding a bug.

> 

**Edge case I added:**

- 

## r01 · Spaced retrieval: Chapters 14–18 inside Chapter 19

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

**When can scoped threads borrow data that spawn cannot?**

> 

**How do join errors differ from a worker Result error?**

> 

**Which values must be Send, and when does Arc<T> require Sync?**

> 

**How do channel closure, bounded backpressure, and a forgotten sender affect termination?**

> 

**What invariant is a mutex protecting; why does a Condvar wait in a loop?**

> 

**Which guarantees does relaxed atomic arithmetic provide, and which does it not provide?**

> 

**How can you design concurrent tests around completion signals rather than sleeps?**

>
