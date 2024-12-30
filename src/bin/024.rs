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
fn next_permutation<T: Ord>(arr: &mut [T]) -> bool
{
    let last = arr.len() - 1;

    // Find the first index 'i' such that arr[i] < arr[i + 1].
    let i = std::iter::from_fn(|| -> Option<usize> {
        (0..last)
            .rev()
            .find(|&i| arr[i] < arr[i + 1])
    })
    .next();

    let i = match i
    {
        | Some(index) => index,
        | None => return false,
    };

    // Find the smallest element greater than arr[i] to the right of
    // 'i'.
    let j = std::iter::from_fn(|| -> Option<usize> {
        (i + 1..arr.len())
            .rev()
            .find(|&j| arr[j] > arr[i])
    })
    .next()
    .expect("No valid j found");

    arr.swap(i, j);
    arr[i + 1..].reverse();
    true
}

fn project_euler_024() -> String
{
    let mut digits = "0123456789".chars().collect::<Vec<_>>();
    for _ in 1..1_000_000
    {
        next_permutation(&mut digits);
    }

    digits.into_iter().collect()
}
