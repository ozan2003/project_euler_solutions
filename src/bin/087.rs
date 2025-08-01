use std::collections::HashSet;

use project_euler::project_euler_solution;
use project_euler::utils::primes::Primes;

project_euler_solution!(087);

/// # Prime Power Triples
///
/// The smallest number expressible as the sum of a prime square, prime cube,
/// and prime fourth power is 28. In fact, there are exactly four numbers below
/// fifty that can be expressed in such a way:
///
/// 28 = 2^2 + 2^3 + 2^4
///
/// 33 = 3^2 + 2^3 + 2^4
///
/// 49 = 5^2 + 2^3 + 2^4
///
/// 47 = 2^2 + 3^3 + 2^4
///
///
/// How many numbers below fifty million can be expressed as the sum of a prime
/// square, prime cube, and prime fourth power?
fn project_euler_087() -> usize
{
    const UPPER_LIMIT: usize = 50_000_000;

    // (49_999_991 - 2**3 - 2**4).isqrt() = 7071
    // Any prime greater than 7071 will result in a sum greater than 50_000_000.
    let primes: Vec<_> = Primes::new(7071).iter().collect();

    let mut sums = HashSet::new();

    for &c in &primes
    {
        for &b in &primes
        {
            if c.pow(4) + b.pow(3) >= UPPER_LIMIT
            {
                break;
            }

            for &a in &primes
            {
                let sum = a.pow(2) + c.pow(4) + b.pow(3);
                if sum >= UPPER_LIMIT
                {
                    break;
                }

                sums.insert(sum);
            }
        }
    }

    sums.len()
}
