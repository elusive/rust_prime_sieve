# Prime Rust
A prime number sieve that I can use to test the performance of rust.  I also intend to use the code to play with the asynchronous capabilities of rust.  The prime sieve itself is very simple and not optimized.  Based on the same on I have always used, [Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).

## Getting Started
In order to run this code you must first clone the repository.  And then you need to [install rust](https://www.rust-lang.org/tools/install).  Once rust is installed you should take a look at [getting started](https://www.rust-lang.org/learn/get-started) page for the rust language, especially if you are unfamiliar.

### Executing the code
To execute the code, once you have installed rust (v1.41.1 or higher), use the `cargo` package manager and tooling to run the main script.  You will need to pass the number for the upper end of your prime number search.  e.g. passing 100 will search 0-100 for all prime numbers and write them to the screen.

```
cargo run 1000  // you can change this number
```

## Tests
You can run the unit tests, assuming I have added them by now, using the same `cargo` tool but with a different command.  The following will run the tests.

```
cargo test
```

## Intentions
The following items are planned:
1. Unit tests
2. Stopwatch timers for calculation
3. Other performance monitors as I learn about how to implement them.
4. Addition of help message and enhancement of the cli
5. ...

## Authors
John Gilliland
