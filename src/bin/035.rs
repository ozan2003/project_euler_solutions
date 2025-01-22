use num_prime::buffer::NaiveBuffer;
use project_euler::project_euler_solution;
use std::collections::HashSet;

project_euler_solution!(035);

/// Returns the number of digits in a number.
fn num_length(num: u64) -> u32
{
    (num as f64).log10().floor() as u32 + 1
}

/// All rotations of a number.
/// # Example
/// ```
/// let mut rotations = rotations(197);
/// assert_eq!(rotations.next(), Some(197));
/// assert_eq!(rotations.next(), Some(719));
/// assert_eq!(rotations.next(), Some(971));
/// assert_eq!(rotations.next(), None);
/// ```
fn rotations(mut num: u64) -> impl Iterator<Item = u64>
{
    let mut num_len = num_length(num);
    let pow = 10_u64.pow(num_len - 1);

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

fn project_euler_035() -> usize
{
    const UPPER_LIMIT: u64 = 1_000_000;

    let primes: HashSet<u64> = NaiveBuffer::new()
        .primes(UPPER_LIMIT)
        .copied()
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

// test
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

    #[test]
    fn test_num_length()
    {
        assert_eq!(num_length(0), 1);
        assert_eq!(num_length(1), 1);
        assert_eq!(num_length(9), 1);
        assert_eq!(num_length(10), 2);
        assert_eq!(num_length(99), 2);
        assert_eq!(num_length(100), 3);
        assert_eq!(num_length(999), 3);
        assert_eq!(num_length(1000), 4);
    }
}
