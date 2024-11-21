use std::error::Error;

pub fn print_error(error: &Box<dyn Error>) {
    println!("Execution failed!");
    println!("Error: {}", error.to_string());
}