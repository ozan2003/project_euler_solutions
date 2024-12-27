use num_prime::nt_funcs::factorize;
use project_euler::project_euler_solution;

project_euler_solution!(003);

/// # Largest prime factor
/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143?
fn project_euler_003() -> Option<u64>
{
    const NUM: u64 = 600_851_475_143;

    let factors = factorize(NUM);

    Some(*factors.last_key_value().unwrap().0)
}
