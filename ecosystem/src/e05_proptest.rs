//! LESSON E05 — Property-based testing with proptest (after Chapters 15 and 16)
//! Run: cargo test --manifest-path ecosystem/Cargo.toml e05_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: If a property fails on a random
//! 700-byte input, what does proptest report? Which inputs will a naive
//! `vec(any::<u8>(), 0..100)` generator almost never produce?
//! IMPLEMENT the tasks below. The code under test (`Rle`, `RingBuffer`) is supplied
//! and correct; you write the generators and the properties. The `m_` tests then
//! check your properties against deliberately broken versions ("mutants") in the
//! `mutants` module. Write properties from the contracts first; read the mutants
//! only after a mutant survives. Preserve the tests and add an edge case.
//! EXPLAIN afterward: How does shrinking find a minimal counterexample? Why does a
//! round-trip property alone miss the `SplitsEarly` mutant? Why does a model-based
//! test need a model that is obviously correct?
//! JUDGMENT: proptest has value trees with integrated shrinking and is the usual choice.
//! quickcheck is smaller and simpler but shrinks less well. Fuzzers (cargo-fuzz)
//! explore deeper with coverage feedback, but they run outside `cargo test`.
//! Use example tests to document behavior and properties to search for bugs.

use proptest::prelude::*;

/// A run-length codec: `(count, byte)` pairs, `count` in `1..=255`.
pub trait Codec {
    fn encode(&self, data: &[u8]) -> Vec<(u8, u8)>;
    fn decode(&self, runs: &[(u8, u8)]) -> Vec<u8>;
}

/// The codec under test. Its contract: `decode(encode(x)) == x`, and `encode` output is
/// *canonical*: every count is at least 1, the counts sum to the input length, and two
/// adjacent runs share a byte only when the first one is full (count 255).
pub struct Rle;

impl Codec for Rle {
    fn encode(&self, data: &[u8]) -> Vec<(u8, u8)> {
        let mut runs: Vec<(u8, u8)> = Vec::new();
        for &byte in data {
            match runs.last_mut() {
                Some((count, last)) if *last == byte && *count < u8::MAX => *count += 1,
                _ => runs.push((1, byte)),
            }
        }
        runs
    }

    fn decode(&self, runs: &[(u8, u8)]) -> Vec<u8> {
        runs.iter()
            .flat_map(|&(count, byte)| std::iter::repeat_n(byte, usize::from(count)))
            .collect()
    }
}

/// A queue with fixed capacity: `push` on a full ring overwrites the oldest item,
/// `pop` removes the oldest item.
pub trait Ring {
    fn push(&mut self, value: u8);
    fn pop(&mut self) -> Option<u8>;
    fn len(&self) -> usize;
}

/// The ring buffer under test.
pub struct RingBuffer {
    slots: Vec<u8>,
    start: usize,
    len: usize,
}

impl RingBuffer {
    /// Panics if `capacity` is 0.
    pub fn with_capacity(capacity: usize) -> RingBuffer {
        assert!(capacity > 0, "capacity must be positive");
        RingBuffer {
            slots: vec![0; capacity],
            start: 0,
            len: 0,
        }
    }
}

impl Ring for RingBuffer {
    fn push(&mut self, value: u8) {
        let capacity = self.slots.len();
        if self.len == capacity {
            self.slots[self.start] = value;
            self.start = (self.start + 1) % capacity;
        } else {
            self.slots[(self.start + self.len) % capacity] = value;
            self.len += 1;
        }
    }

    fn pop(&mut self) -> Option<u8> {
        if self.len == 0 {
            return None;
        }
        let value = self.slots[self.start];
        self.start = (self.start + 1) % self.slots.len();
        self.len -= 1;
        Some(value)
    }

    fn len(&self) -> usize {
        self.len
    }
}

/// One step of a model-based test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Push(u8),
    Pop,
}

/// A. A generator of codec inputs. It must produce empty inputs, short arbitrary
/// bytes, *and* long runs of a repeated byte (runs longer than 255 in particular).
/// Tip: `prop_oneof!`, `Just`, `proptest::collection::vec`, `prop_map`, and `.boxed()`.
pub fn arb_data() -> BoxedStrategy<Vec<u8>> {
    todo!("Implement this task")
}

/// B. Property: decoding the encoding returns the input. Use `prop_assert_eq!`, which
/// returns a `TestCaseError` instead of panicking.
pub fn check_roundtrip(codec: &dyn Codec, data: &[u8]) -> Result<(), TestCaseError> {
    todo!("Implement this task")
}

/// C. Property: the encoding of `data` is canonical (see `Rle`'s contract).
pub fn check_canonical(codec: &dyn Codec, data: &[u8]) -> Result<(), TestCaseError> {
    todo!("Implement this task")
}

