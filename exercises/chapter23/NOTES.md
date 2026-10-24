# Chapter 23 notes — Unsafe Code

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · A safe invariant boundary

**Predictions I got wrong, and why:**

- 

**Explain:** State the private invariant, identify each unsafe operation, and explain why all public operations preserve it.

> 

**Edge case I added:**

- 

## l02 · Raw pointer reads with contracts

**Predictions I got wrong, and why:**

- 

**Explain:** List every caller obligation and show how the safe slice wrapper discharges them.

> 

**Edge case I added:**

- 

## l03 · Raw slice reconstruction

**Predictions I got wrong, and why:**

- 

**Explain:** Prove bounds, provenance, alignment, initialization, lifetime, and nonoverlap separately.

> 

**Edge case I added:**

- 

## l04 · A flagged borrow

**Predictions I got wrong, and why:**

- 

**Explain:** Explain lifetime tracking with PhantomData and preserving provenance with pointer map_addr.

> 

**Edge case I added:**

- 

## l05 · Layout, unsafe traits, unions

**Predictions I got wrong, and why:**

- 

**Explain:** State the unsafe trait contract and explain why bool would violate an all-bit-patterns-valid promise.

> 

**Edge case I added:**

- 

## l06 · Moving values and initialization

**Predictions I got wrong, and why:**

- 

**Explain:** Explain why no panicking code may occur between a raw ownership move and updating initialized-length bookkeeping.

> 

**Edge case I added:**

- 

## l07 · A fixed-capacity byte gap buffer

**Predictions I got wrong, and why:**

- 

**Explain:** State logical length, cursor position, and the gap invariant. Explain why all storage remains initialized in this byte-only version.

> 

**Edge case I added:**

- 

## l08 · Capstone: initialized-prefix storage

**Predictions I got wrong, and why:**

- 

**Explain:** Write the invariant len<=N and exactly data[..len] initialized. Audit push, pop, and provided cleanup for normal returns and panics.

> 

**Edge case I added:**

- 

## r01 · Spaced retrieval: Chapters 17, 21, and 22 inside Chapter 23

**Predictions I got wrong, and why:**

- 

**Edge case I added:**

- 

## c01 · A teammate's unsafe pull request

**My review, written before running the tests:**

- 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**Which safety guarantees remain the programmer’s responsibility inside unsafe blocks?**

> 

**How do an unsafe function contract and a safe abstraction invariant differ?**

> 

**What makes raw pointer reads valid: provenance, alignment, bounds, initialization, lifetime, and aliasing?**

> 

**When must raw moves change ownership bookkeeping to prevent a double drop?**

> 

**How do size, alignment, padding, and valid bit patterns constrain memory reinterpretation?**

> 

**How does a gap buffer maintain disjoint initialized regions?**

> 

**What must stay true if a user-provided operation panics?**

> 

**What do tests and Miri establish, and which universal safety claims still need a written proof?**

>
