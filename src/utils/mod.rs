/// A macro that generates a main function for Project Euler solutions.
///
/// This macro creates a standardized main function that:
/// - Records the start time
/// - Calls the solution function for a specific problem
/// - Measures and prints the elapsed time
/// - Prints the result
///
/// # Arguments
///
/// * `$number` - The problem number. Used to construct the function name
///               in the format `project_euler_N` where N is the problem number.
///
/// Note: Requires the `paste` crate for identifier concatenation.
#[macro_export]
macro_rules! project_euler_solution {
    ($number:expr) => {
        paste::paste! {
            fn main()
            {
                let start = std::time::Instant::now();
                let result = [<project_euler_ $number>]();
                let elapsed = start.elapsed();
                println!("answer: {}", result);
                println!("took {:.2?}", elapsed);
            }
        }
    };
}
