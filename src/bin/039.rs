use project_euler::project_euler_solution;
use std::collections::HashMap;

project_euler_solution!(039);

/// # Integer Right Triangles
/// If p is the perimeter of a right angle triangle with integral length sides,
/// {a,b,c}, there are exactly three solutions for p = 120.
///
/// {20,48,52}, {24,45,51}, {30,40,50}
///
/// For which value of p ≤ 1000, is the number of solutions maximised?
fn project_euler_039() -> i32
{
    let mut perimeters = HashMap::<i32, usize>::new();

    for z in 1..=500
    // a + b > c, so a + b + c > 2c, so c < 500.
    {
        for y in 1..z
        {
            for x in 1..y
            {
                if x * x + y * y == z * z
                {
                    let p = x + y + z;
                    *perimeters.entry(p).or_insert(0) += 1;
                }
            }
        }
    }

    perimeters
        .into_iter()
        .max_by_key(|&(_p, count)| count)
        .unwrap()
        .0
}
