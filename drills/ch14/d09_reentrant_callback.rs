//! DRILL 14-09 · A callback that re-enters its owner
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch14`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: who holds a RefCell borrow while the callback runs?

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let log: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
    let sink = Rc::clone(&log);
    let record = move |msg: &str| sink.borrow_mut().push(msg.to_string());
    record("started"); // KEEP
    record("stopped"); // KEEP
    for entry in log.borrow().iter() {
        record(&format!("echo:{entry}"));
    }
    assert_eq!(log.borrow().len(), 4); // KEEP
}
