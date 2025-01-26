use num_prime::nt_funcs::is_prime;
use permutohedron::Heap;
use project_euler::project_euler_solution;

project_euler_solution!(041);

/// # Pandigital Prime
/// We shall say that an n-digit number is pandigital if it makes use of all the
/// digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is
/// also prime.
///
/// What is the largest n-digit pandigital prime that exists?
fn project_euler_041() -> u32
{
    // Look for 7 digit numbers since sum of 9 digits is 45 which is divisible
    // by 3 and 8 digits is 36 which is also divisible by 3.
    // So, the largest pandigital prime number is a 7 digit number.
    let mut digits: Vec<u32> = (1..=7).collect();

    Heap::new(&mut digits)
        .map(|perm| {
            // Build up the numbers.
            perm.iter().fold(0, |acc, &d| acc * 10 + d)
        })
        .filter(|&n| is_prime(&n, None).probably())
        .max()
        .unwrap()
}
