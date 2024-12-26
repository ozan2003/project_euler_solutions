#[macro_export]
macro_rules! project_euler_solution {
    ($number:expr) => {
        paste::paste! {

            fn main() -> Result<(), Box<dyn std::error::Error>>
            {
                let start = std::time::Instant::now();
                let result = [<project_euler_ $number>]();
                let elapsed = start.elapsed();
                println!("answer: {}", result);
                println!("took {:.2?}", elapsed);
                Ok(())
            }
        }
    };
}
