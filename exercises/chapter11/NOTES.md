# Chapter 11 notes — Traits and Generics

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Traits, implementations, and defaults

**Predictions I got wrong, and why:**

- 

**Explain:** Which method does your implementation supply, and which comes from the trait? Why can two unrelated types implement one interface?

> 

**Edge case I added:**

- 

## l02 · Static and dynamic dispatch

**Predictions I got wrong, and why:**

- 

**Explain:** Which call sites require a vtable, which can be specialized, and why does ?Sized matter for dyn Write?

> 

**Edge case I added:**

- 

## l03 · Self, supertraits, and associated items

**Predictions I got wrong, and why:**

- 

**Explain:** Why does Resettable require both the interface of Named and a sized constructor? Distinguish an associated function from a method.

> 

**Edge case I added:**

- 

## l04 · Extension traits and qualified calls

**Predictions I got wrong, and why:**

- 

**Explain:** How do coherence and the orphan rule preserve unambiguous implementations across crates? Why can importing two traits make method syntax ambiguous?

> 

**Edge case I added:**

- 

## l05 · Associated types and generic traits

**Predictions I got wrong, and why:**

- 

**Explain:** When should callers select a trait parameter, and when should the implementation fix an associated type?

> 

**Edge case I added:**

- 

## l06 · Opaque results, lifetimes, and precise capture

**Predictions I got wrong, and why:**

- 

**Explain:** Explain every bound in these signatures. Why does use<'a> list captures while + 'a expresses an outlives requirement?

> 

**Edge case I added:**

- 

## l07 · Capstone: a generic validation pipeline

**Predictions I got wrong, and why:**

- 

**Explain:** Why can rules be heterogeneous while the Source associated type stays fixed? Could Rule include a generic method and still support these trait objects?

> 

**Edge case I added:**

- 

## r01 · Spaced retrieval: Chapters 7, 9, and 10 inside Chapter 11

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

**Explain a trait object’s data/vtable pair and a generic function’s monomorphization.**

> 

**Design a dyn-compatible interface, then explain which methods would break compatibility.**

> 

**Choose associated types versus a generic trait parameter and justify each bound.**

> 

**Explain Self, supertraits, associated constants, extension traits, and coherence.**

> 

**Demonstrate how return-position impl Trait captures lifetimes in edition 2024.**

> 

**Redo the capstone from its contract after a few days without consulting your earlier implementation.**

> 

**Present your predictions, failed compiler experiments, and one new edge case for review.**

>
