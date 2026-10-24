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

<details><summary>Check your explanation (after writing your own)</summary>

Calling an async function runs none of its body: it captures the arguments into a new future value (a state machine) and returns it. The body starts on the first `poll`, runs until it reaches an `.await` on something that returns `Pending` (or until it finishes), and each later poll resumes from the saved point. Inside one async body, `a.await; b.await` is strictly sequential: `b` is not even created until `a` completes, so two awaits never overlap. Running futures concurrently needs a combinator (join, select) or a runtime spawn. A future that is dropped before its first poll never runs at all.

</details>

## l02 · Borrowed and owned futures

<details><summary>Hint 1</summary>

The returned future type must advertise the lifetime of any borrowed inputs.

</details>

<details><summary>Hint 2</summary>

async move captures String ownership in B, but only captures the borrowed &str in A. Neither requires cloning.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

A future that borrows its input carries that lifetime in its type (`impl Future + 'a`), so it cannot outlive the borrowed `&str`, and it can even return a subslice of it. `async move` moves whatever the block captures into the future; if the captured thing is a reference, only the reference moves, and the referent still belongs to the caller, so the future is still bounded by that borrow. To make a future `'static` (as a spawned task requires) it must own its data: move the `String` itself in, or share it through an `Arc`. Owning the data also makes `Send` depend only on what the future stores across awaits.

</details>

## l03 · Async behavior in traits

<details><summary>Hint 1</summary>

Keep the trait generic for the core exercise. The compiler creates a concrete future type for each implementation.

</details>

<details><summary>Hint 2</summary>

Search borrowed entries, copy the u32 value, and await each lookup. Boxing is a separate design tradeoff in the compiler lab.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

Each implementation of an async trait method produces its own anonymous future type, so `total<L: Lookup>` is monomorphized per table type and every `lookup(...).await` polls a concrete state machine stored inline, with no heap allocation. That is also why such a trait is not dyn-compatible: `dyn Lookup` would need one return type for all implementations. Boxing (`Pin<Box<dyn Future<Output = _> + Send + 'a>>`) erases the type, which buys trait objects, heterogeneous collections, and recursion, at the cost of an allocation and dynamic dispatch per call. Also note that an async trait method promises nothing about `Send`; callers that need it must ask for it, for example by declaring the method as `fn lookup(..) -> impl Future<Output = _> + Send` or by returning a boxed `+ Send` future.

</details>

## l04 · Lending async callbacks

<details><summary>Hint 1</summary>

Await one callback result before borrowing the callback again.

</details>

<details><summary>Hint 2</summary>

Use an async closure directly: async |v| { ... }. Its future may borrow the captured mutable accumulator.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

An async closure's call can return a future that *borrows the closure itself*, including its captured state, so `total += v` can live inside the returned future. Because each call lends `&mut` access to the captures, the bound is `AsyncFnMut`: one call's future must be finished (awaited) before the next call can borrow the closure again, which is exactly what a sequential `for` loop with `.await` does. A plain `FnMut() -> Fut` cannot express this: `Fut` is one type chosen before any call, so it cannot borrow from the closure for the duration of a call; that is the "captured variable cannot escape" error. `AsyncFn` lends shared access and `AsyncFnOnce` may consume the captures.

</details>

## l05 · Cancellation by dropping

<details><summary>Hint 1</summary>

The suspended future stores any local values needed after the await. Dropping the future drops those stored values.

</details>

<details><summary>Hint 2</summary>

Read ticket.id after awaiting to keep the ticket live. Mutate the total only after the optional value is ready.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

A future owns everything it captured from the start, plus the locals it is holding at its current `.await`. Dropping a never-polled future drops only the captures; no code in the body runs, so nothing created inside the body exists yet. Dropping a suspended future drops the captures and the locals live at that await (in place, like any other drop), so RAII guards still release their resources, but any code after the await never runs. That is the cancellation-safety rule: do not leave state half-updated across an await; compute first, then mutate after the last await, or use a guard that rolls back in `Drop`.

</details>

## l06 · Polling a race

<details><summary>Hint 1</summary>

poll_fn lets a closure implement one poll step. The closure must not move a pinned future out.

</details>

<details><summary>Hint 2</summary>

Pin both inputs locally, poll left via as_mut, return its Ready immediately, otherwise poll right using the same Context.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

A race polls both futures in a fixed order each time it is polled, returns the first `Ready`, and returns `Pending` only after both returned `Pending` (each having arranged its own wake). When the race future completes or is dropped, it drops whichever branch is still stored inside it, which is how the loser is cancelled. This model is left-biased by construction, so a constantly ready left branch starves the right; `tokio::select!` randomizes the order by default (or is `biased;` on request), supports patterns and many branches, and lives inside a runtime with timers and task aborts, none of which a two-future `poll_fn` covers. Note also that `poll_fn` has no memory of finishing, so a combinator must not poll a completed branch again.

</details>

## l07 · Capstone: asynchronous graph traversal

<details><summary>Hint 1</summary>

Maintain a queue of discovered names and a set of names already expanded.

</details>

<details><summary>Hint 2</summary>

Insert into the visited set when popping; skip a duplicate before looking it up. Propagate lookup errors with ?. Sort by collecting a BTreeSet.

</details>

<details><summary>Check your explanation (after writing your own)</summary>

