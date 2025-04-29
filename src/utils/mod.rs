use num_traits::float::Float;
use num_traits::int::PrimInt;
use std::mem::swap;

pub mod factors;
pub mod primes;

/// A macro that generates a main function for Project Euler solutions.
///
/// This macro creates a standardized main function that:
/// - Records the start time
/// - Calls the solution function for a specific problem
/// - Measures and prints the elapsed time
/// - Prints the result
///
/// # Arguments
///
/// * `$number` - The problem number. Used to construct the function name in the
///   format `project_euler_N` where N is the problem number.
#[macro_export]
macro_rules! project_euler_solution {
    ($number:expr) => {
        fn main()
        {
            let start = std::time::Instant::now();
            let result = paste::paste! { [<project_euler_ $number>]() };
            let elapsed = start.elapsed();
            println!("answer: {}", result);
            println!("took {:.2?}", elapsed);
        }
    };
}

/// # GCD
/// Calculate the greatest common divisor of two numbers.
///
/// # Examples
/// ```
/// assert_eq!(gcd(1071, 462), 21);
/// assert_eq!(gcd(2, 3), 1);
/// ```
#[must_use]
pub fn gcd<T: PrimInt + std::ops::RemAssign>(mut a: T, mut b: T) -> T
{
    while b != T::zero()
    {
        a %= b;
        swap(&mut a, &mut b);
    }

    a
}

/// # LCM
/// Calculate the least common multiple of two numbers.
///
/// # Examples
/// ```
/// assert_eq!(lcm(21, 6), 42);
/// assert_eq!(lcm(2, 3), 6);
/// ```
#[must_use]
pub fn lcm<T: PrimInt + std::ops::RemAssign>(a: T, b: T) -> T
{
    a / gcd(a, b) * b // Prevent overflow.
}

/// # Number Length
/// Calculate the number of digits in a number.[1]
///
/// [1]: https://mathworld.wolfram.com/NumberLength.html
///
/// # Examples
/// ```
/// assert_eq!(number_length(123), 3);
/// assert_eq!(number_length(1000), 4);
/// ```
#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn number_length<T: PrimInt>(num: T) -> usize
where
    f64: std::convert::From<T>,
{
    if num.is_zero()
    {
        return 1;
    }

    ((f64::from(num)).log10().floor() as usize) + 1
}

/// # Number Length for u64
/// 
/// Overload of `number_length` for u64.
/// 
/// # Examples
/// ```
/// assert_eq!(number_length_u64(123), 3);
/// assert_eq!(number_length_u64(1000), 4);
/// ```
#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn number_length_u64(num: u64) -> usize
{
    // This one bypass the trait `From<u64>` is not being implemented for `f64`.
    if num == 0
    {
        return 1;
    }

    (num as f64).log10().floor() as usize + 1
}

/// # Number Length with Base
/// Calculate the number of digits in a number in base B.[1]
///
/// [1]: https://mathworld.wolfram.com/NumberLength.html
///
/// # Examples
/// ```
/// assert_eq!(number_length_base_b(123, 10), 3);
/// assert_eq!(number_length_base_b(585, 2), 10);
/// ```
///
/// # Errors
/// Returns an error if the base is greater than 16 or less than 2.
///
/// # Panics
/// Panics if the conversion from integers 2..=16 to type T fails.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn number_length_with_base<T: PrimInt>(num: T, base: T) -> Result<usize, &'static str>
where
    f64: std::convert::From<T>,
{
    if base > T::from(16).unwrap() || base < T::from(2).unwrap()
    {
        return Err("Base must be between 2 and 16.");
    }

    Ok(((f64::from(num)).log(f64::from(base)).floor()) as usize + 1)
}

/// Check if a decimal number is an integer.
///
/// # Examples
/// ```
/// assert!(is_integer(-1.0));
/// assert!(is_integer(2.0));
/// assert!(!is_integer(1.1));
/// assert!(!is_integer(2.0000000000001));
/// ```
#[must_use]
pub fn is_integer<T: Float>(n: T) -> bool
{
    //n == n.trunc()
    (n - n.trunc()).abs() < Float::epsilon()
}

/// # Min
///
/// Find the minimum of a list of numbers.
///
/// # Examples
///
/// ```
/// assert_eq!(min!(1, 2, 3), 1);
/// assert_eq!(min!(-7, 6, 8, -4, 0), -4);
/// ```
#[macro_export]
macro_rules! min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (::std::cmp::min($x, min!($($y),+)));
}

/// # Max
///
/// Find the maximum of a list of numbers.
///
/// # Examples
///
/// ```
/// assert_eq!(max!(1, 2, 3), 3);
/// assert_eq!(max!(-7, 6, 8, -4, 0), 8);
/// ```
#[macro_export]
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (::std::cmp::max($x, max!($($y),+)));
}

#[cfg(test)]
mod tests
{
    use super::*;
    use core::f64;

    #[test]
    fn test_gcd()
    {
        assert_eq!(gcd(1071, 462), 21);
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(7, 13), 1);
        assert_eq!(gcd(28851538, 1183019), 17657);
    }

    #[test]
    fn test_lcm()
    {
        assert_eq!(lcm(21, 6), 42);
        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(15, 20), 60);
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(7, 13), 91);
        assert_eq!(lcm(48, 180), 720);
    }

    #[test]
    fn test_number_length()
    {
        assert_eq!(number_length(123), 3);
        assert_eq!(number_length(1000), 4);
        assert_eq!(number_length(1), 1);
        assert_eq!(number_length(0), 1);
        assert_eq!(number_length(999999), 6);
        assert_eq!(number_length(1234567), 7);
    }

    #[test]
    fn test_number_length_u64()
    {
        assert_eq!(number_length_u64(123), 3);
        assert_eq!(number_length_u64(1000), 4);
        assert_eq!(number_length_u64(1), 1);
        assert_eq!(number_length_u64(0), 1);
        assert_eq!(number_length_u64(999999), 6);
        assert_eq!(number_length_u64(1234567), 7);
    }

    #[test]
    fn test_number_length_with_base()
    {
        assert_eq!(number_length_with_base(123, 10).unwrap(), 3);
        assert_eq!(number_length_with_base(585, 2).unwrap(), 10);
        assert_eq!(number_length_with_base(15, 16).unwrap(), 1);
        assert_eq!(number_length_with_base(255, 16).unwrap(), 2);
        assert!(number_length_with_base(100, 17).is_err());
        assert!(number_length_with_base(100, 1).is_err());
    }

    #[test]
    fn test_is_integer()
    {
        assert!(is_integer(-1.0));
        assert!(is_integer(2.0));
        assert!(!is_integer(1.1));
        assert!(!is_integer(2.0000000000001));
        assert!(is_integer(0.0));
        assert!(is_integer(-5.0));
        assert!(!is_integer(f64::consts::PI));
        assert!(!is_integer(-2.5));
    }
}
