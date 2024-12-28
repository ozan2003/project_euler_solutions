use chrono::{Datelike, NaiveDate, Weekday};
use project_euler::project_euler_solution;

project_euler_solution!(019);

/// # Counting Sundays
/// You are given the following information, but you may prefer to do some
/// research for yourself.
///
/// - 1 Jan 1900 was a Monday.
/// - Thirty days has September, April, June and November. All the rest have
///   thirty-one, Saving February alone, Which has twenty-eight, rain or shine.
///   And on leap years, twenty-nine.
/// - A leap year occurs on any year evenly divisible by 4, but not on a century
///   unless it is divisible by 400.
///
/// How many Sundays fell on the first of the month during the twentieth century
/// (1 Jan 1901 to 31 Dec 2000)?
fn project_euler_019() -> i32
{
    let mut tally = 0;

    for year in 1901..=2000
    {
        for month in 1..=12
        {
            let date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

            if date.weekday() == Weekday::Sun
            {
                tally += 1;
            }
        }
    }

    tally
}
