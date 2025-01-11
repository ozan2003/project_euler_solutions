use project_euler::project_euler_solution;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::fs::File;
use std::io::{BufReader, prelude::*};

project_euler_solution!(099);

struct Exponent
{
    base: u32,
    exponent: u32,
}

impl PartialEq for Exponent
{
    fn eq(&self, other: &Self) -> bool
    {
        // a^b = c^d
        // b * log(a) = d * log(c)
        (f64::from(self.exponent) * f64::from(self.base).ln()) ==
            (f64::from(other.exponent) * f64::from(other.base).ln())
    }
}

impl Eq for Exponent
{
}

impl Ord for Exponent
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        (f64::from(self.exponent) * f64::from(self.base).ln())
            .partial_cmp(&(f64::from(other.exponent) * f64::from(other.base).ln()))
            .unwrap()
    }
}

impl PartialOrd for Exponent
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        Some(self.cmp(other))
    }
}

/// # Largest Exponential
/// Comparing two numbers written in index form like 2^11 and 3^7 is not
/// difficult, as any calculator would confirm that 2^11 = 2048 < 3^7 = 2187.
///
/// However, confirming that 632382^518061 > 519432^525806 would be much more
/// difficult, as both numbers contain over three million digits.
///
/// Using `base_exp.txt` (right click and 'Save Link/Target As...'), a 22K text
/// file containing one thousand lines with a base/exponent pair on each line,
/// determine which line number has the greatest numerical value.
///
/// NOTE: The first two lines in the file represent the numbers in the example
/// given above.
fn project_euler_099() -> usize
{
    let proj_dir = std::env::current_dir().unwrap();

    let file =
        File::open(format!("{}/data/099.txt", proj_dir.display())).expect("Couldn't find file.");

    let buf = BufReader::new(file);

    let (_max_exp, line_num) = buf
        .lines()
        .zip(1..)
        .max_by_key(|(nums, _line_num)| {
            let (base, exp) = nums
                // Result itself is reference, borrow its internal Ok(...) instead.
                .as_ref()
                .unwrap()
                .split_once(',')
                .unwrap();

            Exponent {
                base: base.parse().unwrap(),
                exponent: exp.parse().unwrap(),
            }
        })
        .unwrap();

    line_num
}
