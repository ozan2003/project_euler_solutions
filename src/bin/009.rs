use project_euler::project_euler_solution;

project_euler_solution!(009);

/// # Pythagorean triples
///
/// Generates Pythagorean triples using Euclid's formula.
///
/// # Example
/// ```
/// let triples: Vec<(i32, i32, i32)> = pythagorean_triples().take(5).collect();
///
/// assert_eq!(triples, vec![
///    (3, 4, 5),
///    (8, 6, 10),
///    (5, 12, 13),
///    (15, 8, 17),
///    (12, 16, 20),
/// ]);
/// ```
fn pythagorean_triples() -> impl Iterator<Item = (i32, i32, i32)>
{
    // https://mathworld.wolfram.com/PythagoreanTriple.html

    // 1, 2, 3,
    // v\u
    // f(), f(1, 1), f(2, 1), f(3, 1)...
    //      f(1, 2), f(2, 2), f(3, 2)...
    //               f(2, 3), f(3, 3)...
    //   .     .        .        .
    (1..).flat_map(|v| (1..v).map(move |u| (v * v - u * u, 2 * u * v, v * v + u * u)))
}

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
fn project_euler_009() -> Option<i32>
{
    pythagorean_triples()
        .find(|&(a, b, c)| a + b + c == 1000)
        .map(|(a, b, c)| a * b * c)
}
