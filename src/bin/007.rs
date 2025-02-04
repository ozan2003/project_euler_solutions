use project_euler::project_euler_solution;
use project_euler::utils::primes::is_prime;

project_euler_solution!(007);

/// # 10001st prime
/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
/// that the 6th prime is 13.
///
/// What is the 10 001st prime number?
fn project_euler_007() -> u64
{
    const UPPER_LIMIT: u64 = u64::MAX;

    (1..UPPER_LIMIT)
        .filter(|&num| is_prime(num))
        .nth(10_000)
        .unwrap()
}
