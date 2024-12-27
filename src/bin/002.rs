use project_euler::project_euler_solution;

project_euler_solution!(002);

fn fibonaccis() -> impl Iterator<Item = i32>
{
    let mut a = 0;
    let mut b = 1;

    std::iter::from_fn(move || {
        (a, b) = (b, a + b);
        Some(a)
    })
}

/// # Even Fibonacci numbers
/// Each new term in the Fibonacci sequence is generated by adding the previous
/// two terms. By starting with 1 and 2, the first 10 terms will be:
///
/// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
///
/// By considering the terms in the Fibonacci sequence whose values do not
/// exceed four million, find the sum of the even-valued terms.
fn project_euler_002() -> i32
{
    const UPPER_LIMIT: i32 = 4_000_000;

    fibonaccis()
        .filter(|&num| num % 2 == 0)
        .take_while(|&num| num < UPPER_LIMIT)
        .sum()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_fibonaccis()
    {
        let fibs: Vec<i32> = fibonaccis().take(10).collect();
        assert_eq!(fibs, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }
}
