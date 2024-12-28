use phf::{Map, phf_map};
use project_euler::project_euler_solution;

const TEN: u64 = 10;
const HUNDRED: u64 = 100;
const THOUSAND: u64 = 1_000;
const MILLION: u64 = 1_000_000;
const BILLION: u64 = 1_000_000_000;
const TRILLION: u64 = 1_000_000_000_000;
const QUADRILLION: u64 = 1_000_000_000_000_000;
const QUINTILLION: u64 = 1_000_000_000_000_000_000;

project_euler_solution!(017);

// Return the numerical representation of the given number.
#[allow(clippy::too_many_lines)]
fn number_to_text(num: u64) -> String
{
    static WORD_REPR: Map<u64, &str> = phf_map! {
        0_u64 => "Zero",
        1_u64 => "One",
        2_u64 => "Two",
        3_u64 => "Three",
        4_u64 => "Four",
        5_u64 => "Five",
        6_u64 => "Six",
        7_u64 => "Seven",
        8_u64 => "Eight",
        9_u64 => "Nine",
        10_u64 => "Ten",
        11_u64 => "Eleven",
        12_u64 => "Twelve",
        13_u64 => "Thirteen",
        14_u64 => "Fourteen",
        15_u64 => "Fifteen",
        16_u64 => "Sixteen",
        17_u64 => "Seventeen",
        18_u64 => "Eighteen",
        19_u64 => "Nineteen",
        20_u64 => "Twenty",
        30_u64 => "Thirty",
        40_u64 => "Forty",
        50_u64 => "Fifty",
        60_u64 => "Sixty",
        70_u64 => "Seventy",
        80_u64 => "Eighty",
        90_u64 => "Ninety",
        100_u64 => "Hundred",
        1000_u64 => "Thousand",
        1_000_000_u64 => "Million",
        1_000_000_000_u64 => "Billion",
        1_000_000_000_000_u64 => "Trillion",
        1_000_000_000_000_000_u64 => "Quadrillion",
        1_000_000_000_000_000_000_u64 => "Quintillion",
    };

    let mut word = String::new();

    if num < 20
    {
        word.push_str(WORD_REPR.get(&num).unwrap());
    }
    else if num < HUNDRED
    {
        word.push_str(WORD_REPR.get(&(num / TEN * TEN)).unwrap());
        if num % TEN != 0
        {
            word.push('-');
            word.push_str(WORD_REPR.get(&(num % TEN)).unwrap());
        }
    }
    else if num < THOUSAND
    {
        word.push_str(WORD_REPR.get(&(num / HUNDRED)).unwrap());
        word.push_str(WORD_REPR.get(&HUNDRED).unwrap());

        if num % HUNDRED != 0
        {
            //word.push_str(" and ");
            word.push_str("and");
            word.push_str(&number_to_text(num % HUNDRED));
        }
    }
    else if num < MILLION
    {
        word.push_str(&number_to_text(num / THOUSAND));
        word.push_str(WORD_REPR.get(&THOUSAND).unwrap());

        if num % THOUSAND != 0
        {
            word.push_str(&number_to_text(num % THOUSAND));
        }
    }
    else if num < BILLION
    {
        word.push_str(&number_to_text(num / MILLION));
        word.push_str(WORD_REPR.get(&MILLION).unwrap());

        if num % MILLION != 0
        {
            word.push_str(&number_to_text(num % MILLION));
        }
    }
    else if num < TRILLION
    {
        word.push_str(&number_to_text(num / BILLION));
        word.push_str(WORD_REPR.get(&BILLION).unwrap());

        if num % BILLION != 0
        {
            word.push_str(&number_to_text(num % BILLION));
        }
    }
    else if num < QUADRILLION
    {
        word.push_str(&number_to_text(num / TRILLION));
        word.push_str(WORD_REPR.get(&TRILLION).unwrap());

        if num % TRILLION != 0
        {
            word.push_str(&number_to_text(num % TRILLION));
        }
    }
    else if num < QUINTILLION
    {
        word.push_str(&number_to_text(num / QUADRILLION));
        word.push_str(WORD_REPR.get(&QUADRILLION).unwrap());
    }

    word
}

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
        .map(|num| number_to_text(num).len())
        .sum()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_number_to_text()
    {
        assert_eq!(number_to_text(0), "Zero");
        assert_eq!(number_to_text(1), "One");
        assert_eq!(number_to_text(16), "Sixteen");
        assert_eq!(number_to_text(20), "Twenty");
        assert_eq!(number_to_text(45), "Forty-Five");
        assert_eq!(number_to_text(100), "OneHundred");
        assert_eq!(number_to_text(101), "OneHundredandOne");
        assert_eq!(number_to_text(999), "NineHundredandNinety-Nine");
        assert_eq!(number_to_text(1000), "OneThousand");
        assert_eq!(number_to_text(1234), "OneThousandTwoHundredandThirty-Four");
        assert_eq!(number_to_text(1000000), "OneMillion");
        assert_eq!(number_to_text(1000001), "OneMillionOne");
        assert_eq!(number_to_text(1000000000), "OneBillion");
        assert_eq!(number_to_text(1000000001), "OneBillionOne");
        assert_eq!(number_to_text(1000000000000), "OneTrillion");
        assert_eq!(number_to_text(1000000000000000), "OneQuadrillion");
    }
}
