use num_prime::nt_funcs::primes;
use project_euler::project_euler_solution;

project_euler_solution!(010);

/// # Summation of primes
/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
/// 
/// Find the sum of all the primes below two million.
fn project_euler_010() -> u64
{
    const UPPER_LIMIT: u64 = 2_000_000;

    primes(UPPER_LIMIT).into_iter().sum()
}
