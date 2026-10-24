//! LESSON E02 — serde: derive, attributes, borrowing, and validated types (after Chapters 11 and 13)
//! Run: cargo test --manifest-path ecosystem/Cargo.toml e02_
//!
//! PREDICT before running: first answer `p_predictions` at the bottom (replace each
//! `guess()` with your answer). Then consider: Which JSON shape does a plain derived
//! enum use? Can a derived `Deserialize` break an invariant your constructor enforces?
//! IMPLEMENT the tasks below. The types already derive `Serialize`/`Deserialize`;
//! your job is mostly attributes and two hand-written impls. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Why does `&'a str` fail for some valid JSON strings while
//! `Cow<'a, str>` with `#[serde(borrow)]` never does? When would you hand-write `Deserialize`
//! rather than use `#[serde(try_from = "...")]`?
//! JUDGMENT: serde is the de-facto standard. It separates the data model (your types)
//! from formats (serde_json, toml, bincode, ...). Your wire format is a public API:
//! attributes such as `deny_unknown_fields` and `default` are decisions about compatibility.

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::collections::BTreeMap;

/// A. One service in a deployment manifest. JSON contract:
/// - field names are camelCase on the wire (`replicaCount`, `healthCheck`);
/// - `port` may be omitted and then defaults to `default_port()`;
/// - `tags` may be omitted (empty) and is left out of the output when empty;
/// - `cache_hits` is runtime state: never read from or written to JSON;
/// - unknown fields are an error, so a typo such as `replicas` cannot silently
///   fall back to a default.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub replica_count: u32,
    pub port: u16,
    pub tags: Vec<String>,
    pub health_check: HealthCheck,
    pub cache_hits: u64,
}

fn default_port() -> u16 {
    8080
}

/// A. Health checks are internally tagged by a `kind` field with snake_case names:
/// `{"kind":"http","path":"/healthz"}`, `{"kind":"tcp","port":5432}`, `{"kind":"none"}`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HealthCheck {
    Http { path: String },
    Tcp { port: u16 },
    None,
}

/// B. A structured log line. `level` always borrows from the input. `msg` must borrow
/// when the JSON string has no escapes, and own a decoded copy when it has escapes.
#[derive(Debug, Deserialize)]
pub struct LogLine<'a> {
    pub level: &'a str,
    pub msg: Cow<'a, str>,
}

/// B. Parse one JSON log line, borrowing from `input`.
pub fn parse_log_line(input: &str) -> Result<LogLine<'_>, serde_json::Error> {
    todo!("Implement this task")
}

/// B. Count lines per level in newline-delimited JSON (skip blank lines). The map's
/// keys must borrow from `input`: no String per line.
pub fn count_levels(input: &str) -> Result<BTreeMap<&str, usize>, serde_json::Error> {
    todo!("Implement this task")
}

/// C. An inclusive port range whose invariant is `start <= end`. The fields are private,
/// so `new` is the only way to build one, and deserialization must not bypass it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortRange {
    start: u16,
    end: u16,
}

impl PortRange {
    pub fn new(start: u16, end: u16) -> Option<PortRange> {
        (start <= end).then_some(PortRange { start, end })
    }
    pub fn start(self) -> u16 {
        self.start
    }
    pub fn end(self) -> u16 {
        self.end
    }
}

/// C. Serialize as the JSON string `"start-end"`, for example `"8000-8080"`.
impl Serialize for PortRange {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        todo!("Implement this task")
    }
}

/// C. Deserialize from a string `"start-end"`. Accept escaped JSON strings too (so do
/// not deserialize into `&str`). Error messages (via `de::Error::custom`) must contain:
/// - ``invalid port range `TEXT` `` when the shape or a number is wrong;
/// - `port range start must not exceed end` when start > end.
impl<'de> Deserialize<'de> for PortRange {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        todo!("Implement this task")
    }
}

