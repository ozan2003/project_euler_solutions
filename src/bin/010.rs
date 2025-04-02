use project_euler::project_euler_solution;
use project_euler::utils::primes::Primes;

project_euler_solution!(010);

/// # Summation of primes
/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
/// Find the sum of all the primes below two million.
fn project_euler_010() -> usize
{
    const UPPER_LIMIT: usize = 2_000_000;

    Primes::new(UPPER_LIMIT).sum()
}
