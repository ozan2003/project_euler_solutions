use dashu::Integer;
use project_euler::project_euler_solution;

project_euler_solution!(016);

/// # Powerful Digit Sum
/// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
///
/// What is the sum of the digits of the number 2^1000?
fn project_euler_016() -> u32
{
    let digits = Integer::from(2).pow(1000).to_string();

    digits
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .sum()
}
