# External macro consumer

After implementing l06, run from the exercise repository root:

```sh
cargo test --manifest-path exercises/chapter22/export_lab/Cargo.toml
```

This isolated workspace has no registry dependencies. The consumer renames its library dependency and provides a same-named helper. `$crate` must resolve the public definition path regardless of either choice. The library explicitly declares `pub mod l06_exports`: a pub function inside a private module would remain inaccessible from another crate. Remove `pub` temporarily, record the privacy error, then restore it. The unfinished helper/macro TODOs intentionally fail until l06 is implemented.
