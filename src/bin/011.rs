use num_traits::int::PrimInt;
use project_euler::{max, project_euler_solution};
use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

project_euler_solution!(011);

type Matrix<T> = Vec<Vec<T>>;

/// # Largest Product in a Grid
///
/// In the 20×20 grid below, four numbers along a diagonal line have been marked
/// in red.
///
/// /.../
///
/// The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
///
/// What is the greatest product of four adjacent numbers in the same direction
/// (up, down, left, right, or diagonally) in the 20×20 grid?
fn project_euler_011() -> u32
{
    let proj_dir = current_dir().unwrap();

    let matrix = parse_matrix(&proj_dir.join("data/011.txt"));

    max!(
        max_diagonal_mult(&matrix),
        max_horizontal_mult(&matrix),
        max_vertical_mult(&matrix)
    )
}

/// Parse the matrix from the file at the given path.
///
/// # Arguments
///
/// * `path` - The path to the file containing the matrix.
///
/// # Returns
///
/// The matrix parsed from the file.
fn parse_matrix(path: &PathBuf) -> Matrix<u32>
{
    /// Parse a line of numbers into an array.
    ///
    /// # Arguments
    ///
    /// * `line` - The line to parse.
    ///
    /// # Returns
    ///
    /// The array of numbers parsed from the line.
    fn parse_line(line: &str) -> Vec<u32>
    {
        line.split_whitespace()
            .map(|num| {
                num.parse()
                    .expect("Couldn't parse the number")
            })
            .collect()
    }

    let file = File::open(path).expect("Couldn't find file.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.expect("Couldn't read line.");
            parse_line(&line)
        })
        .collect()
}

/// Find the maximum product of four horizontally adjacent numbers in the
/// matrix.
///
/// # Arguments
///
/// * `matrix` - The matrix to search.
///
/// # Returns
///
/// The maximum product of four horizontally adjacent numbers in the matrix.
fn max_horizontal_mult<T: PrimInt>(matrix: &Matrix<T>) -> T
{
    let mut max = T::zero();

    for row in matrix
    {
        for i in 0..row.len() - 3
        {
            let product = row[i] * row[i + 1] * row[i + 2] * row[i + 3];
            max = max.max(product);
        }
    }

    max
}

/// Find the maximum product of four vertically adjacent numbers in the matrix.
///
/// # Arguments
///
/// * `matrix` - The matrix to search.
///
/// # Returns
///
/// The maximum product of four vertically adjacent numbers in the matrix.
fn max_vertical_mult<T: PrimInt>(matrix: &Matrix<T>) -> T
{
    let mut max = T::zero();

    for i in 0..matrix.len() - 3
    {
        for j in 0..matrix[i].len()
        {
            let product = matrix[i][j] * matrix[i + 1][j] * matrix[i + 2][j] * matrix[i + 3][j];
            max = max.max(product);
        }
    }

    max
}

/// Find the maximum product of four diagonally adjacent numbers in the matrix.
///
/// Both top left to bottom right and top right to bottom left.
///
/// # Arguments
///
/// * `matrix` - The matrix to search.
///
/// # Returns
///
/// The maximum product of four diagonally adjacent numbers in the matrix.
fn max_diagonal_mult<T: PrimInt>(matrix: &Matrix<T>) -> T
{
    let mut max = T::zero();

    // Top left to bottom right.
    for i in 0..matrix.len() - 3
    {
        for j in 0..matrix[i].len() - 3
        {
            let product =
                matrix[i][j] * matrix[i + 1][j + 1] * matrix[i + 2][j + 2] * matrix[i + 3][j + 3];
            max = max.max(product);
        }
    }

    // Top right to bottom left.
    for i in 0..matrix.len() - 3
    {
        for j in 3..matrix[i].len()
        {
            let product =
                matrix[i][j] * matrix[i + 1][j - 1] * matrix[i + 2][j - 2] * matrix[i + 3][j - 3];
            max = max.max(product);
        }
    }

    max
}


#[cfg(test)]
mod tests
{
    use super::*;
    use std::sync::LazyLock;

    static MATRIX: LazyLock<Matrix<u32>> = LazyLock::new(|| {
        vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ]
    });

    #[test]
    fn test_parse_matrix()
    {
        let matrix = parse_matrix(&PathBuf::from("data/011.txt"));

        assert_eq!(matrix.len(), 20);
        assert_eq!(matrix[0].len(), 20);
    }

    #[test]
    fn test_max_horizontal_mult()
    {
        assert_eq!(max_horizontal_mult(&MATRIX), 43680);
    }

    #[test]
    fn test_max_vertical_mult()
    {
        assert_eq!(max_vertical_mult(&MATRIX), 6144);
    }

    #[test]
    fn test_max_diagonal_mult()
    {
        assert_eq!(max_diagonal_mult(&MATRIX), 3640);
    }
}
