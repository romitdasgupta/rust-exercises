# Chapter 10 notes — Enums and Patterns

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Variants and payloads

**Predictions I got wrong, and why:**

- 

**Explain:** Explain the discriminant, payload, exhaustive match, and why default enum layout is not a stable serialization format.

> 

**Edge case I added:**

- 

## l02 · Recursive data with enums

**Predictions I got wrong, and why:**

- 

**Explain:** Explain finite sized representation, ownership of child nodes, and structural recursion over a tagged tree.

> 

**Edge case I added:**

- 

## l03 · Generic enums

**Predictions I got wrong, and why:**

- 

**Explain:** Explain which type arguments change when variants are exchanged and why matching can transfer ownership without bounds.

> 

**Edge case I added:**

- 

## l04 · Literal, tuple and struct patterns

**Predictions I got wrong, and why:**

- 

**Explain:** Explain shadowing in patterns, ignored fields, and named versus positional destructuring.

> 

**Edge case I added:**

- 

## l05 · Matching references and mutating payloads

**Predictions I got wrong, and why:**

- 

**Explain:** Explain match ergonomics, explicit reference patterns, and the difference between copying a small field and moving an owned payload.

> 

**Edge case I added:**

- 

## l06 · Array and slice patterns

**Predictions I got wrong, and why:**

- 

**Explain:** Explain fixed array length in a type versus a runtime slice shape and the borrowed rest binding.

> 

**Edge case I added:**

- 

## l07 · Guards, alternatives and @ bindings

**Predictions I got wrong, and why:**

- 

**Explain:** Explain the order of pattern matching and guard evaluation, and why @ can preserve a matched subvalue for later use.

> 

**Edge case I added:**

- 

## l08 · Pattern contexts and public compatibility

**Predictions I got wrong, and why:**

- 

**Explain:** List pattern contexts: let, function parameters, for, match, if let, while let, and let else. Explain when each permits refutable patterns.

> 

**Edge case I added:**

- 

## l09 · Capstone: generic ordered tree

**Predictions I got wrong, and why:**

- 

**Explain:** Explain recursive mutable matching, why every child needs indirection, and how inorder traversal proves the ordering invariant.

> 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**Represent mutually exclusive states with variants and choose payload ownership deliberately.**

> 

**Explain recursive indirection and avoid assumptions about default enum layout.**

> 

**Destructure owned, borrowed, and mutably borrowed variants and track moves.**

> 

**Use slice patterns, guards, or-patterns and @ without introducing accidental catch-all bindings.**

> 

**Explain exhaustiveness, the role of wildcard arms, and non_exhaustive public API evolution.**

> 

**Build and query a recursive binary tree using patterns and compare its worst-case shape.**

> 

**Redo the final capstone after a break without looking at your implementation; invent a changed requirement and explain the tradeoff.**

>
