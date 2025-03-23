use permutohedron::Heap;
use project_euler::project_euler_solution;

project_euler_solution!(043);

static PRIMES: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];

/// Converts a slice of digits to a number.
fn slice_to_num(slice: &[u64]) -> u64
{
    slice.iter().fold(0, |acc, &x| acc * 10 + x)
}

/// Check if all the 3 digit slices of the given slice are divisible by the
/// corresponding prime.
fn slice_divisibility(slice: &[u64]) -> bool
{
    slice
        .windows(3)
        .skip(1)
        .zip(PRIMES.iter())
        .all(|(slice, &prime)| slice_to_num(slice) % prime == 0)
}

/// # Sub-string Divisiblity
/// The number, 1406357289, is a 0 to 9 pandigital number because it is made up
/// of each of the digits 0 to 9 in some order, but it also has a rather
/// interesting sub-string divisibility property.
///
/// Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we
/// note the following:
///
/// `d_2d_3d_4=406` is divisible by 2
///
/// `d_3d_4d_5=063` is divisible by 3
///
/// `d_4d_5d_6=635` is divisible by 5
///
/// `d_5d_6d_7=357` is divisible by 7
///
/// `d_6d_7d_8=572` is divisible by 11
///
/// `d_7d_8d_9=728` is divisible by 13
///
/// `d_8d_9d_10=289` is divisible by 17
///
///
/// Find the sum of all 0 to 9 pandigital numbers with this property.
fn project_euler_043() -> u64
{
    let mut digits: Vec<u64> = (0..=9).collect();

    Heap::new(&mut digits)
        .filter(|perm| slice_divisibility(perm))
        .map(|perm| slice_to_num(&perm))
        .sum()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_slice_to_num()
    {
        assert_eq!(slice_to_num(&[1, 2, 3]), 123);
        assert_eq!(slice_to_num(&[0, 1, 2, 3]), 123);
    }

    #[test]
    fn test_slice_divisibility()
    {
        assert!(slice_divisibility(&[1, 4, 0, 6, 3, 5, 7, 2, 8, 9]));
        assert!(!slice_divisibility(&[1, 4, 0, 6, 3, 5, 7, 2, 8, 8]));
    }
}
