# Chapter 16 lab — Random elements

This isolated Cargo package completes the chapter's **Random Elements** section. It keeps the core exercises free of external dependencies. It is part of chapter mastery; run it when you can fetch `rand` (or have the crate cached).

Use the book alongside the official [IndexedRandom](https://docs.rs/rand/0.10.3/rand/seq/trait.IndexedRandom.html), [SliceRandom](https://docs.rs/rand/0.10.3/rand/seq/trait.SliceRandom.html), and [StdRng](https://docs.rs/rand/0.10.3/rand/rngs/struct.StdRng.html) APIs. The package pins rand 0.10.3. `sample` is the current name for distinct sampling.

Before coding, predict: which operations borrow and which mutate? What does choosing from an empty slice return? Does a shuffled list have to change order? Can two different seeds happen to choose the same element?

Implement all three functions in `src/lib.rs` and run from the repository root:

```sh
cargo test --manifest-path exercises/chapter16/random-lab/Cargo.toml
cargo test --release --manifest-path exercises/chapter16/random-lab/Cargo.toml
```

Append `--offline` if dependencies are already cached. The starter compiles and fails its tests intentionally. No test assumes a specific random permutation: tests check membership, multiplicity, requested size, empty handling, and repeatability for the same seed within this pinned environment. StdRng's sequence is not a cross-version or cross-platform data format.

1. **Choose:** seed a StdRng and choose a copied element through IndexedRandom.
2. **Shuffle:** mutate a vector through SliceRandom and preserve its multiset, including duplicates.
3. **Sample:** use IndexedRandom::sample to select at most `amount` distinct positions. Input values in this task are unique, so values must be unique too.
4. **Manual comparison:** replace StdRng with `rand::rng()` in scratch. Run several times; explain why a fixed expected output is unsuitable. Do not turn statistical expectations into a flaky test.
5. **Explain:** distinguish sampling with replacement, without replacement, and shuffling. Why is `seed % length` not a general unbiased sampling algorithm?

<details><summary>Hint 1</summary>

Import `SeedableRng` and the extension traits from `rand::seq`. Random operations take `&mut rng` because they advance its state.

</details>
<details><summary>Hint 2</summary>

Create `StdRng::seed_from_u64(seed)`, use `choose(...).copied()`, `shuffle`, and `sample(...).copied().collect()`. No manual range arithmetic is needed.

</details>
