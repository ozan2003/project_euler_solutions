use std::collections::HashSet;

use project_euler::project_euler_solution;

project_euler_solution!(074);

/// # Digit Factorial Chains
///
/// The number 145 is well known for the property that the sum of the factorial
/// of its digits is equal to 145:
///
/// 1! + 4! + 5! = 1 + 24 + 120 = 145
///
/// Perhaps less well known is 169, in that it produces the longest chain of
/// numbers that link back to 169; it turns out that there are only three such
/// loops that exist:
///
/// 169 -> 363601 -> 1454 -> 169
/// 871 -> 45361 -> 871
/// 872 -> 45362 -> 872
///
/// It is not difficult to prove that EVERY starting number will eventually get
/// stuck in a loop. For example,
///
/// 69 -> 363600 -> 1454 -> 169 -> 363601 (-> 1454)
///
/// Starting with 69 produces a chain of five non-repeating terms, but the
/// longest non-repeating chain with a starting number below one million is
/// sixty terms.
///
/// How many chains, with a starting number below one million, contain exactly
/// sixty non-repeating terms?
fn project_euler_074() -> usize
{
    const UPPER_LIMIT: i32 = 1_000_000;

    (0..UPPER_LIMIT)
        .filter(|&n| chain_length(n) == 60)
        .count()
}

const FACTORIALS: [i32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362_880];

#[allow(clippy::cast_sign_loss)]
// Sum of digits' factorials of a number.
const fn factorial_digit_sum(mut num: i32) -> i32
{
    let mut sum = 0;

    while num > 0
    {
        sum += FACTORIALS[(num % 10) as usize];
        num /= 10;
    }
    sum
}

// Length of the chain of a number.
fn chain_length(mut num: i32) -> usize
{
    let mut chain_len = 1; // Starting with the num itself.
    let mut chain = HashSet::with_capacity(60); // We're looking for at least 60.

    loop
    {
        num = factorial_digit_sum(num);
        // If the chain contains the number, we've found a loop.
        if chain.contains(&num)
        {
            break;
        }

        chain.insert(num);
        chain_len += 1;
    }

    chain_len
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_factorial_digit_sum()
    {
        assert_eq!(factorial_digit_sum(145), 145);
        assert_eq!(factorial_digit_sum(169), 363_601);
    }

    #[test]
    fn test_chain_length()
    {
        assert_eq!(chain_length(69), 5);
        assert_eq!(chain_length(78), 4);
    }
}
