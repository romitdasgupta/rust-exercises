# Runtime-specific async projects

The standard-library lessons practice futures and borrowing. They do not exercise Tokio APIs. These isolated projects cover the chapter's runtime-specific material. Work through the steps in order; each checkpoint tells you what you should see before moving on.

**Setup for every lab.** Create a new temporary Cargo package outside this repository (`cargo new async-lab`), and follow the official Tokio and tokio-util documentation. Keep these dependencies out of the root exercise package. Use only in-memory fixtures or loopback sockets; never crawl public sites.

```sh
cargo add tokio --features rt-multi-thread,macros,sync,time,io-util,net,fs
cargo add tokio-util --features rt   # CancellationToken
```

The resulting `Cargo.toml` section looks like this (keep the versions `cargo add` chose):

```toml
[dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "time", "io-util", "net", "fs"] }
tokio-util = { version = "0.7", features = ["rt"] }
```

**For each project, submit:** your predictions (written before running), runnable source, test output, and a brief explanation of ownership, wakeups, errors, and cancellation.

## Lab 1 · Spawn bounds, outgoing links, and bounded crawling

**Extra dependencies** (in addition to the setup above): a real HTML parser and URL resolution.

```sh
cargo add scraper url
```

**Steps**

1. Write six local HTML fixtures: links that are absolute and relative, a duplicate link, a cycle, a link to a missing page, and one page that nothing links to (disconnected).
2. Extract outgoing links with the HTML parser and resolve each against a loopback base URL such as `http://127.0.0.1:PORT/`.
   - *Checkpoint:* you should now see a unit test listing each fixture's resolved links, with relative links turned into absolute loopback URLs.
3. Serve the fixtures from one loopback HTTP fixture server (a `tokio::net::TcpListener` on port 0 is enough) and fetch them over loopback.
4. Define an async fetch trait with two implementations: the loopback HTTP client and an in-memory transport for tests.
   - *Checkpoint:* you should now see the same crawl result from both transports.
5. Crawl with a Tokio `JoinSet`, a pending queue, and a visited set. Admit at most three in-flight fetches; start a new one only when one finishes. Each spawned task owns an `Arc` of the shared configuration.
6. Collect every successful result *and* every `JoinError` or application error explicitly; nothing is dropped silently. Document your policy for the missing page (skip and report, or fail the crawl).
   - *Checkpoint:* you should now see a sorted list of pages, a list of errors, and an active-request counter whose maximum is at most 3.
7. Compile-only experiments: hold an `Rc` across an `.await` inside a spawned task (the `Send` error), and spawn a future that borrows a local `String` (the `'static` error). Repair each with ownership or `Arc`, or with a borrowed future that is awaited directly instead of spawned.
8. Explain why `async move` alone does not extend a borrowed referent's lifetime.

**Acceptance criteria**

- [ ] Duplicate links are fetched once.
- [ ] The traversal result is sorted deterministically.
- [ ] The active-request counter never exceeds three.
- [ ] The missing-page policy is documented and tested.
- [ ] Every task is joined, and every `JoinError` or application error is collected.
- [ ] Both transports (loopback HTTP and in-memory) pass the same tests.
- [ ] The `Rc`-across-await and borrowed-local-in-spawn diagnostics are recorded and repaired, with the `async move` explanation written down.

## Lab 2 · Waiting versus blocking

**Steps**

1. Build a current-thread Tokio runtime. Start a heartbeat task that records a tick each time it runs, and use channels as start and completion signals.
2. Compute a CPU-bound checksum three ways while the heartbeat runs:
   1. in one async body with no yield;
   2. cooperatively chunked, calling `tokio::task::yield_now().await` between chunks;
   3. inside `spawn_blocking`.
   - *Checkpoint:* you should now see the same checksum all three times, and heartbeat ticks interleaved with versions 2 and 3 but not with version 1.
