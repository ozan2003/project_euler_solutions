use dashu::Integer;
use project_euler::project_euler_solution;

project_euler_solution!(015);

/// # Lattice Paths
///
/// Starting in the top left corner of a 2×2 grid, and only being able to move
/// to the right and down, there are exactly 6 routes to the bottom right
/// corner.
///
/// How many such routes are there through a 20×20 grid?
fn project_euler_015() -> Integer
{
    // Repeated permutation.
    // 10S and 10Es
    factorial(40) / (factorial(20) * factorial(20))
}

/// # Factorial
///
/// Calculate the factorial of a number.
fn factorial(n: i32) -> Integer
{
    (1..=n).map(Integer::from).product()
}

#[cfg(test)]
mod tests
{
    use dashu::ibig;

    use super::*;

    #[test]
    fn test_factorial()
    {
        assert_eq!(factorial(0), Integer::ONE);
        assert_eq!(factorial(1), Integer::ONE);
        assert_eq!(factorial(23), ibig!(25852016738884976640000));
    }
}
