use project_euler::project_euler_solution;
use project_euler::utils::factors::pollards_rho;

project_euler_solution!(047);

// Find the number of distinct prime factors of a number.
fn distinct_factor_num(num: u32) -> usize
{
    pollards_rho(num.into()).len()
}

/// # Distinct Primes Factors
/// The first two consecutive numbers to have two distinct prime factors are:
///
/// 14 = 2 * 7
///
/// 15 = 3 * 5
///
/// The first three consecutive numbers to have three distinct prime factors
/// are:
///
/// 644 = 2^2 * 7 * 23
///
/// 645 = 3 * 5 * 43
///
/// 646 = 2 * 17 * 19
///
/// Find the first four consecutive integers to have four distinct prime factors
/// each. What is the first of these numbers?
fn project_euler_047() -> u32
{
    let mut tally = 0;

    for i in 647..u32::MAX
    {
        if distinct_factor_num(i) == 4
        {
            tally += 1;
        }
        else if tally == 0
        {
            continue;
        }
        else if tally == 4
        {
            return i - 3;
        }
        else
        {
            tally = 0;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_distinct_factor_num()
    {
        assert_eq!(distinct_factor_num(14), 2);
        assert_eq!(distinct_factor_num(15), 2);
        assert_eq!(distinct_factor_num(644), 3);
        assert_eq!(distinct_factor_num(645), 3);
        assert_eq!(distinct_factor_num(646), 3);
    }
}
