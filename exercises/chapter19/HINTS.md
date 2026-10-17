# Chapter 19 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try hint 1 before opening hint 2.

## l01 · Fork, join, and worker failures

<details><summary>Hint 1</summary>

Collect join handles before joining, so all workers can make progress.

</details>

<details><summary>Hint 2</summary>

The join result is Result<Result<u32, &str>, panic payload>. Map only the outer error, then return the inner result.

</details>

## l02 · Borrowed parallel work

<details><summary>Hint 1</summary>

Split before spawning; the compiler can prove the halves do not overlap.

</details>

<details><summary>Hint 2</summary>

thread::scope allows non-static borrows. Use scoped handles for returned sums, and mutate each half directly in separate closures.

</details>

## l03 · Ownership and channel closure

<details><summary>Hint 1</summary>

An iterator over a receiver ends only after all sender handles have been dropped and buffered messages consumed.

</details>

<details><summary>Hint 2</summary>

Use an explicit channel::<String>. In drain_now match TryRecvError variants without retrying Empty.

</details>

## l04 · Send and Sync contracts

<details><summary>Hint 1</summary>

Send is about moving a value across threads; Sync is about sharing references across threads.

</details>

<details><summary>Hint 2</summary>

Arc cloning changes ownership counts, not Vec contents. Clone before spawning and join both workers before returning the original.

</details>

## l05 · Mutex invariants and poisoning

<details><summary>Hint 1</summary>

Keep a guard only while modifying the invariant. PoisonError can return the guard for deliberate recovery.

</details>

<details><summary>Hint 2</summary>

Use into_inner on the poison error, overwrite the entire protected state, then clear_poison. Joining workers establishes completion before reading.

</details>

## l06 · RwLock and condition predicates

<details><summary>Hint 1</summary>

The mutex-protected Option is the state; the Condvar only helps wait efficiently. Tests begin with data available so a broken implementation cannot silently depend on wake ordering.

</details>

<details><summary>Hint 2</summary>

while guard.is_none() { guard = cv.wait(guard).unwrap(); }. After the loop, take the value. The manual lab exercises a real waiting consumer.

</details>

## l07 · Atomic counters and local state

<details><summary>Hint 1</summary>

try_update (called fetch_update on older toolchains) accepts a checked update function and returns the old value on success.

</details>

<details><summary>Hint 2</summary>

Use Cell inside thread_local!, with initialization inside each fresh worker thread. TLS is not shared mutation of one counter.

</details>

## l08 · Capstone: bounded parallel pipeline

<details><summary>Hint 1</summary>

A bounded sender can block until the receiver drains. Drop the original sender before collecting to EOF.

</details>

<details><summary>Hint 2</summary>

Spawn workers, consume rx inside the scope, then join workers. Tag values with input indices to reconstruct order without guessing a schedule.

</details>

