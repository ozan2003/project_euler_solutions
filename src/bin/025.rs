use dashu::{Integer, integer::UBig};
use project_euler::project_euler_solution;

project_euler_solution!(025);

// Generate Fibonacci numbers.
fn fibonaccis() -> impl Iterator<Item = Integer>
{
    let mut a = Integer::ZERO;
    let mut b = Integer::ONE;

    std::iter::from_fn(move || {
        (a, b) = (b.clone(), &a + &b);
        Some(a.clone())
    })
}

/// # 1000-digit Fibonacci number
///
/// The Fibonacci sequence is defined by the recurrence relation:
///
/// F(n) = F(n−1) + F(n−2), where F_1 = 1 and F_2 = 1.
///
/// Hence the first 12 terms will be:
///
/// F_1 = 1
/// F_2 = 1
/// F_3 = 2
/// F_4 = 3
/// F_5 = 5
/// F_6 = 8
/// F_7 = 13
/// F_8 = 21
/// F_9 = 34
/// F_10 = 55
/// F_11 = 89
/// F_12 = 144
///
/// The 12th term, F_12, is the first term to contain three digits.
///
/// What is the index of the first term in the Fibonacci sequence to contain
/// 1000 digits?
fn project_euler_025() -> usize
{
    (1..)
        .zip(fibonaccis())
        //.find(|(_i, n)| n.to_string().len() >= 1000)
        .find(|(_i, n)| n.ilog(&UBig::from(10_u32)) >= 999) // This is faster.
        .map(|(i, _n)| i)
        .unwrap()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_twelve_fibonacci_numbers()
    {
        let expected = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
        let fib_nums: Vec<i32> = fibonaccis()
            .take(12)
            .map(|x| x.try_into().unwrap())
            .collect();
        assert_eq!(fib_nums, expected);
    }

    #[test]
    fn test_first_number_with_three_digits()
    {
        let twelfth = fibonaccis().nth(11).unwrap();
        assert_eq!(twelfth.to_string().len(), 3);
    }
}
