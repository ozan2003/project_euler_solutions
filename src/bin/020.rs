use dashu::Integer;
use project_euler::project_euler_solution;

project_euler_solution!(020);

/// # Factorial Digit Sum
/// n! means n * (n − 1) * ... * 3 * 2 * 1
/// For example, 10! = 10 * 9 * ... * 3 * 2 * 1 = 3628800,
/// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
/// 
/// Find the sum of the digits in the number 100!
fn project_euler_020() -> u32
{
    let factorial: Integer = (1..=100).map(Integer::from).product();

    factorial
        .to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .sum()
}
