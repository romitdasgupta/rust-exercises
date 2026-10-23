//! CODE REVIEW 01 — A teammate's pull request
//! Run: cargo test --test chapter03 c01_
//! Then: cargo clippy --test chapter03
//!
//! This is the kind of code you will be asked to review at work. It compiles,
//! and some behavior tests pass. Your job:
//! 1. Before running anything, read the code and write a review in NOTES.md:
//!    list every bug, every overflow or Unicode hazard, and every unnecessary
//!    allocation or copy.
//! 2. Run the tests in debug AND `--release`. Compare what they found with
//!    your review. Does any bug behave differently in the two profiles?
//! 3. Fix it. You MAY change parameter types where the doc comment says so;
//!    the tests are written to accept the better signature.
//! 4. Run clippy and resolve its warnings for this file.

/// Mean of the readings, rounded down. None for an empty list. Any number
/// of readings, each up to u16::MAX, must work. You may change the parameter type.
fn mean(readings: &Vec<u16>) -> Option<u16> {
    if readings.len() == 0 {
        return None;
    }
    let mut total: u16 = 0;
    for i in 0..readings.len() {
        total += readings[i];
    }
    Some(total / readings.len() as u16)
}

/// The uppercase first letter of each whitespace-separated word:
/// "ada lovelace" becomes "AL". Keep this signature.
fn initials(name: &str) -> String {
    let mut out = String::new();
    for word in name.split_whitespace() {
        let first = &word[0..1];
        out = out + &first.to_uppercase();
    }
    out
}

/// Checksum: the sum of all bytes, wrapping modulo 256.
/// You may change the parameter type.
fn checksum(bytes: &Vec<u8>) -> u8 {
    let mut sum: u32 = 0;
    for b in bytes.clone() {
        sum = sum + b as u32;
    }
    (sum % 256) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use practice::alloc::count;

    fn owned<T: Clone>(items: &[T]) -> Vec<T> {
        items.to_vec()
    }

    #[test]
    fn a_mean_of_small_readings() {
        assert_eq!(mean(&owned(&[1, 2, 4])), Some(2));
        assert_eq!(mean(&owned(&[7])), Some(7));
        assert_eq!(mean(&owned(&[])), None);
    }

    #[test]
    fn a_mean_does_not_overflow() {
        assert_eq!(mean(&owned(&[60_000, 60_000])), Some(60_000));
        assert_eq!(mean(&owned(&[u16::MAX; 300])), Some(u16::MAX));
        assert_eq!(mean(&owned(&[1; 70_000])), Some(1));
    }

    #[test]
    fn b_initials_of_ascii_names() {
        assert_eq!(initials("ada lovelace"), "AL");
        assert_eq!(initials("  grace   hopper "), "GH");
        assert_eq!(initials(""), "");
    }

    #[test]
    fn b_initials_respect_unicode() {
        assert_eq!(initials("émile zola"), "ÉZ");
        assert_eq!(initials("李 小龙"), "李小");
        // Some letters uppercase to more than one character.
        assert_eq!(initials("ßtraße x"), "SSX");
    }

    #[test]
    fn b_initials_build_one_string() {
        let name = "a b c d e f g h i j k l m n o p";
        let (out, allocations) = count(|| initials(name));
        assert_eq!(out, "ABCDEFGHIJKLMNOP");
        assert!(
            allocations <= 6,
            "16 words cost {allocations} allocations; append chars to one String"
        );
    }

    #[test]
    fn c_checksum_wraps() {
        assert_eq!(checksum(&owned(&[200, 100])), 44);
        assert_eq!(checksum(&owned(&[255; 256])), 0);
        assert_eq!(checksum(&owned(&[])), 0);
    }

    #[test]
    fn c_checksum_reads_without_copying() {
        let bytes = owned(&[7_u8; 1_000]);
        let (sum, allocations) = count(|| checksum(&bytes));
        assert_eq!(sum, (7_000 % 256) as u8);
        assert_eq!(allocations, 0, "a checksum only needs to borrow the bytes");
    }
}
