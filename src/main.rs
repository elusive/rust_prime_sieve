mod engine;

extern crate stopwatch;

use std::env;
use stopwatch::{Stopwatch};


fn main() {
    // collect parameters from cmd line
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // no args passed
        1 => { println!("Please provide a number at which to cap the primes search:"); },
        // one argument passed
        2 => {
            let entered_value = &args[1];
            let parsed_value: usize = match entered_value.parse() {
                Ok(n) => { n /* return the parsed value */ },
                Err(_) => {
                    eprintln!("ERROR: could not parse number.");
                    return;
                }
            };
            // call our primes fn with parsed value
            let sw = Stopwatch::start_new();
            engine::primes(parsed_value);
            println!("Primes calculation took {}ms", sw.elapsed_ms());
        },
        // all other cases
        _ => { /* TODO: show help */ }
    }
}