# Chapter 14 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the chapter reference and compiler diagnostics first. Open one level at a time.

## 01 · Borrowed captures and sorting

<details><summary>Hint 1</summary>

A closure may refer to surrounding variables without an explicit parameter for each one.

</details>
<details><summary>Hint 2</summary>

Filter using starts_with and map to owned strings. Sort by (byte length, lexical content) for a deterministic tie-break.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A closure that only reads `prefix` needs nothing more than a shared reference, so the compiler captures `&prefix` (the weakest capture that works), and the closure is `Fn`. Shared borrows can coexist, so the caller can still read `prefix`, and `sort_by` can borrow the vector's elements at the same time. The borrow lasts as long as the closure is *used*, not until the end of the block: after the last call, non-lexical lifetimes release it. Since edition 2021 a closure captures the precise place it uses, such as `point.0`, so it can borrow one field while another field is mutated.

</details>

## 02 · Mutation, move, and consuming captures

<details><summary>Hint 1</summary>

The body’s use of captured data determines the call trait. The move keyword chooses how data enters the closure.

</details>
<details><summary>Hint 2</summary>

Use a mutable local accumulator for the repeated closure; a move closure can retain owned text and still be callable repeatedly if it only reads it.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

The call traits describe what a call does to the closure's captured state. `Fn` calls only read it (`&self`), `FnMut` calls may change it (`&mut self`), and `FnOnce` calls may consume it (`self`). The body decides which traits a closure implements; `move` only decides how values enter the closure. So `move || text.len()` is still `Fn`, and `move || text` is only `FnOnce`. The traits nest: anything callable through `&self` can also be called through `&mut self` or by value, so every `Fn` is also `FnMut` and `FnOnce`, and a function that asks for `FnOnce` accepts all three.

</details>

## 03 · Choosing callable bounds

<details><summary>Hint 1</summary>

Use the least restrictive call trait needed by how often and how the function invokes its argument.

</details>
<details><summary>Hint 2</summary>

FnOnce can be consumed by one call; FnMut needs a mutable binding; Fn supports repeated shared calls, including function pointers.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A bound is a promise about how *your function* will call its argument, so pick the weakest one your body needs. `apply_once` calls `f` once, so `FnOnce` lets callers pass closures that give away their captures; requiring `Fn` would reject them for no reason. `invoke_twice` calls `f` twice, so it needs at least `FnMut`, and a closure that moves its String out cannot be called a second time. Function items and non-capturing closures implement all three traits. Every closure has its own anonymous type, which is why generics (or trait objects) are needed to accept them.

</details>

## 04 · Returning borrowed and erased closures

<details><summary>Hint 1</summary>

An opaque return hides one concrete closure type. Boxing erases distinct closure types behind one trait object.

</details>
<details><summary>Hint 2</summary>

Borrow prefix in a move closure with an explicit lifetime. Box each branch separately in choose.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`impl Fn` returns one concrete closure type, which the caller cannot name; the value lives inline (no allocation), and calls are static and can be inlined. `Box<dyn Fn>` erases the type: it costs a heap allocation (unless the closure is zero-sized) plus a vtable call, but different branches can return different closures. For lifetimes, `impl Fn(&str) -> bool + 'a` says the closure may hold borrows that live for `'a`, so the caller must keep `prefix` alive while using it. A plain `Box<dyn Fn>` return means `Box<dyn Fn + 'static>`, so it may only capture owned data.

</details>

## 05 · Copy and Clone for captures

<details><summary>Hint 1</summary>

A closure is an anonymous struct-like value with fields for its captures.

</details>
<details><summary>Hint 2</summary>

Copy a closure with a moved scalar capture. Clone a closure with a moved Vec capture before either call; each should mutate its own vector.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A closure is an anonymous struct whose fields are its captures, and the compiler derives `Clone` and `Copy` for it the same way it would for such a struct: when every field allows it. A closure that captures only `Copy` values by move, or only shared references, is `Copy`. A closure that owns a `Vec` or `String` is `Clone` but not `Copy`, and cloning it clones those values, so the copies have independent state. A closure that captures `&mut` is neither, because two copies would be two aliasing mutable references. Copying a shared reference copies a pointer; the referent is not cloned.

</details>

## 06 · Callback registration and invocation

<details><summary>Hint 1</summary>

A callback registry owns callable values and calls them in a documented order.

</details>
<details><summary>Hint 2</summary>

Push the boxed callback. Iterate mutably through callbacks; append each result in registration order.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`Vec<Box<dyn FnMut(&str) -> String>>` has one element type, the trait object, but each box can hold a closure of a different concrete type with different captures. The trait object stores a data pointer and a vtable pointer. With no lifetime written, a boxed trait object in a struct field means `+ 'static`, so registered callbacks must own their captures (`move`), or the registry needs a lifetime parameter. Calling an `FnMut` callback may change its state, so `dispatch` needs `&mut self` to get `&mut` access to each box. The callbacks and everything they captured live until the registry is dropped.

</details>

## 07 · Capstone: configurable text processing

<details><summary>Hint 1</summary>

Keep predicate, transformation, and audit responsibilities separate. Stateful callbacks must see a documented invocation order.

