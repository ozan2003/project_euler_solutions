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
            if weekday(year, month, 1) == Weekday::Sunday
            {
                tally += 1;
            }
        }
    }

    tally
}

#[derive(Debug, PartialEq)]
enum Weekday
{
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// Calculate the day of the week for a given date.
/// This function uses Zeller's congruence to calculate the day of the week.
///
/// # Arguments
/// * `year` - The year of the date.
/// * `month` - The month of the date.
/// * `day` - The day of the date.
///
/// # Returns
/// The day of the week for the given date
fn weekday(mut year: i32, mut month: i32, day: i32) -> Weekday
{
    if month < 3
    {
        month += 12;
        year -= 1;
    }

    let h =
        day + 13 * (month + 1) / 5 + year + year / 4 - year / 100 + year / 400;

    match h.rem_euclid(7)
    {
        0 => Weekday::Saturday,
        1 => Weekday::Sunday,
        2 => Weekday::Monday,
        3 => Weekday::Tuesday,
        4 => Weekday::Wednesday,
        5 => Weekday::Thursday,
        6 => Weekday::Friday,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_weekday()
    {
        const CASES: &[(i32, i32, i32, Weekday)] = &[
            (2000, 1, 1, Weekday::Saturday),
            (2000, 2, 29, Weekday::Tuesday),
            (2000, 3, 1, Weekday::Wednesday),
            (1999, 12, 31, Weekday::Friday),
            (2023, 10, 23, Weekday::Monday),
            (1752, 9, 14, Weekday::Thursday),
            (1582, 10, 15, Weekday::Friday),
            (2071, 6, 13, Weekday::Saturday),
        ];

        for &(year, month, day, ref expected) in CASES
        {
            assert_eq!(weekday(year, month, day), *expected);
        }
    }
}
