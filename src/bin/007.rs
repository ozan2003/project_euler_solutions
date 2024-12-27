use num_prime::nt_funcs::is_prime;
use project_euler::project_euler_solution;

project_euler_solution!(007);

/// # 10001st prime
/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
/// that the 6th prime is 13.
///
/// What is the 10 001st prime number?
fn project_euler_007() -> u64
{
    // One-liner. This one's much faster.
    // num_prime::nt_funcs::nth_prime(10_001)

    const UPPER_LIMIT: u64 = u64::MAX;

    (1..UPPER_LIMIT)
        .filter(|&num| is_prime(&num, None).probably())
        .nth(10_000)
        .unwrap()
}
