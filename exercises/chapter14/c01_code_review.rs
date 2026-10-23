//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter14 c01_
//! Then: cargo clippy --test chapter14
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and some behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every needless allocation, and every closure that
//!    captures or clones more than it needs.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. You MAY change parameter and return types where the doc comment
//!    says so; the tests are written to accept the better signature.
//! 4. Run clippy and resolve its warnings for this file.

/// Sort users oldest first; users of equal age by name, A to Z.
/// You may change the parameter type.
fn sort_users(users: &mut Vec<(String, u32)>) {
    users.sort_by_key(|user| (user.1, user.0.clone()));
}

/// Return a filter that accepts a word when it contains every one of
/// `required` (case-sensitive). No requirements accept every word.
/// You may change the return type.
fn contains_all(required: Vec<String>) -> Box<dyn Fn(&str) -> bool> {
    Box::new(move |word| {
        let required = required.clone();
        required.iter().any(|r| word.contains(r.as_str()))
    })
}

/// Apply each step in order to `input` and return the final value.
/// You may change the parameter type.
fn pipeline(input: i32, steps: &Vec<Box<dyn Fn(i32) -> i32>>) -> i32 {
    let mut values = vec![input];
    for i in 0..steps.len() {
        let next = steps[i](values[values.len() - 1]);
        values.push(next);
    }
    *values.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;

    fn users(items: &[(&str, u32)]) -> Vec<(String, u32)> {
        items.iter().map(|&(n, a)| (n.to_string(), a)).collect()
    }

    #[test]
    fn a_oldest_first_then_by_name() {
        let mut list = users(&[("bo", 30), ("al", 30), ("cy", 41), ("di", 7)]);
        sort_users(&mut list);
        assert_eq!(
            list,
            users(&[("cy", 41), ("al", 30), ("bo", 30), ("di", 7)])
        );
        let mut empty: Vec<(String, u32)> = Vec::new();
        sort_users(&mut empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn a_sorting_compares_borrowed_names() {
        let mut list: Vec<(String, u32)> = (0..200).map(|i| (format!("user{i}"), i % 7)).collect();
        let (_, allocations) = count(|| sort_users(&mut list));
        assert!(
            allocations <= 2,
            "sorting 200 users cost {allocations} allocations"
        );
    }

    #[test]
    fn b_filter_requires_every_substring() {
        let filter = contains_all(vec!["ab".into(), "c".into()]);
        assert!(filter("cab"));
        assert!(!filter("ab"));
        assert!(!filter("c"));
        assert!(contains_all(vec![])("anything"));
        assert!(contains_all(vec!["é".into()])("café"));
    }

    #[test]
    fn b_filter_calls_do_not_allocate() {
        let filter = contains_all(vec!["ab".into(), "c".into()]);
        let (hits, allocations) = count(|| (0..100).filter(|_| filter("cab")).count());
        assert_eq!(hits, 100);
        assert_eq!(allocations, 0, "the closure already owns its requirements");
    }

    #[test]
    fn c_pipeline_applies_steps_in_order() {
        let steps: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(|x| x + 1), Box::new(|x| x * 10)];
        assert_eq!(pipeline(4, &steps), 50);
        let no_steps: Vec<Box<dyn Fn(i32) -> i32>> = Vec::new();
        assert_eq!(pipeline(4, &no_steps), 4);
    }

    #[test]
    fn c_pipeline_keeps_no_history() {
        let steps: Vec<Box<dyn Fn(i32) -> i32>> =
            (0..50).map(|_| Box::new(|x| x + 1) as _).collect();
        let (out, allocations) = count(|| pipeline(0, &steps));
        assert_eq!(out, 50);
        assert_eq!(allocations, 0, "only the current value is needed");
    }
}
