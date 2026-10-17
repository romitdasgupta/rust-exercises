#![allow(dead_code, unused_variables, unused_imports)]
use rand::{
    SeedableRng,
    rngs::StdRng,
    seq::{IndexedRandom, SliceRandom},
};
/// A. Choose one borrowed-slice value using a fresh seeded StdRng; empty -> None.
fn choose_one(values: &[i32], seed: u64) -> Option<i32> {
    todo!("Task 1: implement the documented contract")
}
/// B. Shuffle an owned vector in place using a fresh seeded StdRng.
fn shuffled(mut values: Vec<i32>, seed: u64) -> Vec<i32> {
    todo!("Task 2: implement the documented contract")
}
/// C. Sample min(amount,len) distinct positions; input values are unique.
fn sampled(values: &[i32], amount: usize, seed: u64) -> Vec<i32> {
    todo!("Task 3: implement the documented contract")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_choice_is_in_input_repeatable_and_empty_safe() {
        assert_eq!(choose_one(&[], 7), None);
        assert_eq!(choose_one(&[9], 7), Some(9));
        let values = [2, 5, 8, 11];
        let first = choose_one(&values, 42).unwrap();
        assert!(values.contains(&first));
        assert_eq!(choose_one(&values, 42), Some(first));
    }
    #[test]
    fn b_shuffle_preserves_multiplicity_and_seeded_repeatability() {
        let input = vec![1, 2, 2, 3, 4];
        let output = shuffled(input.clone(), 99);
        assert_eq!(output, shuffled(input.clone(), 99));
        let mut sorted = output;
        sorted.sort();
        assert_eq!(sorted, input);
        assert!(shuffled(vec![], 0).is_empty());
    }
    #[test]
    fn c_sample_is_unique_bounded_and_repeatable() {
        let input = [1, 2, 3, 4, 5];
        let result = sampled(&input, 3, 42);
        assert_eq!(result.len(), 3);
        assert!(result.iter().all(|n| input.contains(n)));
        assert_eq!(
            result
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            3
        );
        assert_eq!(result, sampled(&input, 3, 42));
        assert_eq!(sampled(&input, 99, 7).len(), 5);
        assert!(sampled(&input, 0, 7).is_empty());
        assert!(sampled(&[], 3, 7).is_empty());
    }
}
