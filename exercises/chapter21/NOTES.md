# Chapter 21 notes — Futures

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Implement Future

**Predictions I got wrong, and why:**

- 

**Explain:** Explain Output, Pin<&mut Self>, and why this implementation uses Unpin.

> 

**Edge case I added:**

- 

## l02 · Pending and wakeups

**Predictions I got wrong, and why:**

- 

**Explain:** Explain wake coalescing and why a future should not self-wake without making bounded progress.

> 

**Edge case I added:**

- 

## l03 · A completion cell

**Predictions I got wrong, and why:**

- 

**Explain:** Trace a completion-before-poll and poll-before-completion interleaving. Explain why one locked state prevents lost wakeups.

> 

**Edge case I added:**

- 

## l04 · A parker block_on

**Predictions I got wrong, and why:**

- 

**Explain:** Explain the wake token, spurious park returns, and the runtime features this tiny executor lacks.

> 

**Edge case I added:**

- 

## l05 · Readiness registration model

**Predictions I got wrong, and why:**

- 

**Explain:** Separate a readiness signal from I/O completion. Explain registering a waker under the same lock as the ready flag.

> 

**Edge case I added:**

- 

## l06 · Cooperative work budgets

**Predictions I got wrong, and why:**

- 

**Explain:** Explain bounded work per poll and why a self-wake after progress is appropriate here.

> 

**Edge case I added:**

- 

## l07 · Capstone: pinned future adapter

**Predictions I got wrong, and why:**

- 

**Explain:** Explain how storing Pin<Box<F>> lets a wrapper project safely without handwritten unsafe code. Relate this to future construction and first poll.

> 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**What promises accompany Poll::Pending, and why must futures not block in poll?**

> 

**How do wakeups schedule another poll without carrying the result themselves?**

> 

**Where must waker registration and completion state be synchronized to avoid missed wakeups?**

> 

**How does thread::unpark avoid a wake-before-park race?**

> 

**Which responsibilities belong to an executor, a reactor, and a future?**

> 

**Why does pinning matter after polling begins, and which values can safely move through Unpin?**

> 

**How can a future cooperate by bounding work per poll?**

>
