use dashu::Integer;
use project_euler::project_euler_solution;

project_euler_solution!(048);

/// # Self Powers
/// The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
///
/// Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
fn project_euler_048() -> String
{
    let sum: Integer = (1..=1000)
        .map(|n| Integer::from(n).pow(n))
        .sum();

    let string = sum.to_string();
    // last 10 digits
    let index = string.char_indices().nth_back(9).unwrap().0;

    string[index..].to_string()
}