/// D. A generator of `(capacity, ops)` for the ring: capacity in `1..=8`, up to 64
/// ops, with pushes more frequent than pops so the ring fills up and wraps around.
pub fn arb_ops() -> BoxedStrategy<(usize, Vec<Op>)> {
    todo!("Implement this task")
}

/// D. Model-based property: apply `ops` to `make(capacity)` and to a `VecDeque` model
/// that you update by the same contract. After every op, `len` must agree, and every
/// `pop` must return what the model returns.
pub fn check_against_model(
    make: &dyn Fn(usize) -> Box<dyn Ring>,
    capacity: usize,
    ops: &[Op],
) -> Result<(), TestCaseError> {
    todo!("Implement this task")
}

/// Deliberately broken implementations. Each one should make one of your properties fail.
#[cfg(test)]
mod mutants {
    use super::*;

    /// Never splits a run: counts above 255 wrap around.
    pub struct NoSplit;
    impl Codec for NoSplit {
        fn encode(&self, data: &[u8]) -> Vec<(u8, u8)> {
            let mut runs: Vec<(usize, u8)> = Vec::new();
            for &byte in data {
                match runs.last_mut() {
                    Some((count, last)) if *last == byte => *count += 1,
                    _ => runs.push((1, byte)),
                }
            }
            runs.into_iter().map(|(n, b)| (n as u8, b)).collect()
        }
        fn decode(&self, runs: &[(u8, u8)]) -> Vec<u8> {
            Rle.decode(runs)
        }
    }

    /// Forgets the last run when decoding.
    pub struct DropsLast;
    impl Codec for DropsLast {
        fn encode(&self, data: &[u8]) -> Vec<(u8, u8)> {
            Rle.encode(data)
        }
        fn decode(&self, runs: &[(u8, u8)]) -> Vec<u8> {
            Rle.decode(&runs[..runs.len().saturating_sub(1)])
        }
    }

    /// Splits runs at 128: still round-trips, but not canonical.
    pub struct SplitsEarly;
    impl Codec for SplitsEarly {
        fn encode(&self, data: &[u8]) -> Vec<(u8, u8)> {
            let mut runs: Vec<(u8, u8)> = Vec::new();
            for &byte in data {
                match runs.last_mut() {
                    Some((count, last)) if *last == byte && *count < 128 => *count += 1,
                    _ => runs.push((1, byte)),
                }
            }
            runs
        }
        fn decode(&self, runs: &[(u8, u8)]) -> Vec<u8> {
            Rle.decode(runs)
        }
    }

    /// Encodes empty input as one zero-length run: still round-trips.
    pub struct EmptyMarker;
    impl Codec for EmptyMarker {
        fn encode(&self, data: &[u8]) -> Vec<(u8, u8)> {
            if data.is_empty() {
                vec![(0, 0)]
            } else {
                Rle.encode(data)
            }
        }
        fn decode(&self, runs: &[(u8, u8)]) -> Vec<u8> {
            Rle.decode(runs)
        }
    }

    /// One run per byte, never merging: still round-trips.
    pub struct NoMerge;
    impl Codec for NoMerge {
        fn encode(&self, data: &[u8]) -> Vec<(u8, u8)> {
            data.iter().map(|&b| (1, b)).collect()
        }
        fn decode(&self, runs: &[(u8, u8)]) -> Vec<u8> {
            Rle.decode(runs)
        }
    }

    /// Ignores pushes when full, instead of overwriting the oldest item.
    pub struct DropsNewest(pub RingBuffer);
    impl Ring for DropsNewest {
        fn push(&mut self, value: u8) {
            if self.0.len() < self.0.slots.len() {
                self.0.push(value);
            }
        }
        fn pop(&mut self) -> Option<u8> {
            self.0.pop()
        }
        fn len(&self) -> usize {
            self.0.len()
        }
    }

    /// Overwrites correctly but counts every push.
    pub struct LenOverflow(pub RingBuffer, pub usize);
    impl Ring for LenOverflow {
        fn push(&mut self, value: u8) {
            self.0.push(value);
            self.1 += 1;
        }
        fn pop(&mut self) -> Option<u8> {
            let value = self.0.pop();
            self.1 = self.1.saturating_sub(usize::from(value.is_some()));
            value
        }
        fn len(&self) -> usize {
            self.1
        }
    }

