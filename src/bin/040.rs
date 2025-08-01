use project_euler::project_euler_solution;

project_euler_solution!(040);

/// # Champernowne's Constant
///
/// An irrational decimal fraction is created by concatenating the positive
/// integers:
///
/// 0.12345678910`1`112131415161718192021...
///
/// It can be seen that the 12th digit of the fractional part is 1.
///
/// If `d_n` represents the nth digit of the fractional part, find the value of
/// the following expression:
///
/// `d_1` * `d_10` * `d_100` * `d_1000` * `d_10000` * `d_100000` * `d_1000000`
fn project_euler_040() -> u8
{
    const UPPER_LIMIT: usize = 1_000_000;

    // 1, 2, 3, 4, ..., 10, 11, 12, 13, 14, 15, ...
    // 1, 2, 3, 4, ..., 1, 0, 1, 1, 1, 2, 1, 3, ...
    let digits: Vec<u8> = (1..)
        .flat_map(|n| {
            n.to_string()
                .into_bytes()
                .into_iter()
                .map(|b| b - b'0')
        })
        .take(UPPER_LIMIT)
        .collect();

    (0..=6)
        .map(|i| digits[10_usize.pow(i) - 1])
        .product()
}
