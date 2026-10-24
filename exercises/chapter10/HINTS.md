# Chapter 10 hints

[Chapter guide](README.md) · [All hints](../../HINTS.md)

Try the first hint before opening the second. The implementation constraints in each task are part of the exercise.

## 01 · Variants and payloads

<details><summary>Hint 1</summary>

Model each state by a variant and put only the data that state needs inside it.

</details>
<details><summary>Hint 2</summary>

Match all variants explicitly; move an owned String out of the text variant when returning it.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

An enum value is a discriminant (which variant) plus that variant's payload, if any; different variants can carry different types. A `match` must cover every variant, so adding a variant makes the compiler list every match that needs updating. Only fieldless enums can be cast with `as`, and their discriminants count up from the previous explicit value. The default layout is unspecified, so never treat it as a serialization format; the null-pointer optimization is only guaranteed for types such as `Option<Box<T>>` and `Option<&T>`, which are one pointer wide.

</details>

## 02 · Recursive data with enums

<details><summary>Hint 1</summary>

The enum describes one node; Box owns another separately allocated node.

</details>
<details><summary>Hint 2</summary>

Match literals directly, recurse on children for sums and negation, and count one level per current node.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A type must have a size known at compile time, and an enum that contained itself directly would be infinitely large (E0072). `Box` breaks the cycle: the variant holds one pointer, and the child lives in its own heap allocation that the parent owns. Each node owns its children, so dropping the root drops the whole tree, fields in declaration order. Functions over the tree recurse once per variant, mirroring the type's structure. Recursion depth equals tree depth, so a deeply skewed tree can overflow the stack.

</details>

## 03 · Generic enums

<details><summary>Hint 1</summary>

Type parameters describe payload types independently; moving payloads does not require cloning.

</details>
<details><summary>Hint 2</summary>

Swap the variant names and their type order; borrowing maps owned payloads to shared references.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A generic enum such as `Either<L, R>` is a family of types; each variant uses its own type parameter. Swapping variants moves each payload into the other variant and changes the type to `Either<R, L>`, and that needs no bounds, because moving never requires `Clone` or `Copy`. `Either<&L, &R>` borrows the payload in place; this is exactly what `Option::as_ref` does. The std enums `Option` and `Result` derive `Ord` by variant order, which is why `None < Some(_)` and `Ok(_) < Err(_)`. Prefer `Result` when one side means failure; `Either` says nothing about which side is the good one.

</details>

## 04 · Literal, tuple and struct patterns

<details><summary>Hint 1</summary>

A new variable pattern binds any value; a literal pattern checks a value.

</details>
<details><summary>Hint 2</summary>

Use Point{x:0,y:0} before the axis patterns; destructure a tuple parameter for the second task.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

A lowercase identifier in a pattern always introduces a new binding, so it matches anything and shadows any outer variable of the same name; compare with a guard or use a `const`. `_` matches without binding, so it never moves the value, while `_name` is a real binding that moves and is merely exempt from the unused warning. Struct patterns name fields, so they survive field reordering; `field: name` renames, and `..` ignores the rest. Specific arms must come before general ones, because the first match wins.

</details>

## 05 · Matching references and mutating payloads

<details><summary>Hint 1</summary>

Match ergonomics propagates borrowing to payload bindings.

</details>
<details><summary>Hint 2</summary>

Return the String payload as &str through a shared match. A mutable match gives &mut u16; checked_add before assigning.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Matching a reference with a non-reference pattern switches the default binding mode: matching `&State` binds payloads as `&T`, and matching `&mut State` binds them as `&mut T`. That is why `name` borrows the String and `n` can be updated through `*n += 1`. Explicit `&` patterns and `ref`/`ref mut` are the older spelling; Rust 2024 rejects `ref` where the default mode is already a reference. You cannot move a String out through a shared reference (E0507), but Copy fields are simply copied. Write a change only after the check succeeds, so a failure leaves the value untouched.

</details>

## 06 · Array and slice patterns

<details><summary>Hint 1</summary>

Order empty and one-element cases before a pattern requiring at least two.

