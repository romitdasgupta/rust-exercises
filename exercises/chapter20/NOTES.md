# Chapter 20 notes — Asynchronous Programming

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Lazy futures and await

**Predictions I got wrong, and why:**

- 

**Explain:** Trace future creation, first polling, and await sequencing.

> 

**Edge case I added:**

- 

## l02 · Borrowed and owned futures

**Predictions I got wrong, and why:**

- 

**Explain:** Explain the difference between moving a reference and moving ownership of its referent.

> 

**Edge case I added:**

- 

## l03 · Async behavior in traits

**Predictions I got wrong, and why:**

- 

**Explain:** Explain static dispatch for async trait methods and when explicit boxed futures are useful.

> 

**Edge case I added:**

- 

## l04 · Lending async callbacks

**Predictions I got wrong, and why:**

- 

**Explain:** Explain why the callback bound is AsyncFnMut and what happens when the caller awaits each call before the next.

> 

**Edge case I added:**

- 

## l05 · Cancellation by dropping

**Predictions I got wrong, and why:**

- 

**Explain:** Distinguish captured resources from resources created after polling. Explain what cancellation does to local guards.

> 

**Edge case I added:**

- 

## l06 · Polling a race

**Predictions I got wrong, and why:**

- 

**Explain:** Explain how losing futures are dropped, and why this small model does not cover tokio::select! syntax, fairness, task aborts, or timers.

> 

**Edge case I added:**

- 

## l07 · Capstone: asynchronous graph traversal

**Predictions I got wrong, and why:**

- 

**Explain:** Describe the sequential core honestly. Design the bounded spawned-task version from LABS.md, with task results, cancellation, and shutdown accounted for.

> 

**Edge case I added:**

- 

## r01 · Spaced retrieval: Chapters 13, 17–19 inside Chapter 20

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

**Why is calling an async function different from executing its body?**

> 

**What are the borrowing, Send, and static requirements of a spawned task?**

> 

**Which work can safely block an executor thread, and what belongs in spawn_blocking?**

> 

**How do AsyncFn, AsyncFnMut, and AsyncFnOnce express lending asynchronous callbacks?**

> 

**What state is lost when a future is dropped in a select! branch?**

> 

**How do abort, cooperative signals, and cancellation tokens differ?**

> 

**What evidence demonstrates that a concurrent traversal actually bounds its work and collects all task errors?**

>
