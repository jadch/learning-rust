// https://projecteuler.net/problem=3

/*
<p>The prime factors of 13195 are 5, 7, 13 and 29.</p>
<p>What is the largest prime factor of the number N = 600851475143$?</p>


ANSWER:
    - go through prime numbers in ASC order
    - each time you find a prime number P1 that is a factor of the number N
        => run the function recursively on the number (N / P1)
        => when the function is called on a number that is prime => return that number,
        the largest prime factor of N
*/

pub fn largest_prime_factor() {
    println!("Hello, world largest_prime_factor !");
}
