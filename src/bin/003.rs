use project_euler::project_euler_solution;
use project_euler::utils::factors::pollards_rho;

project_euler_solution!(003);

/// # Largest prime factor
/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143?
fn project_euler_003() -> i64
{
    const NUM: i64 = 600_851_475_143;

    let factors = pollards_rho(NUM);

    *factors.last_key_value().unwrap().0
}
