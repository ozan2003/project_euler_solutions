use project_euler::project_euler_solution;
use std::collections::BTreeMap;

project_euler_solution!(039);

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

/// # Integer Right Triangles
/// If p is the perimeter of a right angle triangle with integral length sides,
/// {a,b,c}, there are exactly three solutions for p = 120.
///
/// {20,48,52}, {24,45,51}, {30,40,50}
///
/// For which value of p ≤ 1000, is the number of solutions maximised?
// TODO: Why it gives 672?
fn project_euler_039() -> i32
{
    let mut perimeters = BTreeMap::<i32, usize>::new();

    for (a, b, c) in pythagorean_triples().take_while(|&(x, y, z)| x + y + z <= 1000)
    {
        let perimeter = a + b + c;
        *perimeters.entry(perimeter).or_insert(0) += 1;
    }

    perimeters
        .into_iter()
        .max_by_key(|&(_p, count)| count)
        .unwrap()
        .0
}
