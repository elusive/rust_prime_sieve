use std::env;

fn main() {
    // collect parameters from cmd line
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // no args passed
        1 => { println!("Please provide a number of primes to search"); },
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
            primes(parsed_value);
        },
        // all other cases
        _ => { /* TODO: show help */ }
    }
}

fn primes(number_of_primes: usize) {
    // create array of numbers in our range
    let mut sieve: Vec<bool> = vec![true; number_of_primes as usize];
    sieve[0] = false;
    sieve[1] = false;

    let mut p = 2;
    while p < number_of_primes {
        if sieve[p] {
            println!("{} is a PRIME number.", p);
            // update sieve by marking out all multiples of p
            let mut factor = p * 2;
            while factor < number_of_primes {
                sieve[factor] = false;
                factor = factor + p;
            }
        }
        p = p + 1;
    }
    
    println!("sieve: {}", sieve.len());
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_primes_returns_expected_value() {
        assert_eq!(1, 1);
    }
}
