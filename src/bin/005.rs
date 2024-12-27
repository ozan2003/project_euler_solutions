use project_euler::project_euler_solution;

project_euler_solution!(005);

/// # GCD
/// Calculate the greatest common divisor of two numbers.
/// 
/// # Examples
/// ```
/// assert_eq!(gcd(1071, 462), 21);
/// assert_eq!(gcd(2, 3), 1);
/// ```
/// 
fn gcd(mut a: i32, mut b: i32) -> i32
{
    while b != 0
    {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

/// # LCM
/// Calculate the least common multiple of two numbers.
/// 
/// # Examples
/// ```
/// assert_eq!(lcm(21, 6), 42);
/// assert_eq!(lcm(2, 3), 6);
/// ```
fn lcm(a: i32, b: i32) -> i32
{
    a / gcd(a, b) * b // Prevent overflow.
}

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
