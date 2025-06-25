use project_euler::project_euler_solution;
use project_euler::utils::pythagorean_triples;

project_euler_solution!(009);

/// # Special Pythagorean Triplet
///
/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for
/// which,
///
/// a^2 + b^2 = c^2
///
/// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
///
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
fn project_euler_009() -> u32
{
    pythagorean_triples()
        .find(|&(a, b, c)| a + b + c == 1000)
        .map(|(a, b, c)| a * b * c)
        .unwrap()
}
