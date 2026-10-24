//! LESSON 04 — Extension traits and qualified calls
//! Run: cargo test --test chapter11 l04_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Can this crate implement its own trait for
//! [i32]? Can it implement Display for Vec<i32>?
//!
//! IMPLEMENT the documented tasks, then keep the tests and add an edge case.
//! EXPLAIN afterward: How do coherence and the orphan rule preserve unambiguous
//! implementations across crates? Why can importing two traits make method syntax
//! ambiguous?
//!
//! COMPILER / MANUAL LAB (use examples/ch11_scratch.rs):
//! In scratch, define two traits with the same method name and implement both for one type.
//! Trigger ambiguity and resolve it with <Type as Trait>::method. Try impl Display for
//! Vec<i32> and then repair it with a newtype.
//!
//! TRANSFER: invent a new edge case, predict it, then add a check. A lesson is
//! complete only when its tests, predictions, lab, and explanation are done.

trait SliceStats {
    fn span(&self) -> Option<i64>;
}
impl SliceStats for [i32] {
    /// A. max - min in i64; return None for an empty slice. Never overflow i32.
    fn span(&self) -> Option<i64> {
        todo!("Task 1: implement the documented contract")
    }
}
trait Warm {
    fn tone(&self) -> &'static str;
}
trait Cool {
    fn tone(&self) -> &'static str;
}
struct Palette;
impl Warm for Palette {
    fn tone(&self) -> &'static str {
        "amber"
    }
}
impl Cool for Palette {
    fn tone(&self) -> &'static str {
        "blue"
    }
}
/// B. Call each trait explicitly and return warm/cool separated by a literal slash.
fn tones(value: &Palette) -> String {
    todo!("Task 2: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    use practice::guess;
    #[test]
    #[allow(clippy::useless_vec)] // The vector is the point: Vec<i32> derefs to [i32].
    fn a_extension_accepts_arrays_vectors_and_extremes() {
        assert_eq!([].as_slice().span(), None);
        assert_eq!([5, 5].span(), Some(0));
        assert_eq!(vec![-3, 8, 1].span(), Some(11));
        assert_eq!([i32::MIN, i32::MAX].span(), Some(4_294_967_295));
    }
    #[test]
    fn b_qualified_calls_disambiguate() {
        assert_eq!(tones(&Palette), "amber/blue");
    }

    #[test]
    fn a_span_ignores_position_and_handles_one_element() {
        assert_eq!([7].span(), Some(0));
        assert_eq!([i32::MAX, 0, i32::MIN].span(), Some(4_294_967_295));
        assert_eq!([3, -1, 2].span(), Some(4));
    }

    struct Gauge;
    impl Gauge {
        fn label(&self) -> &'static str {
            "inherent"
        }
    }
    trait Show {
        fn label(&self) -> &'static str;
    }
    impl Show for Gauge {
        fn label(&self) -> &'static str {
            "trait"
        }
    }

    trait Shout {
        fn shout(&self) -> String;
    }
    impl Shout for str {
        fn shout(&self) -> String {
            self.to_uppercase() + "!"
        }
    }

    trait Kind {
        fn kind(&self) -> &'static str;
    }
    impl Kind for [i32] {
        fn kind(&self) -> &'static str {
            "slice"
        }
    }
    impl Kind for Vec<i32> {
        fn kind(&self) -> &'static str {
            "vec"
        }
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Gauge has an inherent `label` AND a trait `label`. Which runs for each call?
        let p1: (&str, &str) = guess();
        assert_eq!(
            p1,
            (Gauge.label(), Show::label(&Gauge)),
            "P1: inherent methods win"
        );

        // P2. An extension method on str, called on German text.
        let p2: String = guess();
        assert_eq!(
            p2,
            "straße".shout(),
            "P2: Unicode uppercase in an extension"
        );

        // P3. Method lookup on a Vec and on an array, with impls for Vec<i32> and [i32].
        let p3: (&str, &str) = guess();
        assert_eq!(p3, (vec![1].kind(), [1].kind()), "P3: method lookup order");
    }
}
