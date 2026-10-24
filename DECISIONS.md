# My Rust decision rules

Fluency means making these calls quickly and correctly while writing real code. Each chapter's NOTES.md records what you learned. This page condenses it into rules you apply.

Fill in a rule when you finish the chapter shown, in your own words, with one example from your code. Rewrite rules during spaced reviews as your understanding sharpens. Hints for checking yourself are in the chapter guides' mastery checks.

| After | Decision | My rule and an example |
| --- | --- | --- |
| Ch3 | Which integer type, and which overflow policy (wrapping, checked, saturating, strict)? | |
| Ch3 | `String`, `&str`, `Vec<u8>`, `PathBuf`, or `OsString` at a boundary? | |
| Ch4 | Move, clone, or share with `Rc`/`Arc`? | |
| Ch5 | Should a function take `T`, `&T`, or `&mut T`? Return owned or borrowed? | |
| Ch5 | When do I need an explicit lifetime, and what does it promise? | |
| Ch7 | `panic!`, `Result`, or `Option`? `?` or `match`? One error enum or `Box<dyn Error>`? | |
| Ch8 | What is `pub`, `pub(crate)`, or private? How do I lay out modules? | |
| Ch9 | Struct field visibility, constructors, and when to use `Cell`/`RefCell`? | |
| Ch10 | Enum or trait object? How do I make invalid states unrepresentable? | |
| Ch11 | Generic `T: Trait`, `impl Trait`, or `dyn Trait`? | |
| Ch13 | Which conversion trait: `From`, `TryFrom`, `AsRef`, `Borrow`, or `Into`? When is `Cow` worth it? | |
| Ch14 | Which closure bound: `Fn`, `FnMut`, or `FnOnce`? When do I need `move`? | |
| Ch15 | Iterator chain or `for` loop? When should I `collect`? | |
| Ch16 | `Vec`, `VecDeque`, `HashMap`, `BTreeMap`, `HashSet`, or `BinaryHeap`? | |
| Ch17 | Bytes, chars, or graphemes? When do I need `format!` versus `write!`? | |
| Ch18 | Buffered or unbuffered? Where do I handle `io::Error`? | |
| Ch19 | Scoped threads, channels, `Mutex`, `RwLock`, or atomics? | |
| Ch20–21 | Threads or async? What must be `Send`, and what is cancellation-safe? | |
| Ch22 | Macro or function or generic? | |
| Ch23–24 | When is `unsafe` justified, and what goes in the `// SAFETY:` comment? | |
