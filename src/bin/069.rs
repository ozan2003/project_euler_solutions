use project_euler::project_euler_solution;
use project_euler::utils::primes::Primes;

project_euler_solution!(069);

/// Totient Maximum
/// Euler's Totient function, phi(n) [sometimes called the phi function], is
/// used to determine the number of numbers less than n which are relatively
/// prime to n. For example, as 1, 2, 4, 5, 7, and 8, are all less than nine and
/// relatively prime to nine, phi(9) = 6.
///
/// / ... /
///
/// It can be seen that n = 6 produces a maximum n/phi(n) for n <= 10.
///
/// Find the value of n <= 1,000,000 for which n/phi(n) is a maximum.
fn project_euler_069() -> u32
{
    const UPPER_LIMIT: u32 = 1_000_000;

    /*
     * phi(n) = n * (p1-1)/p1 * (p2-1)/p2 * ... * (pk-1)/pk
     *        = n * (p1-1)*(p2-1)*...*(pk-1) / (p1*p2*...*pk)
     *
     * n/phi(n) = (p1*p2*...*pk) / ((p1-1)*(p2-1)*...*(pk-1))
     *          = ∏(pi / (pi - 1))
     *
     * To maximize n/phi(n), we need to minimize (p1-1)*(p2-1)*...*(pk-1)
     * and maximize (p1*p2*...*pk).
     *
     * The factors maximised when pi - 1 is minimised, when pi is smallest.
     * And each prime only has one factor, so we need to find the product of
     * the smallest primes that is less than 1_000_000.
     */
    Primes::new(UPPER_LIMIT as usize)
        .iter()
        .map(|p| u32::try_from(p).expect("Prime too large for u32"))
        .scan(1, |acc, p| {
            *acc *= p;
            Some(*acc)
        })
        .take_while(|&prod| prod <= UPPER_LIMIT)
        .last()
        .unwrap()
}
