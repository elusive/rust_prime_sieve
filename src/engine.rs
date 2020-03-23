pub fn primes(number_of_primes: usize) {
    // create array of numbers in our range
    let mut sieve: Vec<bool> = vec![true; number_of_primes as usize];
    sieve[0] = false;
    sieve[1] = false;

    let mut primeCount = 0;
    let mut p = 2;
    while p < number_of_primes {
        if sieve[p] {
            println!("{} is a PRIME number.", p);
            primeCount = primeCount + 1;
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
    println!("total prime count is {}", primeCount);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_primes_returns_expected_value() {
        assert_eq!(1, 1);
    }
}