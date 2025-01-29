use project_euler::{project_euler_solution, utils::is_integer};

project_euler_solution!(045);

// Triangular numbers.
// 1, 3, 6, 10, 15, ...
fn triangulars() -> impl Iterator<Item = usize>
{
    (1..).scan(0, |acc, x| {
        *acc += x;
        Some(*acc)
    })
}

// Check if a number is pentagonal.
// https://en.wikipedia.org/wiki/Pentagonal_number#Tests_for_pentagonal_numbers
#[allow(clippy::cast_precision_loss)]
fn is_pentagonal(n: usize) -> bool
{
    let m = (1.0 + ((1 + 24 * n) as f64).sqrt()) / 6.0;

    is_integer(m)
}

// Check if a number is hexagonal.
// https://planetmath.org/testforhexagonalnumbers
#[allow(clippy::cast_precision_loss)]
fn is_hexagonal(n: usize) -> bool
{
    let m = (1.0 + ((1 + 8 * n) as f64).sqrt()) / 4.0;

    is_integer(m)
}

/// # Triangular, Pentagonal, and Hexagonal
/// Triangle, pentagonal, and hexagonal numbers are generated by the following
/// formulae:
///
/// Triangle: `T_n` = n(n + 1) / 2
///
/// Pentagonal: `P_n` = n(3n - 1) / 2
///
/// Hexagonal: `H_n` = n(2n - 1)
///
/// It can be verified that `T_285` = `P_165` = `H_143` = 40755.
///
/// Find the next triangle number that is also pentagonal and hexagonal after
/// `T_285`.
fn project_euler_045() -> usize
{
    // T_285 = P_165 = H_143 = 40755
    triangulars()
        .skip(285)
        .find(|&num| is_pentagonal(num) && is_hexagonal(num))
        .unwrap()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_triangulars()
    {
        let mut t = triangulars();
        assert_eq!(t.next(), Some(1));
        assert_eq!(t.next(), Some(3));
        assert_eq!(t.next(), Some(6));
        assert_eq!(t.next(), Some(10));
        assert_eq!(t.next(), Some(15));
    }

    #[test]
    fn test_is_pentagonal()
    {
        assert!(is_pentagonal(1));
        assert!(is_pentagonal(5));
        assert!(is_pentagonal(12));
        assert!(is_pentagonal(22));
        assert!(is_pentagonal(247));
        assert!(is_pentagonal(287));
        assert!(is_pentagonal(330));
        assert!(is_pentagonal(376));
        assert!(is_pentagonal(1820));
        assert!(is_pentagonal(1926));
    }

    #[test]
    fn test_is_hexagonal()
    {
        assert!(is_hexagonal(1));
        assert!(is_hexagonal(6));
        assert!(is_hexagonal(15));
        assert!(is_hexagonal(28));
        assert!(is_hexagonal(45));
        assert!(is_hexagonal(66));
        assert!(is_hexagonal(91));
        assert!(is_hexagonal(120));
        assert!(is_hexagonal(153));
        assert!(is_hexagonal(190));
    }
}
