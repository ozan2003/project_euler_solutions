use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use dashu::Integer;
use project_euler::project_euler_solution;

project_euler_solution!(013);

/// # Large Sum
/// Work out the first ten digits of the sum of the following one-hundred
/// 50-digit numbers.
///
/// / ... /
fn project_euler_013() -> String
{
    let proj_dir = std::env::current_dir().unwrap();

    let file =
        File::open(proj_dir.join("data/013.txt")).expect("Couldn't find file.");

    let buf = BufReader::new(file);

    let sum = buf
        .lines()
        .map(|line| line.unwrap().parse::<Integer>().unwrap())
        .sum::<Integer>();

    sum.to_string().chars().take(10).collect()
}
