mod primes;
pub use crate::primes::*;

fn main() {
    let mut sieve = primes::Sieve::new();
    println!("{}", sieve.next().unwrap()); // 2
    println!("{}", sieve.next().unwrap()); // 2
    println!("{}", sieve.next().unwrap()); // 2
    println!("{}", sieve.next().unwrap()); // 2
    println!("{}", sieve.next().unwrap()); // 2
    println!("{}", sieve.next().unwrap()); // 2
}