</details>
<details><summary>Hint 2</summary>

For each row call keep exactly once. Accepted rows are formatted, audited, then pushed. Audit only sees formatted accepted rows.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

The pipeline fixes the *order* of operations, and the caller supplies the *policy* as closures. `FnMut` is the weakest bound that allows several calls, so callers can pass stateful predicates (accept every other row) and auditors that push into a captured Vec. `finish_report` calls its callback once, so `FnOnce` lets the callback consume what it captured. `format` and `audit` must not run for rejected rows: a formatter or auditor with side effects would record rows that were never accepted. Each row is fully handled before the next `keep` call, so stateful callbacks see one consistent sequence.

</details>

## R01 · Spaced review of Chapters 10–13

<details><summary>Hint 1</summary>

`Cow::Borrowed` keeps a `&'a str` from the input; `Cow::Owned` holds a new String. A blanket `impl<F: Fn(i32) -> bool> Rule for F` makes every matching closure a `Rule`. A `match` arm can have an `if` guard, and `break` leaves the loop.

</details>
<details><summary>Hint 2</summary>

`items.iter().map(|&s| match fix(s) { ... }).collect()` calls `fix` once per item. In the blanket impl, `self` is the closure: call it as `self(value)`. `rules.iter().all(|r| r.check(value))` stops at the first false. Put the guarded arm `Event::Click { x, y } if x < 0 || y < 0 => {}` before the general Click arm.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one sorts in the wrong direction, one accepts too much. Two of the three functions allocate on every call, or every comparison, for no reason.

</details>
<details><summary>Hint 2</summary>

`sort_by_key` cannot return a key that borrows from the element, so the teammate cloned the name; `sort_by` with `b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))` borrows both. The filter closure already owns `required`, so it can iterate it in place; `all` rather than `any`. A fold carries only the current value through the steps.

</details>
<details><summary>Check your review</summary>

`sort_users` sorts youngest first, and its key clones every name on every comparison (thousands of allocations for 200 users). It can take `&mut [(String, u32)]`. `contains_all` uses `any` where the contract says every substring, so a word containing only one requirement is accepted, and an empty requirement list rejects everything. It also clones the captured Vec on every call, though the `move` closure already owns it. It can return `impl Fn(&str) -> bool`: one concrete type, no Box, no vtable. `pipeline` keeps a history Vec it never needs, indexes in a range loop, and `unwrap`s a value that always exists; `steps.iter().fold(input, |v, step| step(v))` says the same thing with no allocation. Clippy also flags the `&Vec` and `&mut Vec` parameters and the range loop.

</details>

## Drills

Run `cargo drill ch14`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · Calling a consuming closure twice</summary>E0382. The body returns `name`, moving it out of the closure, so the closure is only `FnOnce`, and the first call consumes the closure itself. Return `name.clone()` if every call needs its own String, or call it once.</details>
<details><summary>d02 · Reading a variable a closure is mutating</summary>E0502. `bump` holds a mutable borrow of `count` from its creation until its last use, and the second `bump()` comes after the `println!`. Move the `println!` after the last call; the borrow ends there.</details>
<details><summary>d03 · The borrow ends at the last call</summary>Compiles. The closure's mutable borrow lasts only until its last use (non-lexical lifetimes), so `count` is free to read afterward.</details>
<details><summary>d04 · Mutating inside an Fn closure</summary>E0594. `F: Fn()` promises calls through `&self`, so the closure cannot assign to a captured variable. Ask for the weakest bound the function needs: `F: FnMut()` with `mut f`.</details>
<details><summary>d05 · A thread borrowing a local</summary>E0373. `thread::spawn` needs a `'static` closure, because the thread may outlive `main`'s stack frame. Add `move` so the thread owns `words` (or use `thread::scope` to borrow).</details>
<details><summary>d06 · Two closures, one return type</summary>E0308. Every closure has its own anonymous type, even with identical signatures, and `impl Fn` stands for exactly one. Box each branch and return `Box<dyn Fn(i32) -> i32>`. (Closures that capture nothing would coerce to a `fn` pointer and compile.)</details>
<details><summary>d07 · Passing a consuming closure where Fn is required</summary>E0525. `let owned = log;` moves the Vec out, so the closure is only `FnOnce`, but `run_twice` asks for `Fn`. Borrow instead: `let owned = &log;`. When the closure is written directly in the call, rustc reports E0507 for the same mistake.</details>
<details><summary>d08 · A move closure called twice</summary>Compiles. `move` decides how `greeting` enters the closure (by value); the body only reads it, so the closure is still `Fn` and callable many times.</details>
<details><summary>d09 · A callback that re-enters its owner</summary>Panics with "already borrowed": the loop holds `log.borrow()` while `record` calls `borrow_mut()` on the same RefCell. RefCell checks at run time what the borrow checker checks at compile time. Copy what you need first (`let snapshot = log.borrow().clone();`) and loop over the snapshot.</details>
<details><summary>d10 · A nested fn is not a closure</summary>E0434. A `fn` item inside `main` is an ordinary function: it cannot see `main`'s local variables. Use a closure, `let scale = |x: i32| x * rate;`, or pass `rate` as a parameter.</details>
