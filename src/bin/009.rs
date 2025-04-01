use project_euler::project_euler_solution;

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
fn project_euler_009() -> i32
{
    pythagorean_triples()
        .find(|&(a, b, c)| a + b + c == 1000)
        .map(|(a, b, c)| a * b * c)
        .unwrap()
}

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

    // {1, 2, 3, ...}
    // {x_1, x_2, x_3, ...}
    // x = f(v, u) = (v^2 - u^2, 2uv, v^2 + u^2)
    (1..).flat_map(|v| (1..v).map(move |u| (v * v - u * u, 2 * u * v, v * v + u * u)))
}

