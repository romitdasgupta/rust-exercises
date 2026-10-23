# Chapter 12 notes — Operator Overloading

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Unary, binary, and assignment operators

**Predictions I got wrong, and why:**

- 

**Explain:** Why are assignment traits separate? What are the move semantics of each receiver in these implementations?

> 

**Edge case I added:**

- 

## l02 · Equivalence and NaN

**Predictions I got wrong, and why:**

- 

**Explain:** List reflexivity, symmetry, and transitivity. Why must Eq not be added to a wrapper whose equality follows NaN behavior?

> 

**Edge case I added:**

- 

## l03 · Partial and total comparisons

**Predictions I got wrong, and why:**

- 

**Explain:** Why is lexicographic tie-breaking part of the public contract? Why must PartialOrd delegate to Ord for a totally ordered type?

> 

**Edge case I added:**

- 

## l04 · Index and IndexMut

**Predictions I got wrong, and why:**

- 

**Explain:** Which trait enables reads and which enables writes? Why is returning a reference to a temporary incorrect?

> 

**Edge case I added:**

- 

## l05 · Capstone: a small bit-set API

**Predictions I got wrong, and why:**

- 

**Explain:** Why does Eq remain consistent with derived Hash here? Why is indexing a mask less natural than asking contains(bit)?

> 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**Explain operand ownership and why Add<Rhs> has an associated Output.**

> 

**Keep PartialEq, Eq, PartialOrd, and Ord consistent, including hashing implications.**

> 

**Explain why floats cannot promise Eq or a natural total order.**

> 

**Implement indexing that returns a reference and assignment that mutates in place.**

> 

**Name operators that Rust does not allow user code to overload.**

> 

**Redo the capstone from its contract after a few days without consulting your earlier implementation.**

> 

**Present your predictions, failed compiler experiments, and one new edge case for review.**

>
