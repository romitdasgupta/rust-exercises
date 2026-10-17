//! LESSON 05 — Structs that borrow
//! Run: cargo test --test chapter05 l05_
//!
//! PREDICT before running: Can a borrowed view outlive its source String? Does moving the view move its source?
//! IMPLEMENT the tasks below. Preserve the tests and add an edge case.
//! EXPLAIN afterward: Explain why the struct has a lifetime parameter and how fields remain tied to one owner.
//! COMPILER / TRANSFER LAB: Construct a view of a String in an inner scope, store the view outside, then try using it after the String drops. Move only the view while keeping the String alive and compare.
//! Use examples/ch05_scratch.rs; repair intentional failures before committing.

#[derive(Debug, PartialEq)]
struct Record<'a> {
    key: &'a str,
    value: &'a str,
}
/// A. Build a borrowed record at the first '='. Trim each side, reject an empty
/// key or a missing '='; allow an empty value. No owned String allocations.
fn parse_view(line: &str) -> Option<Record<'_>> {
    todo!("Implement this task")
}
/// B. Return the borrowed value field, preserving the SOURCE lifetime rather
/// than unnecessarily tying it to the short borrow of the Record wrapper.
fn value<'a>(record: &Record<'a>) -> &'a str {
    todo!("Implement this task")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_view_validates_without_owning() {
        let s = String::from(" mode = calm ");
        let r = parse_view(&s).unwrap();
        assert_eq!(
            r,
            Record {
                key: "mode",
                value: "calm"
            }
        );
        assert_eq!(r.key.as_ptr(), s[1..].as_ptr());
        assert_eq!(
            parse_view("x="),
            Some(Record {
                key: "x",
                value: ""
            })
        );
        assert_eq!(parse_view(" = x"), None);
        assert_eq!(parse_view("x"), None);
    }
    #[test]
    fn b_source_can_outlive_wrapper() {
        let source = String::from("k=v");
        let out;
        {
            let record = parse_view(&source).unwrap();
            out = value(&record);
        }
        assert_eq!(out, "v");
    }
}
