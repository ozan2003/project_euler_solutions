use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use project_euler::project_euler_solution;

project_euler_solution!(008);

/// # Largest Product in a Series
///
/// The four adjacent digits in the 1000-digit number that have the greatest
/// product are 9 * 9 * 8 * 9 = 5832.
///
/// / ... /
///
/// Find the thirteen adjacent digits in the 1000-digit number that have the
/// greatest product. What is the value of this product?
fn project_euler_008() -> u64
{
    let proj_dir = env::current_dir().unwrap();

    let file =
        File::open(proj_dir.join("data/008.txt")).expect("Couldn't find file.");

    let nums: Vec<u64> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        // Flatten the line into a char iterator otherwise we'll get a Vec<char>
        // iterator.
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .map(|ch| u64::from(ch.to_digit(10).unwrap()))
        .collect();

    nums.windows(13)
        .map(|window| window.iter().product())
        .max()
        .unwrap()
}
