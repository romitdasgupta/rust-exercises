//! CODE REVIEW 01 — A teammate's FFI wrapper pull request
//! Run: cargo test --test chapter24 c01_
//! Then: cargo clippy --test chapter24
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and most behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every unnecessary allocation, every `unwrap()` or cast
//!    that hides an error, and every way a panic could reach the C boundary.
//! 2. Run the tests. Compare what they found with your review.
//! 3. Fix the code below the mock library. Keep the three signatures.
//! 4. Run clippy with three extra lints and resolve the warnings for this file:
//!    `cargo clippy --test chapter24 -- -W clippy::undocumented_unsafe_blocks
//!    -W clippy::ptr_as_ptr -W clippy::cast_possible_truncation`

use std::cell::Cell;
use std::ffi::{CStr, CString, c_char, c_int, c_uint, c_void};

// ---- Supplied mock C library: not under review, keep it unchanged. ----

thread_local! {
    static LAST_ERROR: Cell<&'static CStr> = const { Cell::new(c"") };
}

const LIB_VALUES: [c_int; 20] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4];

/// Parse a decimal port (1..=65535). Returns 0 and writes `*out` on success,
/// 1 if the library is busy (try again later; it treats the text "busy" as
/// busy), and -1 for anything else. On failure it sets the thread's last
/// error message and leaves `*out` untouched.
/// # Safety: `text` is NUL-terminated and `out` is writable for one c_uint.
unsafe extern "C" fn lib_parse_port(text: *const c_char, out: *mut c_uint) -> c_int {
    // SAFETY: per the contract, text is a valid C string.
    let text = unsafe { CStr::from_ptr(text) }.to_bytes();
    if text == b"busy" {
        LAST_ERROR.set(c"busy, try again");
        return 1;
    }
    let port = std::str::from_utf8(text)
        .ok()
        .and_then(|t| t.parse::<u16>().ok())
        .filter(|&p| p != 0);
    match port {
        Some(p) => {
            // SAFETY: per the contract, out is writable.
            unsafe { out.write(c_uint::from(p)) };
            0
        }
        None => {
            LAST_ERROR.set(c"not a port");
            -1
        }
    }
}

/// Copy the last error message into `buf`, truncated to fit and always
/// NUL-terminated. Returns the number of message bytes written, without the NUL.
/// # Safety: `buf` is writable for `cap` bytes, and `cap >= 1`.
unsafe extern "C" fn lib_last_error(buf: *mut c_char, cap: usize) -> usize {
    let msg = LAST_ERROR.get().to_bytes();
    let n = msg.len().min(cap - 1);
    // SAFETY: n + 1 <= cap bytes are written; the regions cannot overlap.
    unsafe {
        std::ptr::copy_nonoverlapping(msg.as_ptr().cast::<c_char>(), buf, n);
        buf.add(n).write(0);
    }
    n
}

/// Number of values `lib_each` will report.
fn lib_count() -> usize {
    LIB_VALUES.len()
}

/// Call `each(value, ctx)` once per value, in order.
/// # Safety: `ctx` must be valid for every call of `each`.
unsafe extern "C" fn lib_each(each: extern "C" fn(c_int, *mut c_void), ctx: *mut c_void) {
    for value in LIB_VALUES {
        each(value, ctx);
    }
}

// ---- The pull request under review. ----

/// Parse `text` as a port through the library. On failure return the
/// library's error message; text containing NUL gives Err("interior NUL").
fn parse_port(text: &str) -> Result<u16, String> {
    let c_text = CString::new(text).unwrap();
    let mut out: c_uint = 0;
    let status = unsafe { lib_parse_port(c_text.as_ptr(), &mut out) };
    if status < 0 {
        return Err(last_error());
    }
    Ok(out as u16)
}

/// Copy the library's last error message into an owned String.
fn last_error() -> String {
    let mut buf = vec![0u8; 64];
    unsafe { lib_last_error(buf.as_mut_ptr() as *mut c_char, buf.len()) };
    String::from_utf8_lossy(&buf).to_string()
}

/// Collect every value the library reports, in order.
fn all_values() -> Vec<c_int> {
    extern "C" fn collect(value: c_int, ctx: *mut c_void) {
        let out = unsafe { &mut *(ctx as *mut Vec<c_int>) };
        out.push(value);
    }
    let mut values = Vec::new();
    unsafe { lib_each(collect, &mut values as *mut Vec<c_int> as *mut c_void) };
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;

    #[test]
    fn a_valid_ports_parse() {
        assert_eq!(parse_port("8080"), Ok(8080));
        assert_eq!(parse_port("65535"), Ok(65535));
    }

    #[test]
    fn a_busy_is_an_error_not_port_zero() {
        assert_eq!(parse_port("busy"), Err("busy, try again".to_string()));
    }

    #[test]
    fn a_invalid_text_reports_the_message() {
        assert_eq!(parse_port("http"), Err("not a port".to_string()));
        assert_eq!(parse_port("0"), Err("not a port".to_string()));
        assert_eq!(parse_port("65536"), Err("not a port".to_string()));
    }

    #[test]
    fn a_interior_nul_is_an_error_not_a_panic() {
        assert_eq!(parse_port("80\08"), Err("interior NUL".to_string()));
    }

    #[test]
    fn b_last_error_allocates_only_its_result() {
        let _ = parse_port("http");
        let (message, allocations) = count(last_error);
        assert_eq!(message, "not a port");
        assert!(
            allocations <= 1,
            "only the returned String; got {allocations}"
        );
    }

    #[test]
    fn c_all_values_arrive_in_order() {
        assert_eq!(all_values(), LIB_VALUES);
    }

    #[test]
    fn c_all_values_allocates_once() {
        let (values, allocations) = count(all_values);
        assert_eq!(values.len(), lib_count());
        assert!(
            allocations <= 1,
            "the library says how many values to expect; got {allocations}"
        );
    }
}
