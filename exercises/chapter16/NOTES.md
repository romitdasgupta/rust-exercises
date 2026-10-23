# Chapter 16 notes — Collections

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Vector access and structural editing

**Predictions I got wrong, and why:**

- 

**Explain:** Explain length versus capacity and how each selected operation changes both ownership and ordering.

> 

**Edge case I added:**

- 

## l02 · Growth, removal, and borrow validity

**Predictions I got wrong, and why:**

- 

**Explain:** Why does Rust reject potential invalidation without proving whether this particular push reallocates? How does splitting prove disjoint mutable access?

> 

**Edge case I added:**

- 

## l03 · Sorting, binary search, and slices

**Predictions I got wrong, and why:**

- 

**Explain:** Why does stable sort preserve equal-key ordering? When can an unstable sort be preferable?

> 

**Edge case I added:**

- 

## l04 · Queues and ring buffers

**Predictions I got wrong, and why:**

- 

**Explain:** Why can VecDeque support efficient front removal where Vec shifts elements? When is make_contiguous useful?

> 

**Edge case I added:**

- 

## l05 · Priority queues and heap ordering

**Predictions I got wrong, and why:**

- 

**Explain:** Why should you pop to obtain priority order? How does Reverse adapt ordering without writing a second collection?

> 

**Edge case I added:**

- 

## l06 · Hash maps, tree maps, and entries

**Predictions I got wrong, and why:**

- 

**Explain:** When does a BTreeMap range query justify ordered keys? How can borrowed lookup avoid temporary String allocation?

> 

**Edge case I added:**

- 

## l07 · Set algebra and equal-but-distinct values

**Predictions I got wrong, and why:**

- 

**Explain:** Why must equality and hashing use the same identity fields? Why is mutating an element’s identity while stored a logic error?

> 

**Edge case I added:**

- 

## l08 · Hashing and custom builders

**Predictions I got wrong, and why:**

- 

**Explain:** Why does a collision not change map correctness? Why would this deliberately weak teaching hasher be inappropriate for untrusted keys?

> 

**Edge case I added:**

- 

## l09 · Capstone: a task scheduler and audit

**Predictions I got wrong, and why:**

- 

**Explain:** What complexity do heap insertion/pop and hash lookup suggest? Which output order must be made explicit?

> 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**Choose Vec, VecDeque, BinaryHeap, hash, or tree collections by required operations.**

> 

**Predict which operations preserve order, invalidate references, or may reallocate.**

> 

**Use Entry to update maps and produce deterministic output without assuming hash iteration order.**

> 

**Keep Eq and Hash consistent, including equal keys with different payloads.**

> 

**Distinguish safe deterministic random tests from brittle hard-coded random sequences.**

> 

**Redo the capstone from its contract after a few days without consulting your earlier implementation.**

> 

**Present your predictions, failed compiler experiments, and one new edge case for review.**

>
