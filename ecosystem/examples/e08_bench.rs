//! E08 (TOOLING.md task 6): compare two implementations with your `measure` and `summarize`.
//! Run: cargo run --release --manifest-path ecosystem/Cargo.toml --example e08_bench
//! Then run it again without `--release` and compare the numbers.

use ecosystem::e08_tooling::{measure, summarize};
use std::fmt::Write as _;
use std::hint::black_box;

/// Rebuilds the whole string on every step: quadratic copying.
fn join_with_format(words: &[String]) -> String {
    let mut out = String::new();
    for word in words {
        out = format!("{out}{word},");
    }
    out
}

/// Appends in place to one growing buffer.
fn join_with_write(words: &[String]) -> String {
    let mut out = String::new();
    for word in words {
        let _ = write!(out, "{word},");
    }
    out
}

type Join = fn(&[String]) -> String;

fn main() {
    let words: Vec<String> = (0..2_000).map(|i| format!("word{i}")).collect();
    assert_eq!(join_with_format(&words), join_with_write(&words));
    let candidates: [(&str, Join); 2] =
        [("format!", join_with_format), ("write!", join_with_write)];
    for (name, join) in candidates {
        let samples = measure(3, 21, || join(black_box(&words)));
        let stats = summarize(&samples).expect("21 samples");
        println!(
            "{name:>8}: min {:>10?}  median {:>10?}  max {:>10?}",
            stats.min, stats.median, stats.max
        );
    }
    if cfg!(debug_assertions) {
        println!("debug build: these numbers say little about production; use --release");
    }
}
