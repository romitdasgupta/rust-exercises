//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter05 c01_
//! Then: cargo clippy --test chapter05
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and most behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every clone that only exists to quiet the borrow
//!    checker, and every signature that asks for more than it needs.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. You MAY change parameter and return types where the doc comment
//!    says so; the tests are written to accept the better signature.
//! 4. Run clippy and resolve its warnings for this file.

/// Return the trimmed value of the first "key = value" line whose trimmed key
/// equals `key`. Lines without '=' (comments, blank lines) are skipped.
/// None if no line matches. You may change the parameter and return types.
fn config_value(lines: &Vec<String>, key: &String) -> Option<String> {
    for line in lines.clone() {
        let (k, v) = line.split_once('=').unwrap();
        if k.trim() == key.as_str() {
            return Some(v.trim().to_string());
        }
    }
    None
}

/// Replace every label that is exactly `old` with `new`, in place. Return how
/// many labels changed. You may change the parameter types.
fn rename_all(labels: &mut Vec<String>, old: &String, new: &String) -> usize {
    let mut changed = 0;
    for i in 0..labels.len() {
        if labels[i].clone().contains(old.as_str()) {
            labels[i] = new.clone();
            changed += 1;
        }
    }
    changed
}

/// Move the first label equal to `name` to the end, keeping the others in
/// order. Return whether it was found. Keep this signature.
fn bump_to_end(labels: &mut Vec<String>, name: &str) -> bool {
    // A borrow of `labels` can't be held while we remove and push,
    // so iterate over a snapshot instead.
    let snapshot = labels.clone();
    for (i, label) in snapshot.iter().enumerate() {
        if label == name {
            labels.remove(i);
            labels.push(label.clone());
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;
    use std::hint::black_box;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    // Accepts Option<String> and Option<&str>, so either signature works.
    fn text(value: Option<impl AsRef<str>>) -> Option<String> {
        value.map(|v| v.as_ref().to_string())
    }

    #[test]
    fn a_config_finds_the_first_match() {
        let lines = strings(&["host = example.org", "port = 8080", "port = 9"]);
        let (port, user) = (String::from("port"), String::from("user"));
        assert_eq!(text(config_value(&lines, &port)).as_deref(), Some("8080"));
        assert_eq!(text(config_value(&lines, &user)), None);
        let lines = strings(&["url = https://x.test/?a=b"]);
        let url = String::from("url");
        assert_eq!(
            text(config_value(&lines, &url)).as_deref(),
            Some("https://x.test/?a=b")
        );
    }

    #[test]
    fn a_config_skips_lines_without_equals() {
        let lines = strings(&["# settings", "", "mode = fast"]);
        let mode = String::from("mode");
        assert_eq!(text(config_value(&lines, &mode)).as_deref(), Some("fast"));
    }

    #[test]
    fn a_config_borrows_instead_of_copying() {
        let lines: Vec<String> = (0..50).map(|i| format!("key{i} = value{i}")).collect();
        let key = String::from("key42");
        let (found, allocations) = count(|| black_box(config_value(&lines, &key)).is_some());
        assert!(found);
        assert_eq!(
            allocations, 0,
            "a lookup can borrow its input and return a borrow into it"
        );
    }

    #[test]
    fn b_rename_counts_changes() {
        let mut labels = strings(&["old", "keep", "old"]);
        let (old, new) = ("old".to_string(), "new".to_string());
        assert_eq!(rename_all(&mut labels, &old, &new), 2);
        assert_eq!(labels, ["new", "keep", "new"]);
    }

    #[test]
    fn b_rename_matches_whole_labels_only() {
        let mut labels = strings(&["old", "older", "bold"]);
        let (old, new) = ("old".to_string(), "new".to_string());
        assert_eq!(rename_all(&mut labels, &old, &new), 1);
        assert_eq!(labels, ["new", "older", "bold"]);
    }

    #[test]
    fn b_rename_does_not_clone_every_label() {
        let mut labels: Vec<String> = (0..50)
            .map(|i| {
                if i % 10 == 0 {
                    "old".into()
                } else {
                    format!("label{i}")
                }
            })
            .collect();
        let (old, new) = ("old".to_string(), "new".to_string());
        let (changed, allocations) = count(|| rename_all(&mut labels, &old, &new));
        assert_eq!(changed, 5);
        assert!(
            allocations <= 5,
            "50 labels with 5 matches cost {allocations} allocations"
        );
    }

    #[test]
    fn c_bump_moves_the_match_to_the_end() {
        let mut labels = strings(&["a", "b", "c", "b"]);
        assert!(bump_to_end(&mut labels, "b"));
        assert_eq!(labels, ["a", "c", "b", "b"]);
        assert!(!bump_to_end(&mut labels, "z"));
        assert_eq!(labels, ["a", "c", "b", "b"]);
        assert!(!bump_to_end(&mut Vec::new(), "a"));
    }

    #[test]
    fn c_bump_moves_the_string_instead_of_cloning() {
        let mut labels: Vec<String> = (0..50).map(|i| format!("label{i}")).collect();
        let p = labels[3].as_ptr();
        let (found, allocations) = count(|| bump_to_end(&mut labels, "label3"));
        assert!(found);
        assert_eq!(labels[49], "label3");
        assert_eq!(labels[49].as_ptr(), p, "the String itself should move");
        assert_eq!(
            allocations, 0,
            "{allocations} allocations to move one label"
        );
    }
}
