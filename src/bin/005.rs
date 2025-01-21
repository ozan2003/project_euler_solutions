use project_euler::{project_euler_solution, utils::lcm};

project_euler_solution!(005);

/// # Smallest multiple
///
/// 2520 is the smallest number that can be divided by each of the numbers from
/// 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly divisible by all of the
/// numbers from 1 to 20?
fn project_euler_005() -> i32
{
    // Calculate lcm(1..=20).
    // lcm(1, lcm(2, lcm(3, lcm(4, lcm(5, lcm(6, lcm(7, ... lcm(19, 20))))))))
    (1..=20).fold(1, lcm)
}
