use project_euler::project_euler_solution;

project_euler_solution!(006);

/// Sum Square Difference
///
/// The sum of the squares of the first ten natural numbers is,
///
/// 1^2 + 2^2 + ... + 10^2 = 385
///
/// The square of the sum of the first ten natural numbers is,
///
/// (1 + 2 + ... + 10)^2 = 55^2 = 3025
///
/// Hence the difference between the sum of the squares of the first ten natural
/// numbers and the square of the sum is 3025 - 385 = 2640.
///
/// Find the difference between the sum of the squares of the first one hundred
/// natural numbers and the square of the sum.
fn project_euler_006() -> u32
{
    const UPPER_LIMIT: i32 = 100;

    let square_of_sum = (1..=UPPER_LIMIT).sum::<i32>().pow(2);
    let sum_of_square = (1..=UPPER_LIMIT).map(|num| num * num).sum();

    square_of_sum.abs_diff(sum_of_square)
}
