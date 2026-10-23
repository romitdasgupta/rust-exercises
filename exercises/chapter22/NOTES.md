# Chapter 22 notes — Macros

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Expansion and evaluation once

**Predictions I got wrong, and why:**

- 

**Explain:** Trace expansion and evaluation separately. Explain why syntax substitution can change control flow.

> 

**Edge case I added:**

- 

## l02 · Repetition and separators

**Predictions I got wrong, and why:**

- 

**Explain:** Explain nesting/separator syntax and the type-inference issue for an empty map.

> 

**Edge case I added:**

- 

## l03 · Type and identifier fragments

**Predictions I got wrong, and why:**

- 

**Explain:** Explain which tokens the macro parses and which rules the Rust compiler still checks after expansion.

> 

**Edge case I added:**

- 

## l04 · Recursive structured literals

**Predictions I got wrong, and why:**

- 

**Explain:** Explain the recursive grammar and its deliberate limitations (negative/compound expressions must be grouped).

> 

**Edge case I added:**

- 

## l05 · Traits handle macro leaf types

**Predictions I got wrong, and why:**

- 

**Explain:** Explain syntax/type separation and the trait bound error for an unsupported leaf.

> 

**Edge case I added:**

- 

## l06 · Exports and hygiene

**Predictions I got wrong, and why:**

- 

**Explain:** Explain $crate resolution and the different roles of hygienic locals and referenced item paths.

> 

**Edge case I added:**

- 

## l07 · Capstone: structured events

**Predictions I got wrong, and why:**

- 

**Explain:** Specify the event grammar, evaluation order, exported-helper paths, and diagnostics for unsupported syntax.

> 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**How does macro expansion differ from calling a function, especially for evaluation count?**

> 

**Which fragment types fit expressions, types, identifiers, token trees, and patterns?**

> 

**Where can repetition be empty, and how do trailing commas affect matching?**

> 

**How do recursive macro rules and traits divide syntax handling from type behavior?**

> 

**Why do exported macros use $crate rather than relying on a caller import?**

> 

**How does macro hygiene handle local variables and referenced items?**

> 

**When does a procedural macro justify a separate proc-macro crate?**

>
