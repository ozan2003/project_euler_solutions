use project_euler::project_euler_solution;

project_euler_solution!(001);

/// # Multiples of 3 and 5
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we
/// get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.
fn project_euler_001() -> i32
{
    (1..1000)
        .filter(|&num| num % 3 == 0 || num % 5 == 0)
        .sum()
}
