# Chapter 13 notes — Utility Traits

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Destruction and resource lifetimes

**Predictions I got wrong, and why:**

- 

**Explain:** Explain why calling .drop() directly is prohibited and why std::mem::drop is an ordinary generic function. When does forgetting a value leak resources?

> 

**Edge case I added:**

- 

## l02 · Sized, Clone, and Copy

**Predictions I got wrong, and why:**

- 

**Explain:** Why is Copy a promise about implicit duplication rather than a customization hook? Which fields prevent a struct from implementing Copy?

> 

**Edge case I added:**

- 

## l03 · Deref, DerefMut, and Default

**Predictions I got wrong, and why:**

- 

**Explain:** When is a transparent wrapper a suitable Deref design? Why should a domain object avoid surprising deref-based behavior?

> 

**Edge case I added:**

- 

## l04 · AsRef and AsMut views

**Predictions I got wrong, and why:**

- 

**Explain:** Why can an API accept multiple owned and borrowed input shapes through one cheap conversion?

> 

**Edge case I added:**

- 

## l05 · Borrow and borrowed lookup

**Predictions I got wrong, and why:**

- 

**Explain:** Explain the stronger equality/order/hash contract of Borrow compared with AsRef. What does BorrowMut add?

> 

**Edge case I added:**

- 

## l06 · Infallible and fallible conversions

**Predictions I got wrong, and why:**

- 

**Explain:** Why should From avoid failure and loss of value? Where does TryFrom communicate validation failure?

> 

**Edge case I added:**

- 

## l07 · ToOwned and clone-on-write

**Predictions I got wrong, and why:**

- 

**Explain:** Explain why ToOwned generalizes Clone for unsized borrowed data. Distinguish Cow::into_owned from Cow::to_mut.

> 

**Edge case I added:**

- 

## l08 · Capstone: validated names with cheap views

**Predictions I got wrong, and why:**

- 

**Explain:** Which utility traits would be misleading for this type? Why does AsRef expose a view while TryFrom performs validation?

> 

**Edge case I added:**

- 

## r01 · Spaced retrieval: Chapters 9, 11, and 12 inside Chapter 13

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

**Predict destruction order, including explicit drop, moves, and unwinding.**

> 

**Explain why Copy forbids Drop and how Clone can do real work.**

> 

**Use ?Sized when an API need not require statically sized values.**

> 

**Distinguish cheap reference views, identity-preserving borrows, and owned conversions.**

> 

**Explain where Cow avoids allocation and where it must allocate.**

> 

**Redo the capstone from its contract after a few days without consulting your earlier implementation.**

> 

**Present your predictions, failed compiler experiments, and one new edge case for review.**

>
