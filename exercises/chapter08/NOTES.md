# Chapter 8 notes — Crates and Modules

Your workbook. Write answers in your own words, *before* checking the hints. When `cargo progress` says this chapter is due for review, answer the mastery questions at the bottom again from memory, then compare with what you wrote here.

## l01 · Crates, editions and profiles

**Predictions I got wrong, and why:**

- 

**Explain:** Distinguish package, crate, target, compiler version, edition, rust-version, and build profile.

> 

**Edge case I added:**

- 

## l02 · Nested modules and private implementation

**Predictions I got wrong, and why:**

- 

**Explain:** Explain why a module boundary should expose operations that preserve invariants instead of raw representation.

> 

**Edge case I added:**

- 

## l03 · Paths, imports and reexports

**Predictions I got wrong, and why:**

- 

**Explain:** Explain absolute paths, self/super/crate, imported aliases, prelude names, and the role of a facade reexport.

> 

**Edge case I added:**

- 

## l04 · Constants and statics

**Predictions I got wrong, and why:**

- 

**Explain:** Explain shared static storage and why const is a value definition, without assuming identity for constant references.

> 

**Edge case I added:**

- 

## l05 · Attributes, cfg and included files

**Predictions I got wrong, and why:**

- 

**Explain:** Explain conditional compilation versus runtime branching, environment capture at compilation, and file inclusion paths.

> 

**Edge case I added:**

- 

## l06 · Tests and executable documentation

**Predictions I got wrong, and why:**

- 

**Explain:** Explain unit versus integration tests and doc-tests, including what a public consumer can prove.

> 

**Edge case I added:**

- 

## l07 · Dependency requirements and lockfiles

**Predictions I got wrong, and why:**

- 

**Explain:** Explain compatible version requirements, the pre-1.0 exception, lockfile reproducibility, and why path dependencies are useful for offline experiments.

> 

**Edge case I added:**

- 

## l08 · Capstone: library facade and workspace

**Predictions I got wrong, and why:**

- 

**Explain:** Explain the public facade and the external test that proves its privacy. Audit exactly what a package archive contains.

> 

**Edge case I added:**

- 

## Drills

Drills whose verdict surprised me, and the rule I learned:

- 

## Mastery check (answer from memory)

**Explain crate versus module versus package, and make a multi-target package with a library.**

> 

**Control visibility with private fields and public reexports; prove it from an external consumer crate.**

> 

**Split modules into files without changing their public paths.**

> 

**Distinguish const from static and compile-time cfg from a runtime if.**

> 

**Write unit, integration, and doc tests; explain lockfiles, dependency requirements, build scripts, and workspaces.**

> 

**Inspect a local package archive without publishing anything.**

> 

**Redo the final capstone after a break without looking at your implementation; invent a changed requirement and explain the tradeoff.**

>