</details>
<details><summary>Hint 2</summary>

Use [first,middle @ ..,last] for the borrowed middle; destructure the fixed array directly for the checksum.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

An array's length is part of its type, so `[a, b, c]` is an irrefutable pattern for `[u8; 3]` and the compiler rejects patterns of the wrong length. A slice's length is only known at run time, so slice patterns are refutable and `match` needs a case for every length. `[first, rest @ .., last]` requires at least two elements; `rest` borrows the middle as a subslice without copying. Only one `..` is allowed per slice pattern. On an array matched by value, `rest @ ..` binds an array, not a slice.

</details>

## 07 · Guards, alternatives and @ bindings

<details><summary>Hint 1</summary>

A guard adds a runtime condition after structural matching; @ binds a value while checking its shape.

</details>
<details><summary>Hint 2</summary>

Use n @ 10..=20 for the code. For the tuple alternatives bind the numeric payload consistently before testing the guard.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Rust tries arms top to bottom: first the pattern, then its guard; if the guard is false, it moves on to the next arm. A guard applies to the whole or-pattern before it, and it may run once for each alternative that matches. Guards are not checked for exhaustiveness, so a match with guarded arms still needs an unguarded fallback. Every alternative of an or-pattern must bind the same names with the same types. `name @ pattern` tests the subpattern and keeps the whole matched value, so you do not have to repeat the range in the body.

</details>

## 08 · Pattern contexts and public compatibility

<details><summary>Hint 1</summary>

Use while let for a repeated optional value, and a refutable guard for one-shot extraction.

</details>
<details><summary>Hint 2</summary>

Repeatedly pop while Some to transfer ownership. Use let Some((key,value)) = ... else to expose a successful tuple in the remaining scope.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

Irrefutable patterns, which always match, appear in `let`, function parameters, closure parameters, and `for`. Refutable patterns need a way to handle failure: `match` arms, `if let`, `while let`, and `let ... else`, whose `else` must diverge (return, break, continue, panic). Let chains (`if let A = x && cond && let B = y`) combine several checks without nesting. Outside its crate, a `#[non_exhaustive]` enum requires a wildcard arm, so the author can add variants without breaking you. Inside the defining crate, the match stays exhaustive, and you should not add `_` to closed enums just in case, because it hides missing cases.

</details>

## 09 · Capstone: generic ordered tree

<details><summary>Hint 1</summary>

Represent Empty separately from Node. Recurse left for smaller values and right for larger ones; ignore duplicates.

</details>
<details><summary>Hint 2</summary>

Match self by mutable reference for insertion. Create a boxed Node only in Empty; inorder recurses left, visits the borrowed value, then recurses right.

</details>
<details><summary>Check your explanation (after writing your own)</summary>

`insert` matches `&mut *self` (or `self`) recursively: at `Empty` it replaces the node with a new `Node`, otherwise it compares and recurses into one child, so the tree stays ordered and unique. Every child is a `Box<Tree<T>>`, because a node cannot contain itself directly. `inorder` visits left, value, right, and since every left value is smaller and every right value is larger, the output is sorted; it pushes `&'a T` borrows tied to the tree, so no `Clone` bound is needed. Sorted insertion degenerates into a list of depth n; balanced trees such as `std::collections::BTreeMap` avoid that, which is why production code uses them.

</details>

## R01 · Spaced review of Chapters 3, 7 and 9

<details><summary>Hint 1</summary>

`DoorState` is Copy, so `self.state.get()` hands you a value to match and `set` stores the new one; no `&mut self` is needed. For commands, collect `line.split_whitespace()` into a `Vec<&str>` and match on `words.as_slice()`. For the tally, `text.chars()` yields Unicode scalars; match each against `'a'..='z' | 'A'..='Z'` and `'0'..='9'`.

</details>
<details><summary>Hint 2</summary>

`From<ParseIntError>` is what lets `dx.parse::<i32>()?` turn into `ParseError::BadNumber` automatically. Slice patterns: `["move", dx, dy]`, `["say", rest @ ..] if !rest.is_empty()` with `rest.join(" ")`, `["quit"]`, `[]`, and `[first, ..]`. In `tally`, the whitespace case needs a guard: `c if c.is_whitespace()`.

