use project_euler::project_euler_solution;

project_euler_solution!(014);

fn collatz_seq(mut num: u64) -> impl Iterator<Item = u64>
{
    std::iter::from_fn(move || {
        if num > 1
        {
            if num % 2 == 0
            {
                num /= 2;
            }
            else
            {
                num = 3 * num + 1;
            }

            Some(num)
        }
        else
        {
            None
        }
    })
}

/// # Longest Collatz sequence
/// The following iterative sequence is defined for the set of positive
/// integers: n -> n/2 (n is even)
/// n -> 3n + 1 (n is odd)
///
/// Using the rule above and starting with 13, we generate the following
/// sequence:
///
/// 13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
///
/// It can be seen that this sequence (starting at 13 and finishing at 1)
/// contains 10 terms. Although it has not been proved yet (Collatz Problem), it
/// is thought that all starting numbers finish at 1.
///
/// Which starting number, under one million, produces the longest chain?
///
/// NOTE: Once the chain starts the terms are allowed to go above one million.
fn project_euler_014() -> u64
{
    const UPPER_LIMIT: u64 = 1_000_000;

    (1..UPPER_LIMIT)
        .max_by_key(|&num| collatz_seq(num).count())
        .unwrap()
}
