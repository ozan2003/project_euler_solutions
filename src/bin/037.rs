#![feature(gen_blocks)]
use project_euler::project_euler_solution;
use project_euler::utils::primes::is_prime;

project_euler_solution!(037);

/// # Truncatable Primes
/// The number 3797 has an interesting property. Being prime itself, it is
/// possible to continuously remove digits from left to right, and remain prime
/// at each stage: 3797, 797, 97, and 7. Similarly we can work from right to
/// left: 3797, 379, 37, and 3.
///
/// Find the sum of the only eleven primes that are both truncatable from left
/// to right and right to left.
///
/// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
fn project_euler_037() -> u32
{
    (23..) // 23 is the first truncatable prime.
        .step_by(2) // Skip even numbers.
        .filter(|&num| is_truncatable_prime(num))
        .take(11)
        .sum()
}

// Truncate a number from left.
// 1234 -> 234 -> 34 -> 4
fn left_truncate(mut num: u32) -> impl Iterator<Item = u32>
{
    let mut len = num.ilog10();

    gen move {
        while len > 0
        {
            // |->
            // 12345
            num %= 10_u32.pow(len);
            len -= 1;

            yield num;
        }
    }
}

// Truncate a number from right.
// 1234 -> 123 -> 12 -> 1
fn right_truncate(mut num: u32) -> impl Iterator<Item = u32>
{
    gen move {
        while num > 10
        {
            //   <-|
            // 12345
            num /= 10;
            yield num;
        }
    }
}

// Check if a number is a truncatable prime.
// A truncatable prime is a prime number that remains prime when repeatedly
// removing the leftmost and rightmost digits.
fn is_truncatable_prime(num: u32) -> bool
{
    if !is_prime(num.into())
    {
        return false;
    }

    left_truncate(num).all(|num| is_prime(num.into())) &&
        right_truncate(num).all(|num| is_prime(num.into()))
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_left_truncate()
    {
        assert_eq!(
            left_truncate(123456).collect::<Vec<_>>(),
            vec![23456, 3456, 456, 56, 6]
        );
    }

    #[test]
    fn test_right_truncate()
    {
        assert_eq!(
            right_truncate(123456).collect::<Vec<_>>(),
            vec![12345, 1234, 123, 12, 1]
        );
    }

    #[test]
    fn test_is_truncatable_prime()
    {
        assert!(is_truncatable_prime(3797));
        assert!(is_truncatable_prime(23));
        assert!(!is_truncatable_prime(379));
    }
}
