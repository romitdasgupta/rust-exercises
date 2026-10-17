# Chapter 20 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try hint 1 before opening hint 2.

## l01 · Lazy futures and await

<details><summary>Hint 1</summary>

An async function body is run by polling its returned future, not by constructing it.

</details>

<details><summary>Hint 2</summary>

No executor is needed for these immediately ready futures in the supplied single-poll test helper. Use .await separately for the left and right values.

</details>

## l02 · Borrowed and owned futures

<details><summary>Hint 1</summary>

The returned future type must advertise the lifetime of any borrowed inputs.

</details>

<details><summary>Hint 2</summary>

async move captures String ownership in B, but only captures the borrowed &str in A. Neither requires cloning.

</details>

## l03 · Async behavior in traits

<details><summary>Hint 1</summary>

Keep the trait generic for the core exercise. The compiler creates a concrete future type for each implementation.

</details>

<details><summary>Hint 2</summary>

Search borrowed entries, copy the u32 value, and await each lookup. Boxing is a separate design tradeoff in the compiler lab.

</details>

## l04 · Lending async callbacks

<details><summary>Hint 1</summary>

Await one callback result before borrowing the callback again.

</details>

<details><summary>Hint 2</summary>

Use an async closure directly: async |v| { ... }. Its future may borrow the captured mutable accumulator.

</details>

## l05 · Cancellation by dropping

<details><summary>Hint 1</summary>

The suspended future stores any local values needed after the await. Dropping the future drops those stored values.

</details>

<details><summary>Hint 2</summary>

Read ticket.id after awaiting to keep the ticket live. Mutate the total only after the optional value is ready.

</details>

## l06 · Polling a race

<details><summary>Hint 1</summary>

poll_fn lets a closure implement one poll step. The closure must not move a pinned future out.

</details>

<details><summary>Hint 2</summary>

Pin both inputs locally, poll left via as_mut, return its Ready immediately, otherwise poll right using the same Context.

</details>

## l07 · Capstone: asynchronous graph traversal

<details><summary>Hint 1</summary>

Maintain a queue of discovered names and a set of names already expanded.

</details>

<details><summary>Hint 2</summary>

Insert into the visited set when popping; skip a duplicate before looking it up. Propagate lookup errors with ?. Sort by collecting a BTreeSet.

</details>

