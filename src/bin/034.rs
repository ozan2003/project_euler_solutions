use project_euler::project_euler_solution;

project_euler_solution!(034);

/// # Digit Factorials
/// The number 145 is well known for the property that the sum of the factorial
/// of its digits is equal to 145:
///
/// 1! + 4! + 5! = 1 + 24 + 120 = 145
///
/// Find the sum of all numbers which are equal to the sum of the factorial of
/// their digits.
///
/// Note: As 1! = 1 and 2! = 2 are not sums they are not included.
fn project_euler_034() -> u32
{
    const UPPER_LIMIT: u32 = 60000; // 60000 should be enough.

    // Don't include 1 and 2.
    (3..UPPER_LIMIT)
        .filter(|&x| is_curious_number(x))
        .sum()
}

// Factorials of 0 to 9 pre-calculated.
const FACTORIALS: [u32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362_880];

// Finds the sum of all digits' factorials of a number.
#[allow(clippy::cast_sign_loss)]
const fn digit_factorial_sum(mut num: u32) -> u32
{
    let mut sum = 0;

    while num > 0
    {
        sum += FACTORIALS[(num % 10) as usize];
        num /= 10;
    }

    sum
}

// Checks if a number is a curious number.
// A curious number is a number that is equal to the sum of the factorials of
// its digits.
const fn is_curious_number(num: u32) -> bool
{
    num == digit_factorial_sum(num)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_digit_factorial_sum()
    {
        assert_eq!(digit_factorial_sum(149), 362_905);
        assert_eq!(digit_factorial_sum(47585), 45624);
    }

    #[test]
    fn test_is_curious_number()
    {
        assert!(is_curious_number(145));
        assert!(is_curious_number(40585));
    }
}