/// D. Serialize `value` to JSON text and deserialize it back.
pub fn roundtrip<T: Serialize + DeserializeOwned>(value: &T) -> Result<T, serde_json::Error> {
    todo!("Implement this task")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guess;
    use serde_json::json;

    fn api() -> Service {
        Service {
            name: "api".into(),
            replica_count: 3,
            port: 9000,
            tags: vec!["edge".into()],
            health_check: HealthCheck::Http {
                path: "/healthz".into(),
            },
            cache_hits: 0,
        }
    }

    #[test]
    fn a_service_matches_the_wire_contract() {
        let value = json!({
            "name": "api", "replicaCount": 3, "port": 9000, "tags": ["edge"],
            "healthCheck": {"kind": "http", "path": "/healthz"}
        });
        assert_eq!(serde_json::to_value(api()).unwrap(), value);
        assert_eq!(serde_json::from_value::<Service>(value).unwrap(), api());
    }

    #[test]
    fn a_defaults_skips_and_unknown_fields() {
        let minimal = r#"{"name":"db","replicaCount":1,"healthCheck":{"kind":"tcp","port":5432}}"#;
        let db: Service = serde_json::from_str(minimal).unwrap();
        assert_eq!((db.port, db.tags.len()), (8080, 0));
        assert_eq!(db.health_check, HealthCheck::Tcp { port: 5432 });
        let out = serde_json::to_value(&db).unwrap();
        assert!(out.get("tags").is_none(), "empty tags are not written");
        assert!(out.get("cacheHits").is_none() && out.get("cache_hits").is_none());

        let typo = r#"{"name":"db","replicas":1,"replicaCount":1,"healthCheck":{"kind":"none"}}"#;
        let err = serde_json::from_str::<Service>(typo).unwrap_err();
        assert!(
            err.to_string().contains("unknown field `replicas`"),
            "{err}"
        );
        let injected =
            r#"{"name":"x","replicaCount":1,"cacheHits":5,"healthCheck":{"kind":"none"}}"#;
        assert!(serde_json::from_str::<Service>(injected).is_err());
    }

    #[test]
    fn a_enum_is_internally_tagged() {
        assert_eq!(
            serde_json::to_value(HealthCheck::None).unwrap(),
            json!({"kind": "none"})
        );
        assert_eq!(
            serde_json::to_value(HealthCheck::Tcp { port: 1 }).unwrap(),
            json!({"kind": "tcp", "port": 1})
        );
        assert!(serde_json::from_value::<HealthCheck>(json!({"kind": "udp"})).is_err());
    }

    fn within(outer: &str, inner: &str) -> bool {
        let range = outer.as_bytes().as_ptr_range();
        range.contains(&inner.as_ptr())
    }

    #[test]
    fn b_plain_strings_are_borrowed() {
        let input = r#"{"level":"warn","msg":"disk 91% full"}"#;
        let line = parse_log_line(input).unwrap();
        assert!(within(input, line.level));
        assert!(matches!(line.msg, Cow::Borrowed(_)), "msg should borrow");
        assert!(within(input, &line.msg));
    }

    #[test]
    fn b_escaped_strings_are_owned() {
        let line = parse_log_line(r#"{"level":"info","msg":"say \"hi\""}"#).unwrap();
        assert_eq!(line.msg, r#"say "hi""#);
        assert!(matches!(line.msg, Cow::Owned(_)));
    }

    #[test]
    fn b_count_levels_borrows_keys() {
        let input = concat!(
            r#"{"level":"info","msg":"a"}"#,
            "\n\n",
            r#"{"level":"warn","msg":"b"}"#,
            "\n",
            r#"{"level":"info","msg":"c"}"#,
            "\n"
        );
        let counts = count_levels(input).unwrap();
        assert_eq!(
            counts.into_iter().collect::<Vec<_>>(),
            [("info", 2), ("warn", 1)]
        );
        assert!(
            count_levels(input)
                .unwrap()
                .keys()
                .all(|k| within(input, k))
        );
        assert!(count_levels("{not json}").is_err());
        assert!(count_levels("").unwrap().is_empty());
    }

    #[test]
    fn c_port_range_uses_its_string_form() {
        let range = PortRange::new(8000, 8080).unwrap();
        assert_eq!(serde_json::to_string(&range).unwrap(), r#""8000-8080""#);
        assert_eq!(
            serde_json::from_str::<PortRange>(r#""8000-8080""#).unwrap(),
            range
        );
        // `8` is an escaped '8': the parser must hand you a decoded, owned string.
        assert_eq!(
            serde_json::from_str::<PortRange>(r#""8000-8080""#).unwrap(),
            range
        );
        assert_eq!(
            serde_json::from_str::<PortRange>(r#""7-7""#).unwrap().end(),
            7
        );
    }

    #[test]
    fn c_port_range_rejects_invalid_input() {
        let msg = |s: &str| {
            serde_json::from_str::<PortRange>(s)
                .unwrap_err()
                .to_string()
        };
        assert!(msg(r#""9-1""#).contains("port range start must not exceed end"));
        assert!(msg(r#""80""#).contains("invalid port range `80`"));
        assert!(msg(r#""1-99999""#).contains("invalid port range `1-99999`"));
        assert!(serde_json::from_str::<PortRange>("8080").is_err());
        assert!(serde_json::from_str::<PortRange>(r#"{"start":9,"end":1}"#).is_err());
    }

    #[test]
    fn d_roundtrips_preserve_data_but_not_skipped_fields() {
        let mut service = api();
        service.cache_hits = 42;
        let back = roundtrip(&service).unwrap();
        assert_eq!(back.cache_hits, 0, "skipped fields come back as Default");
        service.cache_hits = 0;
        assert_eq!(back, service);
        let range = PortRange::new(1, 65535).unwrap();
        assert_eq!(roundtrip(&range).unwrap(), range);
        let checks = vec![
            HealthCheck::None,
            HealthCheck::Tcp { port: 0 },
            HealthCheck::Http {
                path: "/✓".into()
            },
        ];
        assert_eq!(roundtrip(&checks).unwrap(), checks);
    }

    #[derive(Serialize)]
    enum Plain {
        Tcp { port: u16 },
    }

    #[derive(Deserialize)]
    struct DerivedRange {
        start: u16,
        end: u16,
    }

    /// P. Predict BEFORE running: replace each `guess()` with your answer.
    #[test]
    fn p_predictions() {
        // P1. Can a JSON string containing an escape deserialize into `&str`?
        let p1: bool = guess();
        assert_eq!(
            p1,
            serde_json::from_str::<&str>(r#""a\"b""#).is_ok(),
            "P1: &str needs the bytes to exist unescaped in the input"
        );

        // P2. JSON text for a plain derived enum (no serde attributes)?
        let p2: &str = guess();
        assert_eq!(
            p2,
            serde_json::to_string(&Plain::Tcp { port: 1 }).unwrap(),
            "P2: the default enum representation is externally tagged"
        );

        // P3. Does a *derived* Deserialize accept start > end?
        let p3: bool = guess();
        assert_eq!(
            p3,
            serde_json::from_str::<DerivedRange>(r#"{"start":9,"end":1}"#).is_ok(),
            "P3: derive fills fields directly; it never calls your constructor"
        );

        // P4. What does serde_json write for f64::NAN?
        let p4: &str = guess();
        assert_eq!(
            p4,
            serde_json::to_string(&f64::NAN).unwrap(),
            "P4: JSON has no NaN"
        );
    }
}