    /// Pops the newest item (a stack, not a queue).
    pub struct PopsNewest(pub Vec<u8>, pub usize);
    impl Ring for PopsNewest {
        fn push(&mut self, value: u8) {
            if self.0.len() == self.1 {
                self.0.remove(0);
            }
            self.0.push(value);
        }
        fn pop(&mut self) -> Option<u8> {
            self.0.pop()
        }
        fn len(&self) -> usize {
            self.0.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mutants::*;
    use super::*;
    use crate::guess;
    use proptest::test_runner::{Config, RngAlgorithm, TestError, TestRng, TestRunner};

    fn ring_buffer(capacity: usize) -> Box<dyn Ring> {
        Box::new(RingBuffer::with_capacity(capacity))
    }

    // The supplied implementations must satisfy your properties.
    proptest! {
        #[test]
        fn a_b_rle_roundtrips(data in arb_data()) {
            check_roundtrip(&Rle, &data)?;
        }

        #[test]
        fn a_c_rle_is_canonical(data in arb_data()) {
            check_canonical(&Rle, &data)?;
        }

        #[test]
        fn d_ring_buffer_matches_the_model((capacity, ops) in arb_ops()) {
            check_against_model(&ring_buffer, capacity, &ops)?;
        }
    }

    /// Runs a property with a fixed seed and returns the *shrunk* counterexample, if any.
    fn counterexample<S: Strategy>(
        strategy: S,
        property: impl Fn(S::Value) -> Result<(), TestCaseError>,
    ) -> Option<S::Value> {
        let config = Config {
            cases: 256,
            failure_persistence: None,
            ..Config::default()
        };
        let rng = TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        match TestRunner::new_with_rng(config, rng).run(&strategy, property) {
            Err(TestError::Fail(_, value)) => Some(value),
            _ => None,
        }
    }

    fn codec_killed(codec: &dyn Codec) -> bool {
        counterexample(arb_data(), |data| {
            check_roundtrip(codec, &data)?;
            check_canonical(codec, &data)
        })
        .is_some()
    }

    fn ring_killed(make: &dyn Fn(usize) -> Box<dyn Ring>) -> bool {
        counterexample(arb_ops(), |(capacity, ops)| {
            check_against_model(make, capacity, &ops)
        })
        .is_some()
    }

    #[test]
    fn m_codec_mutants_are_caught() {
        assert!(codec_killed(&DropsLast), "DropsLast survived");
        assert!(
            codec_killed(&NoSplit),
            "NoSplit survived: do you generate runs > 255?"
        );
        assert!(codec_killed(&SplitsEarly), "SplitsEarly survived");
        assert!(codec_killed(&NoMerge), "NoMerge survived");
        assert!(
            codec_killed(&EmptyMarker),
            "EmptyMarker survived: is empty input generated?"
        );
    }

    #[test]
    fn m_roundtrip_alone_misses_canonical_bugs() {
        // The same generator, with only the round-trip property: these mutants survive.
        for codec in [&SplitsEarly as &dyn Codec, &NoMerge, &EmptyMarker] {
            let found = counterexample(arb_data(), |data| check_roundtrip(codec, &data));
            assert!(found.is_none(), "{found:?}");
        }
    }

    #[test]
    fn m_ring_mutants_are_caught() {
        assert!(ring_killed(&|c| Box::new(DropsNewest(
            RingBuffer::with_capacity(c)
        ))));
        assert!(ring_killed(&|c| Box::new(LenOverflow(
            RingBuffer::with_capacity(c),
            0
        ))));
        assert!(ring_killed(&|c| Box::new(PopsNewest(Vec::new(), c))));
    }

    #[test]
    fn m_counterexamples_are_shrunk() {
        let found = counterexample(arb_data(), |data| check_roundtrip(&DropsLast, &data));
        let data = found.expect("DropsLast must fail");
        assert_eq!(
            data.len(),
            1,
            "shrinking should reach one byte, got {data:?}"
        );
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    /// These use their own strategies and properties, so they work before your tasks are done.
    #[test]
    fn p_predictions() {
        let roundtrips = |codec: &dyn Codec, data: &[u8]| codec.decode(&codec.encode(data)) == data;

        // P1. DropsLast with inputs from `vec(0u8..=1, 0..40)`. The run fails, then
        // shrinks. What is the reported (shrunk) input?
        let shrunk = counterexample(proptest::collection::vec(0u8..=1, 0..40), |data| {
            prop_assert!(roundtrips(&DropsLast, &data));
            Ok(())
        });
        let p1: Option<Vec<u8>> = guess();
        assert_eq!(
            p1, shrunk,
            "P1: shrinking removes elements, then simplifies values"
        );

        // P2. NoSplit with one run: (byte, length) from `(any::<u8>(), 0..1000)`.
        // Which (byte, length) does shrinking report?
        let shrunk = counterexample((any::<u8>(), 0usize..1000), |(byte, n)| {
            prop_assert!(roundtrips(&NoSplit, &vec![byte; n]));
            Ok(())
        });
        let p2: Option<(u8, usize)> = guess();
        assert_eq!(
            p2, shrunk,
            "P2: numbers shrink toward 0, stopping at the boundary"
        );
    }
}
