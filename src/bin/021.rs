use project_euler::project_euler_solution;

project_euler_solution!(021);

/// # Amicable Numbers
///
/// Let `d(n)` be defined as the sum of proper divisors of `n` (numbers less
/// than `n` which divide evenly into `n`).
///
/// If `d(a) = b` and `d(b) = a`, where
/// `a != b`, then `a` and `b` are an amicable pair and each of `a` and `b` are
/// called amicable numbers.
///
/// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44,
/// 55 and 110; therefore `d(220) = 284`. The proper divisors of 284 are 1, 2,
/// 4, 71 and 142; so `d(284) = 220`.
///
/// Evaluate the sum of all the amicable numbers under 10000.
fn project_euler_021() -> i32
{
    (1..10000).filter(|&n| is_amicable(n)).sum()
}

// Sum of proper divisors of `n`.
fn d(n: i32) -> i32
{
    let mut sum = 1;
    for i in 2..=n.isqrt()
    {
        if n % i == 0
        {
            sum += i;
            let j = n / i;
            // If `n` is a perfect square, we don't want to add its square root twice.
            if j != i
            {
                sum += j;
            }
        }
    }

    sum
}

// Returns true if `a` and `b` are an amicable pair.
// https://mathworld.wolfram.com/AmicablePair.html
fn is_amicable(a: i32) -> bool
{
    let b = d(a);
    a != b && a == d(b)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_d()
    {
        assert_eq!(d(220), 284);
        assert_eq!(d(284), 220);
    }

    #[test]
    fn test_is_amicable()
    {
        assert!(is_amicable(220));
        assert!(is_amicable(284));
        assert!(!is_amicable(285));
    }
}
