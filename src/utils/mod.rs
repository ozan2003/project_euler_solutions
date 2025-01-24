/// A macro that generates a main function for Project Euler solutions.
///
/// This macro creates a standardized main function that:
/// - Records the start time
/// - Calls the solution function for a specific problem
/// - Measures and prints the elapsed time
/// - Prints the result
///
/// # Arguments
///
/// * `$number` - The problem number. Used to construct the function name in the
///   format `project_euler_N` where N is the problem number.
///
/// Note: Requires the `paste` crate for identifier concatenation.
#[macro_export]
macro_rules! project_euler_solution {
    ($number:expr) => {
        fn main()
        {
            let start = std::time::Instant::now();
            let result = paste::paste! { [<project_euler_ $number>]() };
            let elapsed = start.elapsed();
            println!("answer: {}", result);
            println!("took {:.2?}", elapsed);
        }
    };
}

/// # GCD
/// Calculate the greatest common divisor of two numbers.
///
/// # Examples
/// ```
/// assert_eq!(gcd(1071, 462), 21);
/// assert_eq!(gcd(2, 3), 1);
/// ```
pub fn gcd(mut a: i32, mut b: i32) -> i32
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
pub fn lcm(a: i32, b: i32) -> i32
{
    a / gcd(a, b) * b // Prevent overflow.
}

/// # Number Length
/// Calculate the number of digits in a number.[1]
/// 
/// [1]: https://mathworld.wolfram.com/NumberLength.html
///
/// # Examples
/// ```
/// assert_eq!(number_length(123), 3);
/// assert_eq!(number_length(1000), 4);
/// ```
pub fn number_length(num: i32) -> usize
{
    ((num as f64).log10().floor()) as usize + 1
}

/// # Number Length with Base
/// Calculate the number of digits in a number in base B.[1]
/// 
/// [1]: https://mathworld.wolfram.com/NumberLength.html
///
/// # Examples
/// ```
/// assert_eq!(number_length_base_b(123, 10), 3);
/// assert_eq!(number_length_base_b(585, 2), 10);
/// ```
/// 
/// # Errors
/// Returns an error if the base is greater than 16.
pub fn number_length_with_base(num: i32, base: u32) -> Result<usize, &'static str>
{   
    if base > 16
    {
        return Err("Base must be less than 16.");
    }

    Ok(((num as f64).log(base as f64).floor()) as usize + 1)
}