3. Inspect every worker task's result, including `JoinError`s, rather than unwrapping blindly.
4. Do not use sleep as a correctness assertion. Assert only on signal order; record elapsed time as an observation.
5. Read a file with blocking `std::fs` inside an async task, then with `tokio::fs`. Discuss how `tokio::fs` is implemented and the runtime tradeoff.
   - *Checkpoint:* you should now be able to say which work occupied the single executor thread in each version.

**Acceptance criteria**

- [ ] Identical checksum from all three versions.
- [ ] Worker task errors are inspected.
- [ ] No sleep is used to decide correctness; timing is reported only as an observation.
- [ ] A written explanation of which work occupies the executor thread in each version.
- [ ] A comparison of `std::fs` and `tokio::fs`, including the implementation/runtime tradeoff.

## Lab 3 · select!, state loss, and cancel safety

**Steps**

1. Connect a reader and a controllable writer with `tokio::io::duplex`. Race reading a two-byte header against a shutdown `oneshot` inside `tokio::select!` in a loop.
2. **Unsafe version:** keep the header progress inside a temporary `read_exact` future created in the `select!`, so it is discarded whenever the other branch wins.
3. **Safe version:** keep a persistent byte buffer and progress counter outside `select!`, and read into it with a cancel-safe operation.
4. Drive the interleaving with acknowledgement channels, never timeouts: send the first byte and wait until it is consumed, then release the competing branch, then send the second byte.
   - *Checkpoint:* you should now see the unsafe version lose the first byte, and the safe version keep it.
5. For each operation you use, consult the current Tokio cancellation-safety documentation. Distinguish dropping a `read_exact` future from dropping a persistent future or a single `read`.
6. Test `biased;` order, and test default (random) fairness without asserting a particular random winning sequence (for example, assert that both branches win at least once over many iterations).
7. Add a drop counter to the values held by both branches and inspect which values survive branch cancellation.
   - *Checkpoint:* you should now see a drop for each value owned by a cancelled branch future, and none for state kept outside `select!`.

**Acceptance criteria**

- [ ] No timeout is used to induce an interleaving.
- [ ] Persistent protocol state is explicit, outside `select!`.
- [ ] A test proves the safe design does not lose the consumed first byte.
- [ ] Both the shutdown path and the complete-message path have tests.
- [ ] `biased;` order and default fairness are tested without asserting a random sequence.
- [ ] Drop counters show which values survive branch cancellation.

## Lab 4 · Shutdown signals, aborts, and CancellationToken

**Steps**

1. Run three workers in a `JoinSet`. Each loops on `select!` between a bounded work channel and a child `CancellationToken`.
2. Write the owner's shutdown protocol: stop accepting work, signal cooperative cancellation, drain and join the workers, and report completion.
   - *Checkpoint:* you should now see every worker exit and every handle accounted for.
3. Compare with `JoinHandle::abort`: await the aborted handle and inspect `JoinError::is_cancelled`.
4. Put an RAII drop counter inside each worker. Show which synchronous cleanup runs on cancellation and on abort, and discuss why asynchronous cleanup needs an explicit shutdown protocol instead.
5. Test cancellation in three scenarios, arranged with acknowledgement channels: before the worker starts, while it is blocked on `recv`, and after it completed normally. Use `tokio::time::timeout` only to bound a failing test.
   - *Checkpoint:* you should now see all three scenarios pass deterministically.
6. Explain why aborting a running `spawn_blocking` job does not forcibly stop its closure.

**Acceptance criteria**

- [ ] No detached tasks; all handles are accounted for.
- [ ] All channels are closed at the end.
- [ ] No work is lost beyond what the documented shutdown policy allows.
- [ ] `abort` is compared with cooperative cancellation, including `JoinError::is_cancelled`.
- [ ] Drop counters show which synchronous cleanup runs.
- [ ] Cancellation before startup, during a blocked receive, and after normal completion are all tested, with timeouts used only to bound failure.
- [ ] A written explanation of why aborting `spawn_blocking` does not stop its closure.

References: [Tokio tutorial](https://tokio.rs/tokio/tutorial), [select! and cancellation safety](https://docs.rs/tokio/latest/tokio/macro.select.html), [spawn_blocking](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html), and [CancellationToken](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html).
