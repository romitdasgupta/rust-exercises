# Chapter 15 notes — Iterators

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Borrowed, mutable, owned, and draining sources

**Predictions I got wrong, and why:**

- 

**Explain:** Why can reading and editing use different iterator types over the same container? Who owns elements returned by drain?

> 

**Edge case I added:**

- 

## l02 · Generating finite sequences

**Predictions I got wrong, and why:**

- 

**Explain:** How do you guarantee termination and avoid overflow at the final step?

> 

**Edge case I added:**

- 

## l03 · Mapping, filtering, and flattening

**Predictions I got wrong, and why:**

- 

**Explain:** What values vanish in each pipeline? Why does flatten on Option discard None?

> 

**Edge case I added:**

- 

## l04 · Stopping, peeking, and resuming

**Predictions I got wrong, and why:**

- 

**Explain:** Why does by_ref permit continued use of the original iterator? What guarantee does fuse add?

> 

**Edge case I added:**

- 

## l05 · Reversal, pairing, inspection, and repetition

**Predictions I got wrong, and why:**

- 

**Explain:** Which adapters require DoubleEndedIterator or Clone? Why is inspect useful for observing a lazy pipeline?

> 

**Edge case I added:**

- 

## l06 · Accumulators, extrema, and sequence comparisons

**Predictions I got wrong, and why:**

- 

**Explain:** Why must an extrema function specify tie behavior? How do max_by_key and max_by differ?

> 

**Edge case I added:**

- 

## l07 · Searching and iterator positions

**Predictions I got wrong, and why:**

- 

**Explain:** Explain consuming search versus indexing. Why must a reverse position be reported in original index coordinates?

> 

**Edge case I added:**

- 

## l08 · Folds, short circuits, and side effects

**Predictions I got wrong, and why:**

- 

**Explain:** Why should a fallible consumer stop immediately? How do rfold and try_rfold change processing direction?

> 

**Edge case I added:**

- 

## l09 · Collections, extension, and partition

**Predictions I got wrong, and why:**

- 

**Explain:** Explain FromIterator versus Extend. Why does partition require two output collections of compatible type?

> 

**Edge case I added:**

- 

## l10 · Implementing a double-ended iterator

**Predictions I got wrong, and why:**

- 

**Explain:** Why does ExactSizeIterator promise more than a useful hint? Why should an empty iterator remain empty?

> 

**Edge case I added:**

- 

## l11 · Capstone: a lazy measurement report

**Predictions I got wrong, and why:**

- 

**Explain:** Why must enumerate precede trimming/filtering to preserve original source numbering? Show where each consumer starts execution.

> 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**Explain laziness and the precise ownership of Item for iter, iter_mut, and into_iter.**

> 

**Choose adapters and consumers while predicting short-circuit and consumption behavior.**

> 

**Implement Iterator, DoubleEndedIterator, ExactSizeIterator, and FusedIterator consistently.**

> 

**Use by_ref and peekable without accidentally consuming delimiters.**

> 

**Explain why iterator pipelines need not allocate intermediate collections.**

> 

**Redo the capstone from its contract after a few days without consulting your earlier implementation.**

> 

**Present your predictions, failed compiler experiments, and one new edge case for review.**

>
