//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter10 c01_
//! Then: cargo clippy --test chapter10
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and some behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every pattern that matches more than it should, and
//!    every unnecessary clone.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix it. You MAY change parameter and return types where the doc comment
//!    says so; the tests are written to accept the better signature.
//! 4. Run clippy and resolve its warnings for this file.

#[derive(Debug, Clone, PartialEq)]
enum Event {
    Login { user: String },
    Upload { user: String, bytes: u64 },
    Logout { user: String },
    Error(String),
}

/// A. The user an event belongs to; None for errors. You may change the
/// return type.
fn user(event: &Event) -> Option<String> {
    match event.clone() {
        Event::Login { user } => Some(user),
        Event::Upload { user, .. } => Some(user),
        _ => None,
    }
}

/// B. Total bytes uploaded by `who`. You may change the parameter type.
fn uploaded_by(events: &Vec<Event>, who: &str) -> u64 {
    let mut total = 0;
    for i in 0..events.len() {
        match events[i].clone() {
            Event::Upload { user, bytes } => {
                if user == who.to_string() {
                    total += bytes;
                }
            }
            _ => {}
        }
    }
    total
}

/// C. How many events need an audit: every Error, plus any event by the
/// `admin` account. Keep this signature.
fn audit_count(events: &[Event], admin: &str) -> usize {
    events
        .iter()
        .filter(|event| match event {
            Event::Error(_) => true,
            Event::Login { user: admin }
            | Event::Upload { user: admin, .. }
            | Event::Logout { user: admin } => true,
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;

    fn login(user: &str) -> Event {
        Event::Login { user: user.into() }
    }
    fn upload(user: &str, bytes: u64) -> Event {
        Event::Upload {
            user: user.into(),
            bytes,
        }
    }
    fn logout(user: &str) -> Event {
        Event::Logout { user: user.into() }
    }
    /// Accepts both the starting Option<String> and a borrowed Option<&str>.
    fn text<S: AsRef<str>>(value: &Option<S>) -> Option<&str> {
        value.as_ref().map(|s| s.as_ref())
    }

    #[test]
    fn a_every_user_event_has_a_user() {
        assert_eq!(text(&user(&login("ann"))), Some("ann"));
        assert_eq!(text(&user(&upload("bob", 1))), Some("bob"));
        assert_eq!(text(&user(&logout("cy"))), Some("cy"));
        assert_eq!(text(&user(&Event::Error("disk".into()))), None);
    }

    #[test]
    fn a_user_borrows_from_the_event() {
        let e = upload("ann", 10);
        let (len, allocations) = count(|| text(&user(&e)).map(str::len));
        assert_eq!(len, Some(3));
        assert_eq!(allocations, 0, "the user can be borrowed from the event");
    }

    #[test]
    fn b_uploads_are_summed_per_user() {
        let events = vec![
            upload("ann", 10),
            login("ann"),
            upload("bob", 5),
            upload("ann", 32),
        ];
        assert_eq!(uploaded_by(&events, "ann"), 42);
        assert_eq!(uploaded_by(&events, "bob"), 5);
        assert_eq!(uploaded_by(&events, "nobody"), 0);
        assert_eq!(uploaded_by(&Vec::new(), "ann"), 0);
    }

    #[test]
    fn b_summing_does_not_copy_events() {
        let events: Vec<Event> = (0..50).map(|i| upload("ann", i)).collect();
        let (total, allocations) = count(|| uploaded_by(&events, "ann"));
        assert_eq!(total, (0..50).sum());
        assert_eq!(allocations, 0, "summing cost {allocations} allocations");
    }

    #[test]
    fn c_audit_counts_errors_and_admin_events_only() {
        let events = vec![
            login("ann"),
            upload("root", 1),
            Event::Error("disk".into()),
            logout("bob"),
            logout("root"),
        ];
        assert_eq!(audit_count(&events, "root"), 3);
        assert_eq!(audit_count(&events, "ann"), 2);
        assert_eq!(audit_count(&[login("ann")], "root"), 0);
        assert_eq!(audit_count(&[], "root"), 0);
    }
}
