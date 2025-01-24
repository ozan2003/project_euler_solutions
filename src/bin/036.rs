use project_euler::{project_euler_solution, utils::number_length_with_base};

project_euler_solution!(036);

const DIGITS: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
];

// Convert a number to a base N number.
fn to_base<const N: i32>(mut num: i32) -> Vec<u8>
{
    let mut converted = Vec::with_capacity(number_length_with_base(num, N as u32).unwrap());

    while num > 0
    {
        converted.push(DIGITS[(num % N) as usize]);
        num /= N;
    }

    converted.reverse();

    converted
}

// Check if a slice of digits is a palindrome.
fn is_palindrome(digits: &[u8]) -> bool
{
    let half_len = digits.len() / 2;

    digits
        .iter()
        .take(half_len)
        .eq(digits.iter().rev().take(half_len))
}

/// # Double-base Palindromes
/// The decimal number, 585 = 0b1001001001, is palindromic in both bases.
///
/// Find the sum of all numbers, less than one million, which are palindromic in
/// base 10 and base 2.
///
/// (Please note that the palindromic number, in either base, may not include
/// leading zeros.)
fn project_euler_036() -> i32
{
    const UPPER_LIMIT: i32 = 1_000_000;

    (1..UPPER_LIMIT)
        // Skip even numbers as they will have trailing zeros in binary. (Binaries won't include
        // leading zeros.)
        .step_by(2)
        .filter(|&num| is_palindrome(&to_base::<10>(num)) && is_palindrome(&to_base::<2>(num)))
        .sum()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_to_base()
    {
        assert_eq!(to_base::<2>(585), vec![
            b'1', b'0', b'0', b'1', b'0', b'0', b'1', b'0', b'0', b'1'
        ]);
        assert_eq!(to_base::<10>(585), vec![b'5', b'8', b'5']);
        assert_eq!(to_base::<16>(585), vec![b'2', b'4', b'9']);
    }

    #[test]
    fn test_is_palindrome()
    {
        assert!(is_palindrome(b"585"));
        assert!(is_palindrome(b"1001001001"));
        assert!(!is_palindrome(b"123"));
        assert!(!is_palindrome(b"1001001000"));
    }
}
