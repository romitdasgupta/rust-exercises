# Chapter 21 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try hint 1 before opening hint 2.

## l01 · Implement Future

<details><summary>Hint 1</summary>

Pin restricts moving the pointee; the smart-pointer handle itself can move.

</details>

<details><summary>Hint 2</summary>

get_mut is available because Once<T> is Unpin when T is Unpin. Take from Option rather than cloning the output.

</details>

## l02 · Pending and wakeups

<details><summary>Hint 1</summary>

A waker is a scheduling notification; result storage belongs elsewhere.

</details>

<details><summary>Hint 2</summary>

Use cx.waker().wake_by_ref on the pending path only. Atomic fetch_add returns an old value that you can discard.

</details>

## l03 · A completion cell

<details><summary>Hint 1</summary>

The state and waiter registration must be inspected atomically under one mutex. Never wait on that mutex for future completion.

</details>

<details><summary>Hint 2</summary>

Store a clone of the latest waker. Move the waker out while locked, then invoke it only after the lock guard is dropped.

</details>

## l04 · A parker block_on

<details><summary>Hint 1</summary>

thread::unpark stores a token if the thread has not parked yet. Always poll again after a park return.

</details>

<details><summary>Hint 2</summary>

Use pin! on the stack. A poll_fn closure can capture a yielded bool and an Option<T> that is taken only on the second poll.

</details>

## l05 · Readiness registration model

<details><summary>Hint 1</summary>

The ready flag stores the event, not the waker. This toy event has one waiter and is never reset.

</details>

<details><summary>Hint 2</summary>

Never leave a window between checking ready and installing the waker. Actual I/O also retries the operation and handles WouldBlock.

</details>

## l06 · Cooperative work budgets

<details><summary>Hint 1</summary>

A Future does not automatically make CPU work cooperative. Count elements processed in one poll.

</details>

<details><summary>Hint 2</summary>

Compute an end index bounded by both budget and slice length. Wake only when work remains after consuming the budget.

</details>

## l07 · Capstone: pinned future adapter

<details><summary>Hint 1</summary>

Pin<Box<F>> is itself a movable handle. Moving the wrapper does not move F inside its heap allocation.

</details>

<details><summary>Hint 2</summary>

get_mut on the wrapper is safe because its fields can move; then call inner.as_mut().poll(cx). Do not take the tag until completion.

</details>

