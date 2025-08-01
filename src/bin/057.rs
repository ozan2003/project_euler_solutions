use cached::proc_macro::cached;
use fraction::{BigFraction, One};
use project_euler::project_euler_solution;

project_euler_solution!(057);

/// Square Root Convergents
///
/// The square root of two can be expressed as an infinite continued fraction:
///
/// sqrt(2) = 1 + 1/(2 + 1/(2 + ...))
///
/// By expanding this for the first four iterations, we get:
///
/// 1 + 1/12 = 3/2
///
/// 1 + 1/(2 + 1/2) = 7/5
///
/// 1 + 1/(2 + 1/(2 + 1/2)) = 17/12
///
/// 1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) = 41/29
///
/// The next three expansions are `99/70`, `239/169`, and `577/408`, but the
/// eighth expansion, `1393/985`, is the first example where the number of
/// digits in the numerator exceeds the number of digits in the denominator.
///
/// In the first one-thousand expansions, how many fractions contain a numerator
/// with more digits than the denominator?
fn project_euler_057() -> usize
{
    (1..1000)
        .map(square2)
        .filter(|frac| {
            let (numer_len, denom_len) = number_lengths(frac);
            numer_len > denom_len
        })
        .count()
}

/// Calculate the square root of 2 using continued fractions.
///
/// Bigger the `n`, the more accurate the result.
///
/// Args:
/// * `n`: The number of iterations.
///
/// Returns:
/// * A `Fraction` representing the square root of 2.
#[cached]
fn square2(n: u32) -> BigFraction
{
    /*
    sqrt(2) = 1 + (sqrt(2) - 1)
            = 1 + 1/1 + sqrt(2)
            = 1 + 1/(1 + (1 + 1/(1 + sqrt(2))))
            ...
            = 1 + 1/(2 + 1/(2 + ...))
     */
    match n
    {
        0 => BigFraction::one(),
        _ =>
        {
            BigFraction::one() +
                (BigFraction::one() / (BigFraction::one() + square2(n - 1)))
        },
    }
}

/// Get the number of digits in the numerator and denominator of a fraction.
///
/// This function returns the number of digits in the numerator and denominator
/// of a fraction.
///
/// # Arguments
/// * `frac` - A `BigFraction` reference.
///
/// # Returns
/// A tuple containing the number of digits in the numerator and denominator.
fn number_lengths(frac: &BigFraction) -> (usize, usize)
{
    let numer = frac.numer().unwrap();
    let denom = frac.denom().unwrap();

    let numer_len = numer.to_string().len();
    let denom_len = denom.to_string().len();

    (numer_len, denom_len)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_square2()
    {
        assert_eq!(square2(0), BigFraction::one());
        assert_eq!(square2(1), BigFraction::new(3u32, 2u32));
        assert_eq!(square2(2), BigFraction::new(7u32, 5u32));
        assert_eq!(square2(3), BigFraction::new(17u32, 12u32));
        assert_eq!(square2(4), BigFraction::new(41u32, 29u32));
        assert_eq!(square2(5), BigFraction::new(99u32, 70u32));
        assert_eq!(square2(6), BigFraction::new(239u32, 169u32));
        assert_eq!(square2(7), BigFraction::new(577u32, 408u32));
        assert_eq!(square2(8), BigFraction::new(1393u32, 985u32));
    }

    #[test]
    fn test_number_lengths()
    {
        let frac = BigFraction::new(3u32, 2u32);
        assert_eq!(number_lengths(&frac), (1, 1));

        let frac = BigFraction::new(77899u32, 5_124_785_u32);
        assert_eq!(number_lengths(&frac), (5, 7));
    }
}
