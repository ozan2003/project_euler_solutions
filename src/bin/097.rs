use dashu::Integer;
use project_euler::project_euler_solution;

project_euler_solution!(097);

/// Large Non-Mersenne Prime
///
/// The first known prime found to exceed one million digits was discovered in
/// 1999, and is a Mersenne prime of the form 2^6972593−1; it contains exactly
/// 2,098,960 digits. Subsequently other Mersenne primes, of the form 2^p−1,
/// have been found which contain more digits.
///
/// However, in 2004 there was found a massive non-Mersenne prime which contains
/// 2,357,207 digits: 28433*2^7830457+1.
///
/// Find the last ten digits of this prime number.
fn project_euler_097() -> String
{
    // 28433 * 2**7830457 + 1
    let mut bignum = Integer::from(28433);
    bignum *= Integer::from(2).pow(7_830_457);
    bignum += 1;

    let buffer = bignum.to_string();
    buffer[buffer.len() - 10..].to_string()
}
