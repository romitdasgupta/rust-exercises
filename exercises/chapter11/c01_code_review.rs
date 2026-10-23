//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter11 c01_
//! Then: cargo clippy --test chapter11
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and some behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every unnecessary allocation, and every bound or trait
//!    object that the code does not need.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. You MAY change parameter types, return types, and bounds where the
//!    doc comment says so; the tests are written to accept the better signature.
//! 4. Run clippy and resolve its warnings for this file.

use std::fmt::Display;

trait Metric {
    fn name(&self) -> &str;
    fn value(&self) -> f64;
}

#[derive(Clone)]
struct Gauge {
    name: String,
    value: f64,
}

impl Metric for Gauge {
    fn name(&self) -> &str {
        &self.name
    }
    fn value(&self) -> f64 {
        self.value
    }
}

/// Names of the metrics whose value is strictly above `limit`, in input order.
/// You may change the parameter type, and return any iterator of names.
fn above<'a>(
    metrics: &'a Vec<Box<dyn Metric>>,
    limit: f64,
) -> Box<dyn Iterator<Item = String> + 'a> {
    Box::new(
        metrics
            .iter()
            .filter(move |m| m.value() >= limit)
            .map(|m| m.name().to_string()),
    )
}

/// The largest item; ties go to the earliest. None for an empty slice.
/// You may change the bounds and the return type.
fn largest<T: PartialOrd + Clone + Display>(items: &[T]) -> Option<T> {
    if items.is_empty() {
        return None;
    }
    let mut best = items[0].clone();
    for i in 0..items.len() {
        if items[i] >= best {
            best = items[i].clone();
        }
    }
    Some(best)
}

/// One "name=value" line per metric, each ending in '\n'.
/// Keep the parameter shape (`&[M]`); you may relax the bounds.
fn render<M: Metric + Clone>(metrics: &[M]) -> String {
    let mut out = String::new();
    for m in metrics.to_vec() {
        out = out + &format!("{}={}\n", m.name(), m.value());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    use std::hint::black_box;

    fn gauge(name: &str, value: f64) -> Gauge {
        Gauge {
            name: name.into(),
            value,
        }
    }

    fn boxed() -> Vec<Box<dyn Metric>> {
        vec![
            Box::new(gauge("cpu", 1.0)),
            Box::new(gauge("disk", 2.0)),
            Box::new(gauge("net", 3.0)),
        ]
    }

    #[derive(Clone)]
    struct Score {
        points: u32,
        who: &'static str,
    }
    impl PartialEq for Score {
        fn eq(&self, other: &Self) -> bool {
            self.points == other.points
        }
    }
    impl PartialOrd for Score {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.points.partial_cmp(&other.points)
        }
    }
    impl Display for Score {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.who, self.points)
        }
    }

    #[test]
    fn a_above_is_strict_and_keeps_order() {
        let metrics = boxed();
        assert_eq!(above(&metrics, 1.0).collect::<Vec<_>>(), ["disk", "net"]);
        assert_eq!(
            above(&metrics, 0.5).collect::<Vec<_>>(),
            ["cpu", "disk", "net"]
        );
        assert_eq!(above(&metrics, 3.0).count(), 0);
    }

    #[test]
    fn a_above_borrows_names_without_boxing() {
        let metrics = boxed();
        let (n, allocations) = count(|| above(&metrics, 0.0).map(black_box).count());
        assert_eq!(n, 3);
        assert_eq!(
            allocations, 0,
            "filtering can return borrowed names from an unboxed iterator"
        );
    }

    #[test]
    fn b_largest_keeps_the_first_tie() {
        let scores = [
            Score {
                points: 3,
                who: "first",
            },
            Score {
                points: 1,
                who: "low",
            },
            Score {
                points: 3,
                who: "second",
            },
        ];
        assert_eq!(largest(&scores).map(|s| s.who), Some("first"));
        assert_eq!(largest::<Score>(&[]).map(|s| s.who), None);
    }

    #[test]
    fn b_largest_does_not_clone() {
        let names = vec!["ab".to_string(), "zz".to_string(), "m".to_string()];
        let (len, allocations) = count(|| black_box(largest(black_box(&names))).map(|s| s.len()));
        assert_eq!(len, Some(2));
        assert_eq!(
            allocations, 0,
            "a search can return a borrow into its input"
        );
    }

    #[test]
    fn c_render_writes_one_line_per_metric() {
        let gauges = vec![gauge("cpu", 0.5), gauge("é", -2.0)];
        assert_eq!(render(&gauges), "cpu=0.5\né=-2\n");
        assert_eq!(render::<Gauge>(&[]), "");
    }

    #[test]
    fn c_render_writes_into_one_growing_string() {
        let gauges: Vec<Gauge> = (0..100).map(|i| gauge(&format!("m{i}"), 1.5)).collect();
        let (text, allocations) = count(|| render(&gauges));
        assert!(text.starts_with("m0=1.5\nm1=1.5\n"));
        assert!(
            allocations <= 16,
            "100 lines cost {allocations} allocations"
        );
    }
}
