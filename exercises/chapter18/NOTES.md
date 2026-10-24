# Chapter 18 notes — Input and Output

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Streaming bytes

**Predictions I got wrong, and why:**

- 

**Explain:** Why use Read rather than File in the contract? Explain EOF versus Interrupted.

> 

**Edge case I added:**

- 

## l02 · Buffered text

**Predictions I got wrong, and why:**

- 

**Explain:** Why can byte records succeed where text lines return InvalidData?

> 

**Edge case I added:**

- 

## l03 · Writers and flushing

**Predictions I got wrong, and why:**

- 

**Explain:** Why should a function explicitly propagate flush failure?

> 

**Edge case I added:**

- 

## l04 · Seeking and binary formats

**Predictions I got wrong, and why:**

- 

**Explain:** Explain why field sizes and byte order belong to a format contract.

> 

**Edge case I added:**

- 

## l05 · Paths preserve OS data

**Predictions I got wrong, and why:**

- 

**Explain:** Distinguish lexical path processing from filesystem canonicalization and security containment.

> 

**Edge case I added:**

- 

## l06 · Directory metadata

**Predictions I got wrong, and why:**

- 

**Explain:** Why should a listing specify symlink and ordering policies?

> 

**Edge case I added:**

- 

## l07 · Capstone: a bounded record service

**Predictions I got wrong, and why:**

- 

**Explain:** Explain framing, length validation before allocation, error propagation, and when the response is flushed.

> 

**Edge case I added:**

- 

## r01 · Spaced retrieval: earlier chapters inside Chapter 18

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

**What may a successful Read::read or Write::write return before the buffer is exhausted?**

> 

**When does read_line append, and how do lines() and split() differ?**

> 

**How do Cursor, BufReader, Take, Chain, and Seek compose?**

> 

**Why must paths and file names not be forced through String?**

> 

**Which error kinds should retry, propagate, or end a stream?**

> 

**How do you frame messages on TCP, and why does a single read not correspond to a whole message?**

>