</details>

## C01 · Code review

<details><summary>Hint 1</summary>

There are two bugs: one is a wildcard arm that swallows a variant, and one is a pattern that was meant to compare but binds instead. Every `.clone()` in this file is unnecessary.

</details>
<details><summary>Hint 2</summary>

Return `Option<&str>` and match the borrowed event, so payloads bind as references. Use an or-pattern for the three user-bearing variants and list `Event::Error(_)` explicitly instead of `_`. In `audit_count`, a guard (`if user == admin`) does the comparison. `&[Event]` and `filter`/`map`/`sum` replace the index loop.

</details>
<details><summary>Check your review</summary>

`user` clones the whole event to match it and returns an owned String, and its `_ => None` arm silently swallows `Logout`, so logouts have no user; listing every variant would have made the compiler catch it. `uploaded_by` takes `&Vec<Event>`, loops by index, clones every event, and builds a `String` just to compare; an `if let` over `events.iter()` with borrowed fields needs no allocation at all. `audit_count` binds a new `admin` in each alternative instead of comparing with the parameter, so every event counts; the crate's `allow(unused_variables)` hid the warning that would have flagged the unused parameter. Clippy also flags the single-arm `match`, the collapsible `if`, and `to_string` in the comparison.

</details>

## Drills

Run `cargo drill ch10`. The compiler output explains *what* happened; these notes explain *why*. Read one only after the drill's verdict.

<details><summary>d01 · A new variant</summary>E0004: the match does not cover `Status::Archived`. Exhaustiveness checking is the payoff of not writing `_` arms: adding a variant lists every place that needs a decision. Add the `Archived` arm.</details>
<details><summary>d02 · A refutable let</summary>E0005: `let` needs an irrefutable pattern, and `Some(word)` fails for an empty string. Use `let Some(word) = ... else { return ""; };`, or `unwrap_or("")`.</details>
<details><summary>d03 · A lowercase name in a pattern</summary>Panics: the program compiles (with an unreachable-pattern warning), but `expected` in the pattern is a NEW binding that matches every code, so `check(404)` returns "ok". Compare with a guard (`c if c == expected`) or make it a constant (`const EXPECTED: u16 = 200;`), because paths and constants in patterns are compared, not bound.</details>
<details><summary>d04 · Or-patterns must bind the same names</summary>E0408: `code` exists only when the event is a Key, so the arm's body could not rely on it. Every alternative must bind the same names with the same types; here the value is unused, so write `Event::Key(_) | Event::Click`.</details>
<details><summary>d05 · Moving a payload out of a reference</summary>E0507: matching `*slot` by value would move the String out of something the caller still owns. Match `slot` (the reference) so `name` binds as `&String`, then `name.clone()`; or change the signature to return `&str`, or take `Option<String>` by value.</details>
<details><summary>d06 · A recursive enum</summary>E0072: a `List` containing a `List` directly would be infinitely large. Store the tail as `Box<List>`, a fixed-size pointer to a separate heap node, and wrap each tail in `Box::new`.</details>
<details><summary>d07 · Mutating through a match on &mut</summary>Compiles and runs. Matching `&mut Counter` with a non-reference pattern binds `n` as `&mut u32` (default binding modes), so `*n += 1` changes the counter in place; no `ref mut` is needed.</details>
<details><summary>d08 · A match moves its payload</summary>E0382: matching `message` by value moves the String into `text`, so `message` is partially moved afterward. Match `&message` (or `message.as_ref()`) so `text` is a `&String` borrow.</details>
<details><summary>d09 · Array patterns have a length</summary>E0527: an array's length is part of its type, so the pattern needs three elements. Write `[red, green, _]` or `[red, green, ..]`.</details>
<details><summary>d10 · Guards and exhaustiveness</summary>E0004: the compiler does not reason about guard conditions, so it cannot prove that `x < 0`, `0`, and `x > 0` cover every i32. Make the last arm unguarded (`_ => "positive"`).</details>
