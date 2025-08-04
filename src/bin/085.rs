use project_euler::project_euler_solution;

project_euler_solution!(085);

/// # Counting Rectangles
///
/// By counting carefully it can be seen that a rectangular grid measuring 3 by
/// 2 contains eighteen rectangles:
///
/// /* snip */
///
/// Although there exists no rectangular grid that contains exactly two million
/// rectangles, find the area of the grid with the nearest solution.
fn project_euler_085() -> usize
{
    // i*j * (i + 1)*(j + 1) / 4
    //
    // i*j * i*j => i**2 * j**2 / 4
    //
    // i**2 * j**2 / 4 = 2_000_000
    //
    // i**2 * j**2 = 8_000_000
    //
    // i * j = 2828
    const UPPER_LIMIT: usize = 2828;
    const TARGET_RECT_COUNT: usize = 2_000_000;

    let mut closest_count: usize = 0;
    let mut area = 0;

    // x * y = UPPER_LIMIT is symmetric, so we only need to check
    // m in the range [1, UPPER_LIMIT / 2]
    for m in 1..(UPPER_LIMIT / 2)
    {
        let n = UPPER_LIMIT / m;

        let count = rectangle_count(m, n);

        if count.abs_diff(TARGET_RECT_COUNT) <
            closest_count.abs_diff(TARGET_RECT_COUNT)
        {
            closest_count = count;
            area = m * n;
        }
    }

    area
}

/// Return the count of all possible rectangles (of any size) that can be formed
/// within an m x n grid.
const fn rectangle_count(m: usize, n: usize) -> usize
{
    // Total rectangles = C(m+1, 2) * C(n+1, 2) = (m*(m+1)/2) * (n*(n+1)/2)
    //                  = i*(i + 1) / 2 * j*(j + 1) / 2
    //                  = i*j * (i + 1)*(j + 1) / 4
    m * n * (m + 1) * (n + 1) / 4
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_rectangle_count()
    {
        assert_eq!(rectangle_count(3, 2), 18);
        assert_eq!(rectangle_count(2, 3), 18);
    }
}
