use num_prime::buffer::NaiveBuffer;
use project_euler::{project_euler_solution, utils::number_length};
use std::collections::HashSet;

project_euler_solution!(035);

/// All rotations of a number.
/// # Example
/// ```
/// let mut rotations = rotations(197);
/// assert_eq!(rotations.next(), Some(197));
/// assert_eq!(rotations.next(), Some(719));
/// assert_eq!(rotations.next(), Some(971));
/// assert_eq!(rotations.next(), None);
/// ```
fn rotations(mut num: u32) -> impl Iterator<Item = u32>
{
    let mut num_len = number_length(num) as u32;
    let pow = 10_u32.pow(num_len - 1);

    std::iter::from_fn(move || {
        if num_len == 0
        {
            return None;
        }

        let result = num;
        num = num % 10 * pow + num / 10;

        num_len -= 1;

        Some(result)
    })
}

/// # Circular Primes
/// The number, 197, is called a circular prime because all rotations of the
/// digits: 197, 971, and 719, are themselves prime.
///
/// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37,
/// 71, 73, 79, and 97.
///
/// How many circular primes are there below one million?
#[allow(clippy::explicit_iter_loop)]
fn project_euler_035() -> usize
{
    const UPPER_LIMIT: u64 = 1_000_000;

    let primes: HashSet<u32> = NaiveBuffer::new()
        .primes(UPPER_LIMIT)
        .map(|p| *p as u32)
        .collect();

    let mut tally = 0;
    for &num in primes.iter()
    {
        if rotations(num).all(|rotated| primes.contains(&rotated))
        {
            tally += 1;
        }
    }

    tally
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_rotations()
    {
        let mut rotations = rotations(197);
        assert_eq!(rotations.next(), Some(197));
        assert_eq!(rotations.next(), Some(719));
        assert_eq!(rotations.next(), Some(971));
        assert_eq!(rotations.next(), None);
    }
}