The core is a sequential breadth-first loop: pop a node, skip it if already visited, await its neighbors, queue the unvisited ones. `async` here only lets the lookups suspend; nothing runs concurrently, because each `.await` finishes before the next lookup starts. The visited set is what guarantees termination on cycles and one lookup per node. A concurrent version would spawn lookups into a `JoinSet`, keep at most N in flight (admit a new one only when one finishes), give each task an owned key and an `Arc` of shared configuration (spawned tasks must be `'static` and usually `Send`), handle both application errors and `JoinError`s for every task, and on the first error (or a shutdown signal) stop admitting work and abort or drain the remaining tasks so that none is left detached.

</details>

## R01 · Spaced review of Chapters 13 and 17–19

<details><summary>Hint 1</summary>

A: `Pin<Box<dyn Future + Send>>` is `Send` and `'static`, so it can move into `thread::spawn`, and `as_mut().poll(&mut cx)` needs no extra pinning. B: an `async fn` taking `&str` and returning `Option<&str>` ties the output to the input's lifetime, so you can return a word from `split_whitespace()` directly. C: a struct holding `&mut Vec<u32>`, the original length, and a `committed` flag can truncate in its `Drop`.

</details>
<details><summary>Hint 2</summary>

A: spawn all threads first, then join in order. B: track `Option<(&str, usize)>` and replace it only on a strictly greater `chars().count()`. C: build the guard, push through `guard.log`, await the pause, then set `committed = true`; the guard is dropped at the end of the body either way, and only an uncommitted guard rolls back.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: a rejected transfer loses money, and `save_all` never saves (read the compiler warning). Then look at what `get_or_load` is still holding when it reaches `.await`, and at what `transfer` has already changed when it is suspended.

</details>
<details><summary>Hint 2</summary>

In `get_or_load`, let the guard from the first lookup drop before awaiting `load`, then lock again to insert (`entry(..).or_insert(value)` handles a racing insert). In `transfer`, await approval first and mutate both balances only afterwards. In `save_all`, `.await` each `save` and return `items.len()`.

</details>
<details><summary>Check your review</summary>

`get_or_load` holds a `std::sync::MutexGuard` across `load.await`, so every other task touching the cache blocks (or deadlocks on a single-threaded executor), and the future is not `Send`; clippy reports this as `await_holding_lock`. `transfer` debits before the await: a rejection never refunds, and cancelling the future while approval is pending leaves the money debited but never credited. `save_all` creates each `save` future and drops it unpolled, so nothing is stored while it still reports success; the `unused_must_use` warning ("futures do nothing unless you `.await` or poll them") pointed straight at it. All three are the same lesson: an `.await` is a point where the function may be paused for a long time or never resumed.

</details>

## Drills

Run `cargo drill ch20`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · await outside async</summary>E0728: `.await` can only appear inside an `async fn` or `async` block, because only those are compiled into state machines that can pause. Make `describe` an `async fn` and await it (or pass its future to `run`).</details>
<details><summary>d02 · Calling an async fn without awaiting</summary>E0369: `base_price()` is a future (`impl Future<Output = u32>`), not a `u32`, so `+` is not defined. Write `base_price().await + 2`.</details>
<details><summary>d03 · A future that borrows a local</summary>E0373: the async block borrows `name`, which is dropped when `greeting` returns, but the returned future lives longer. `async move` moves the String into the future.</details>
<details><summary>d04 · Two async blocks in one Vec</summary>E0308: every async block has its own anonymous type, even when the bodies look the same, so they cannot share a `Vec`. Erase the types: `Vec<Pin<Box<dyn Future<Output = i32>>>>` with `Box::pin(async { 1 })`, or call one async fn twice.</details>
<details><summary>d05 · Rc held across an await</summary>Error without a code: "future cannot be sent between threads safely". The `Rc` is alive at the `.await`, so it is stored inside the future, which makes the whole future `!Send`. Use `Arc`, or end the `Rc`'s scope before the await.</details>
<details><summary>d06 · A future nobody polls</summary>Compiles and runs: futures are lazy. The async block borrows `sent` but its body never runs, because nothing polls it; dropping it just releases the borrow. Code inside an async block runs only when something drives it.</details>
<details><summary>d07 · Polling a finished future again</summary>Panics at run time: "`async fn` resumed after completion". After a future returns `Ready`, its contract says it must not be polled again; compiler-generated futures panic. Stop polling after `Ready`, or wrap it in a fused future if a combinator might poll again.</details>
<details><summary>d08 · A trait object with an async method</summary>E0038: a trait with an `async fn` is not dyn compatible, because each implementation returns a different future type. Use generics (`Box<Memory>` or `impl Store`), or define an object-safe trait that returns `Pin<Box<dyn Future<Output = Option<u32>> + '_>>`.</details>
<details><summary>d09 · async move takes ownership</summary>E0382: `async move` moved `report` into the future, so `println!` uses a moved value. Print first, borrow with a plain `async` block (the future then borrows `report`), or clone before moving.</details>
<details><summary>d10 · A closure whose future borrows its state</summary>Error without a code: "captured variable cannot escape `FnMut` closure body". The future returned by `|| async { calls += 1 }` would keep borrowing the closure's `&mut calls` after the call returns, which `FnMut() -> Fut` cannot express. Take `impl AsyncFnMut()` and pass an async closure: `async || { calls += 1; }`.</details>
