//! DRILL 13-02 · Calling drop yourself
//!
//! PREDICT without compiling: what happens when this program is built and run?
//! Answer ok, panic, error, or the error code(s), e.g. E0382.
//! EXPECT: ?
//!
//! Run `cargo drill ch13`. Then FIX it with the smallest change that keeps
//! every line marked KEEP and makes the program run cleanly.
//! Ask: what would stop the destructor from running a second time at the end of scope?

struct Connection(&'static str);

impl Drop for Connection {
    fn drop(&mut self) {
        println!("closed {}", self.0);
    }
}

fn main() {
    let conn = Connection("db");
    conn.drop();
    println!("after close"); // KEEP
}
