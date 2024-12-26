#[macro_export]
macro_rules! project_euler_solution {
    ($number:expr) => {
        paste::paste! {
            use std::time::Instant;

            fn main() -> Result<(), Box<dyn std::error::Error>>
            {
                let start = Instant::now();
                let result = [<project_euler_ $number>]();
                let duration = start.elapsed();
                println!("answer: {}", result);
                println!("took {}ms", duration.as_nanos() as f64 / 1_000_000.0);

                Ok(())
            }
        }
    };
}
