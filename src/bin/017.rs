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

// Return the numerical representation of the given number.
#[allow(clippy::too_many_lines)]
fn number_to_text(num: u64) -> String
{
    static WORD_REPR: LazyLock<HashMap<u64, &str>> = LazyLock::new(|| {
        HashMap::from([
            (0, "Zero"),
            (1, "One"),
            (2, "Two"),
            (3, "Three"),
            (4, "Four"),
            (5, "Five"),
            (6, "Six"),
            (7, "Seven"),
            (8, "Eight"),
            (9, "Nine"),
            (TEN, "Ten"),
            (11, "Eleven"),
            (12, "Twelve"),
            (13, "Thirteen"),
            (14, "Fourteen"),
            (15, "Fifteen"),
            (16, "Sixteen"),
            (17, "Seventeen"),
            (18, "Eighteen"),
            (19, "Nineteen"),
            (20, "Twenty"),
            (30, "Thirty"),
            (40, "Forty"),
            (50, "Fifty"),
            (60, "Sixty"),
            (70, "Seventy"),
            (80, "Eighty"),
            (90, "Ninety"),
            (HUNDRED, "Hundred"),
            (THOUSAND, "Thousand"),
            (MILLION, "Million"),
            (BILLION, "Billion"),
            (TRILLION, "Trillion"),
            (QUADRILLION, "Quadrillion"),
            (QUINTILLION, "Quintillion"),
        ])
    });

    let mut word = String::new();

    /*
     * Start from the biggest number and work our way down to the smallest,
     * recursively.
     */
    if num < 20
    {
        word.push_str(WORD_REPR.get(&num).unwrap());
    }
    else if num < HUNDRED
    {
        // Number <- Number / 10 * 10 + Number % 10
        word.push_str(WORD_REPR.get(&(num / TEN * TEN)).unwrap());
        if num % TEN != 0
        {
            //word.push('-');
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
        // Count individual letters instead of bytes for correctness.
        // Coulda used Vec<u8> or len() since all number names are ASCII compliant.
        .map(|num| number_to_text(num).chars().count())
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
        assert_eq!(number_to_text(45), "FortyFive");
        assert_eq!(number_to_text(100), "OneHundred");
        assert_eq!(number_to_text(101), "OneHundredandOne");
        assert_eq!(number_to_text(999), "NineHundredandNinetyNine");
        assert_eq!(number_to_text(1000), "OneThousand");
        assert_eq!(number_to_text(1234), "OneThousandTwoHundredandThirtyFour");
        assert_eq!(number_to_text(1000000), "OneMillion");
        assert_eq!(number_to_text(1000001), "OneMillionOne");
        assert_eq!(number_to_text(1000000000), "OneBillion");
        assert_eq!(number_to_text(1000000001), "OneBillionOne");
        assert_eq!(number_to_text(1000000000000), "OneTrillion");
        assert_eq!(number_to_text(1000000000000000), "OneQuadrillion");
    }
}
