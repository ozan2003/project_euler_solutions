use project_euler::project_euler_solution;

project_euler_solution!(004);

/// Check if a number is a palindrome.
/// 
/// # Examples
/// ```
/// assert!(is_palindrome(90509));
/// assert!(!is_palindrome(1234));
/// ```
fn is_palindrome(num: i32) -> bool
{
    let num_str = num.to_string();

    num_str.chars().eq(num_str.chars().rev())
}

/// # Largest palindrome product
///
/// A palindromic number reads the same both ways. The largest palindrome made
/// from the product of two 2-digit numbers is 9009 = 91 * 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
fn project_euler_004() -> Option<i32>
{
    let mut max_palindrome = 0;

    for i in 100..1000
    {
        for j in 100..1000
        {
            let product = i * j;
            if is_palindrome(product)
            {
                max_palindrome = max_palindrome.max(product);
            }
        }
    }

    Some(max_palindrome)
}
