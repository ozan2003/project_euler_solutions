use project_euler::project_euler_solution;
use std::collections::HashMap;
use std::sync::LazyLock;

const TEN: u64 = 10;
const HUNDRED: u64 = 100;
const THOUSAND: u64 = 1_000;
const MILLION: u64 = 1_000_000;
const BILLION: u64 = 1_000_000_000;
const TRILLION: u64 = 1_000_000_000_000;
const QUADRILLION: u64 = 1_000_000_000_000_000;
const QUINTILLION: u64 = 1_000_000_000_000_000_000;

project_euler_solution!(017);

/// # Number Letter Counts
/// If the numbers 1 to 5 are written out in words: one, two, three, four, five,
/// then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
///
/// If all the numbers from 1 to 1000 (one thousand) inclusive were written out
/// in words, how many letters would be used?
///
/// **NOTE**: Do not count spaces or hyphens. For example, 342 (three hundred
/// and forty-two) contains 23 letters and 115 (one hundred and fifteen)
/// contains 20 letters. The use of "and" when writing out numbers is in
/// compliance with British usage.
fn project_euler_017() -> usize
{
    (1..=1000)
        // Count individual letters instead of bytes for correctness.
        // Coulda used Vec<u8> or len() since all number names are ASCII compliant.
        .map(|num| number_to_text(num).chars().count())
        .sum()
}

// Return the numerical representation of the given number.
fn number_to_text(mut num: u64) -> String
{
    static WORD_REPR: LazyLock<HashMap<u64, &str>> = LazyLock::new(|| {
        HashMap::from([
            (0, "zero"),
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven"),
            (8, "eight"),
            (9, "nine"),
            (TEN, "ten"),
            (11, "eleven"),
            (12, "twelve"),
            (13, "thirteen"),
            (14, "fourteen"),
            (15, "fifteen"),
            (16, "sixteen"),
            (17, "seventeen"),
            (18, "eighteen"),
            (19, "nineteen"),
            (20, "twenty"),
            (30, "thirty"),
            (40, "forty"),
            (50, "fifty"),
            (60, "sixty"),
            (70, "seventy"),
            (80, "eighty"),
            (90, "ninety"),
            (HUNDRED, "hundred"),
            (THOUSAND, "thousand"),
            (MILLION, "million"),
            (BILLION, "billion"),
            (TRILLION, "trillion"),
            (QUADRILLION, "quadrillion"),
            (QUINTILLION, "quintillion"),
        ])
    });

    let mut word = String::new();

    // Look-ups for numbers less than 1000.
    if num < 20
    {
        word.push_str(WORD_REPR.get(&num).unwrap());
    }
    else if num < HUNDRED
    {
        word.push_str(WORD_REPR.get(&(num / TEN * TEN)).unwrap());

        // Combine the tens with hyphen.
        if num % TEN != 0
        {
            //word.push('-');
            word.push_str(WORD_REPR.get(&(num % TEN)).unwrap());
        }
    }
    else if num < THOUSAND
    {
        word.push_str(WORD_REPR.get(&(num / HUNDRED)).unwrap());
        //word.push(' ');
        word.push_str(WORD_REPR.get(&HUNDRED).unwrap());

        if num % HUNDRED != 0
        {
            //word.push_str(" and ");
            word.push_str("and");
            word.push_str(&number_to_text(num % HUNDRED));
        }
    }
    else
    {
        // Break down denominations, organized from smallest to largest.
        static DENOMINATIONS: [(u64, &str); 6] = [
            (QUINTILLION, "quintillion"),
            (QUADRILLION, "quadrillion"),
            (TRILLION, "trillion"),
            (BILLION, "billion"),
            (MILLION, "million"),
            (THOUSAND, "thousand"),
        ];

        //let mut is_in_between = false;

        for &(denomination, name) in &DENOMINATIONS
        {
            if num >= denomination
            {
                /*if is_in_between
                {
                    word.push(' ');
                }*/
                if num / denomination > 0
                {
                    // Get the higher-order part.
                    word.push_str(&number_to_text(num / denomination));
                    //word.push(' ');
                    word.push_str(name);
                }
                num %= denomination; // Focus on the remainding portion.
                // One portion has been processed, meaning that the next portion
                // will be separated by a space.
                //is_in_between = true;
            }
        }

        // Process the last portion, which is less than 100.
        if num > 0
        {
            /*if is_in_between
            {
                word.push(' ');
            }*/
            word.push_str(&number_to_text(num));
        }
    }

    word
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_number_to_text()
    {
        assert_eq!(number_to_text(0), "zero");
        assert_eq!(number_to_text(1), "one");
        assert_eq!(number_to_text(16), "sixteen");
        assert_eq!(number_to_text(20), "twenty");
        assert_eq!(number_to_text(45), "fortyfive");
        assert_eq!(number_to_text(100), "onehundred");
        assert_eq!(number_to_text(101), "onehundredandone");
        assert_eq!(number_to_text(999), "ninehundredandninetynine");
        assert_eq!(number_to_text(1000), "onethousand");
        assert_eq!(number_to_text(1234), "onethousandtwohundredandthirtyfour");
        assert_eq!(number_to_text(1000000), "onemillion");
        assert_eq!(number_to_text(1000001), "onemillionone");
        assert_eq!(number_to_text(1000000000), "onebillion");
        assert_eq!(number_to_text(1000000001), "onebillionone");
        assert_eq!(number_to_text(1000000000000), "onetrillion");
        assert_eq!(number_to_text(1000000000000000), "onequadrillion");
    }
}
