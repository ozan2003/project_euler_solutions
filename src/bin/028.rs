use project_euler::project_euler_solution;

project_euler_solution!(028);

/// # Number spiral diagonals
///
/// Starting with the number 1 and moving to the right in a clockwise direction
/// a 5 by 5 spiral is formed as follows:
///
/// ```
/// 21 22 23 24 25
/// 20  7  8  9 10
/// 19  6  1  2 11
/// 18  5  4  3 12
/// 17 16 15 14 13
/// ```
///
/// It can be verified that the sum of the numbers on the diagonals is 101.
///
/// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral
/// formed in the same way?
fn project_euler_028() -> usize
{
    diagonal_sum(&spiral_matrix(1001).unwrap())
}

type Matrix<T> = Vec<Vec<T>>;

/// Create a square matrix of size `n` with numbers spiraling clockwise from the
/// center.
///
/// # Examples
///
/// ```
/// let n = 5;
/// let mtx = spiral_matrix(n).unwrap();
/// assert_eq!(mtx, vec![
///     vec![21, 22, 23, 24, 25],
///     vec![20, 7, 8, 9, 10],
///     vec![19, 6, 1, 2, 11],
///     vec![18, 5, 4, 3, 12],
///     vec![17, 16, 15, 14, 13],
/// ]);
/// ```
fn spiral_matrix(n: usize) -> Option<Matrix<usize>>
{
    if n == 0 || n % 2 != 1
    {
        return None;
    }

    let mut matrix = vec![vec![0; n]; n];

    let mid = n / 2;
    let mut col = mid;
    let mut row = mid;

    let mut num = 1;
    matrix[row][col] = num;

    for layer in 1..=mid
    {
        // Move right
        col += 1;
        num += 1;
        matrix[row][col] = num;

        // Move down
        for _ in 0..(2 * layer - 1)
        {
            row += 1;
            num += 1;
            matrix[row][col] = num;
        }

        // Move left
        for _ in 0..(2 * layer)
        {
            col -= 1;
            num += 1;
            matrix[row][col] = num;
        }

        // Move up
        for _ in 0..(2 * layer)
        {
            row -= 1;
            num += 1;
            matrix[row][col] = num;
        }

        // Move right
        for _ in 0..(2 * layer)
        {
            col += 1;
            num += 1;
            matrix[row][col] = num;
        }
    }

    Some(matrix)
}

/// Calculate the sum of the numbers on the two diagonals of a square matrix.
///
/// # Examples
///
/// ```
/// let mtx = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![7, 8, 9],
/// ];
/// let sum = diagonal_sum(&mtx);
/// assert_eq!(sum, 25);
/// ```
fn diagonal_sum(mtx: &Matrix<usize>) -> usize
{
    let mut sum = 0;
    let n = mtx.len();

    // Top-left to bottom-right.
    sum += (0..n).map(|i| mtx[i][i]).sum::<usize>();

    // Top-right to bottom-left.
    sum += (0..n)
        .map(|i| mtx[i][n - i - 1])
        .sum::<usize>();

    // If the size is odd, subtract the center which was counted twice.
    if n % 2 == 1
    {
        sum -= mtx[n / 2][n / 2];
    }

    sum
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_spiral_matrix()
    {
        let n = 5;
        let mtx = spiral_matrix(n).unwrap();
        #[rustfmt::skip]
        let expected = vec![
            vec![21, 22,  23, 24, 25],
            vec![20,  7,   8,  9, 10],
            vec![19,  6,   1,  2, 11],
            vec![18,  5,   4,  3, 12],
            vec![17, 16,  15, 14, 13],
        ];

        assert_eq!(mtx, expected);
    }

    #[test]
    fn test_diagonal_sum()
    {
        let n = 5;
        let mtx = spiral_matrix(n).unwrap();
        let sum = diagonal_sum(&mtx);
        assert_eq!(sum, 101);

        let n = 3;
        let mtx = spiral_matrix(n).unwrap();
        let sum = diagonal_sum(&mtx);
        assert_eq!(sum, 25);
    }
}
