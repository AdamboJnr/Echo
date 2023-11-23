extern crate echo;

use std::process;
use std::env;
fn main() {
    // Creating a new Config instance
    let config = echo::Config::new(env::args()).unwrap_or_else(| err |  {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Returning error from print function
    if let Err(e) = echo::print_to_screen(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
    
}


