use project_euler::project_euler_solution;
use project_euler::utils::gcd;

project_euler_solution!(033);

/// # Digit Cancelling Fractions
/// The fraction 49/98 is a curious fraction, as an inexperienced mathematician
/// in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which
/// is correct, is obtained by cancelling the 9s.
///
/// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
///
/// There are exactly four non-trivial examples of this type of fraction, less
/// than one in value, and containing two digits in the numerator and
/// denominator.
///
/// If the product of these four fractions is given in its lowest common terms,
/// find the value of the denominator.
fn project_euler_033() -> i32
{
    // ax/xb = a/b
    // (10a + x)/(10x + b) = a/b
    // 10ab + xb = 10ax + ab
    // 9ab + xb = 10ax

    let mut numerator_product = 1;
    let mut denominator_product = 1;

    for a in 1..10
    {
        for b in 1..10
        {
            for x in 1..10
            {
                if (9 * a * b + x * b) == (10 * a * x)
                {
                    numerator_product *= a;
                    denominator_product *= b;
                }
            }
        }
    }

    // Simplify.
    denominator_product /= gcd(numerator_product, denominator_product);

    denominator_product
}
