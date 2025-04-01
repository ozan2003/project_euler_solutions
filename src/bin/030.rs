use project_euler::project_euler_solution;

project_euler_solution!(030);

/// # Digit Fifth Power
/// Surprisingly there are only three numbers that can be written as the sum of
/// fourth powers of their digits:
///
/// 1634 = 1^4 + 6^4 + 3^4 + 4^4
///
/// 8208 = 8^4 + 2^4 + 0^4 + 8^4
///
/// 9474 = 9^4 + 4^4 + 7^4 + 4^4
///
/// As 1 = 1^4 is not a sum it is not included.
///
/// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
///
/// Find the sum of all the numbers that can be written as the sum of fifth
/// powers of their digits.
fn project_euler_030() -> i32
{
    const UPPER_LIMIT: i32 = 1_000_000; // 6 digits seems appropriate.

    (2..UPPER_LIMIT)
        .filter(|&n| is_sum_of_fifth_power(n))
        .sum()
}

/// Returns true if the number is the sum of the fifth power of its digits.
#[allow(clippy::used_underscore_binding)]
fn is_sum_of_fifth_power(num: i32) -> bool
{
    let mut sum = 0;
    let mut _num = num;

    while _num > 0
    {
        sum += (_num % 10).pow(5);
        _num /= 10;
    }

    sum == num
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_is_sum_of_fifth_power()
    {
        assert!(!is_sum_of_fifth_power(1634));
        assert!(is_sum_of_fifth_power(4150));
        assert!(is_sum_of_fifth_power(54748));
    }
}
