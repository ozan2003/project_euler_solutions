use project_euler::project_euler_solution;

project_euler_solution!(024);

/// # Lexicographic Permutations
///
/// A permutation is an ordered arrangement of objects. For example, 3124 is one
/// possible permutation of the digits 1, 2, 3 and 4. If all of the permutations
/// are listed numerically or alphabetically, we call it lexicographic order.
/// The lexicographic permutations of 0, 1 and 2 are:
///
/// 012   021   102   120   201   210
///
/// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4,
/// 5, 6, 7, 8 and 9?
fn project_euler_024() -> String
{
    let mut digits = "0123456789".chars().collect::<Vec<_>>();
    for _ in 1..1_000_000
    {
        next_permutation(&mut digits);
    }

    digits.into_iter().collect()
}

/// Returns the next lexicographic permutation of the sequence.
///
/// Returns `false` if the sequence is the last permutation.
///
/// Credit: <https://www.nayuki.io/page/next-lexicographical-permutation-algorithm>
fn next_permutation<T: Ord>(seq: &mut [T]) -> bool
{
    // Find non-increasing suffix
    if seq.is_empty()
    {
        return false;
    }

    let mut i: usize = seq.len() - 1;
    while i > 0 && seq[i - 1] >= seq[i]
    {
        i -= 1;
    }

    if i == 0
    {
        return false;
    }

    // Find successor to pivot
    let mut j: usize = seq.len() - 1;
    while seq[j] <= seq[i - 1]
    {
        j -= 1;
    }
    seq.swap(i - 1, j);

    // Reverse suffix
    seq[i..].reverse();

    true
}

#[test]
fn test_next_permutation()
{
    let mut nums = vec![0, 1, 2];
    assert!(next_permutation(&mut nums));
    assert_eq!(nums, vec![0, 2, 1]);

    assert!(next_permutation(&mut nums));
    assert_eq!(nums, vec![1, 0, 2]);

    assert!(next_permutation(&mut nums));
    assert_eq!(nums, vec![1, 2, 0]);

    assert!(next_permutation(&mut nums));
    assert_eq!(nums, vec![2, 0, 1]);

    assert!(next_permutation(&mut nums));
    assert_eq!(nums, vec![2, 1, 0]);

    assert!(!next_permutation(&mut nums)); // Last permutation
    assert_eq!(nums, vec![2, 1, 0]);

    let mut chars = vec!['a', 'b', 'c'];
    assert!(next_permutation(&mut chars));
    assert_eq!(chars, vec!['a', 'c', 'b']);

    let mut single = vec![1];
    assert!(!next_permutation(&mut single));
    assert_eq!(single, vec![1]);

    let mut empty: Vec<i32> = vec![];
    assert!(!next_permutation(&mut empty));
    assert_eq!(empty, vec![]);
}
