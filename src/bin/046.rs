use project_euler::project_euler_solution;
use project_euler::utils::primes::is_prime;

project_euler_solution!(046);

/// Decompose a number into sum of a prime and twice a square.
///
/// Such as:
///
/// 9 = 7 + 2*1^2
///
/// 15 = 13 + 2*1^2
///
/// # Arguments
///
/// * `num` - The number to decompose.
///
/// # Returns
///
/// A tuple containing the prime and twice a square, if it exists.
///
/// If no such prime and square exist, then `None` is returned.
fn decompose(num: i32) -> Option<(i32, i32)>
{
    /*
     * Let p a prime and k an integer.
     * We shall assume:
     *
     * n = p + 2k^2
     *
     * we can get:
     *
     * p = n - 2k^2
     *
     * Primes cannot be negative, so:
     *
     * n - 2k^2 >= 0
     *
     * n >= 2k^2
     *
     * n/2 >= k^2
     *
     * sqrt(n/2) >= k
     *
     * We need to check for `k`s up to sqrt(n/2),
     * to see if n - 2k^2 is prime.
     *
     * If we can't find a prime p that satisfies the equation, then n cannot be
     * decomposed and Goldbach's conjecture is disproven.
     */
    let max_square = (num / 2).isqrt();

    for square in 0..=max_square
    {
        let remaining = num - 2 * square * square;

        if is_prime(u64::try_from(remaining).expect("Number too large for u64"))
        {
            return Some((remaining, square));
        }
    }

    None
}

/// Goldbach's Other Conjecture
/// It was proposed by Christian Goldbach that every odd composite number can be
/// written as the sum of a prime and twice a square.
///
/// 9 = 7 + 2*1^2
///
/// 15 = 7 + 2*2^2
///
/// 21 = 3 + 2*3^2
///
/// 25 = 7 + 2*3^2
///
/// 27 = 19 + 2*2^2
///
/// 33 = 31 + 2*1^2
///
/// It turns out that the conjecture was false.
///
/// What is the smallest odd composite that cannot be written as the sum of a
/// prime and twice a square?
fn project_euler_046() -> i32
{
    (1..i32::MAX)
        .step_by(2)
        .skip(1) // Skip 1, it is not composite.
        .find(|&n| decompose(n).is_none())
        .unwrap()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_decompose()
    {
        assert_eq!(decompose(9), Some((7, 1)));
        assert_eq!(decompose(15), Some((13, 1)));
        assert_eq!(decompose(21), Some((19, 1))); // Can also be 3, 3
        assert_eq!(decompose(25), Some((23, 1))); // Can also be 7, 3
        assert_eq!(decompose(27), Some((19, 2)));
        assert_eq!(decompose(33), Some((31, 1)));
    }
}
