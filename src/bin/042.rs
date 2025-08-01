use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::sync::LazyLock;

use project_euler::project_euler_solution;
use project_euler::utils::is_integer;

project_euler_solution!(042);

/// # Coded Triangle Numbers
///
/// The nth term of the sequence of triangle numbers is given by, `t_n` = 1/2 *
/// n
/// * (n + 1); so the first ten triangle numbers are:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// By converting each letter in a word to a number corresponding to its
/// alphabetical position and adding these values we form a word value. For
/// example, the word value for SKY is 19 + 11 + 25 = 55 = `t_10`. If the word
/// value is a triangle number then we shall call the word a triangle word.
///
/// Using data/042.txt a 16K text file containing nearly two-thousand
/// common English words, how many are triangle words?
fn project_euler_042() -> usize
{
    let proj_dir = std::env::current_dir().unwrap();

    let file = std::fs::File::open(proj_dir.join("data/042.txt"))
        .expect("Couldn't find file.");

    let buf = BufReader::new(file);

    buf.split(b',')
        // Extract the word.
        .map(|name| {
            name.unwrap()
                .into_iter()
                // trim the quotes.
                .skip(1)
                .take_while(|&ch| ch != b'"')
                .collect::<Vec<u8>>()
        })
        .map(|word| word_value(&word)) // Get the word value.
        .filter(|&value| is_triangular(value))
        .count()
}

static ALPHABET: LazyLock<HashMap<u8, i32>> =
    LazyLock::new(|| (b'A'..=b'Z').zip(1..=26).collect());

// Check if n is triangular number.
fn is_triangular(n: i32) -> bool
{
    // let n mth triangular number.
    // n = 1/2 * m * (m + 1)
    // m = (sqrt(8n + 1) - 1) / 2
    // n is triangular number if m is integer.
    let m = (f64::from(8 * n + 1).sqrt() - 1.0) / 2.0;
    is_integer(m)
}

// Get the word value, sum of the index of each character in alphabet in the
// word.
fn word_value(word: &[u8]) -> i32
{
    word.iter()
        .map(|ch| ALPHABET.get(ch).unwrap())
        .sum()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_is_triangular()
    {
        assert!(is_triangular(1));
        assert!(is_triangular(3));
        assert!(is_triangular(6));
        assert!(is_triangular(10));
        assert!(!is_triangular(11));
        assert!(is_triangular(15));
    }

    #[test]
    fn test_word_value()
    {
        assert_eq!(word_value(b"SKY"), 55);
    }
}
