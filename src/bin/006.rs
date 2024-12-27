use project_euler::project_euler_solution;

project_euler_solution!(006);

fn project_euler_006() -> u32
{
    const UPPER_LIMIT: i32 = 100;

    let square_of_sum = i32::pow((1..=UPPER_LIMIT).sum(), 2);
    let sum_of_square = (1..=UPPER_LIMIT).map(|num| num * num).sum();

    square_of_sum.abs_diff(sum_of_square)
}
