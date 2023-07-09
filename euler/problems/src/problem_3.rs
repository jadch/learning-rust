// https://projecteuler.net/problem=3

/*
<p>The prime factors of 13195 are 5, 7, 13 and 29.</p>
<p>What is the largest prime factor of the number N = 600851475143 ?</p>

SOLUTION: 6857

ANSWER:
    - go through prime numbers in ASC order
    - each time you find a prime number P1 that is a factor of the number N
        => run the function recursively on the number (N / P1)
        => when the function is called on a number that is prime => return that number,
        the largest prime factor of N
*/
use num::integer::Roots;



pub fn largest_prime_factor(number: i64, largest_prime_factor_var: i64) -> i64 {
    let mut current_largest_prime_factor_var: i64 = largest_prime_factor_var;

    if number % 2 == 0 {
        if 2 > current_largest_prime_factor_var {
            current_largest_prime_factor_var = 2
        }
        return largest_prime_factor(number/2, current_largest_prime_factor_var);
    }

    let mut i: i64 = 3;
    while i <= number.sqrt() {
        if number % i == 0 {
            if i > current_largest_prime_factor_var {
                current_largest_prime_factor_var = i;
            }
            return largest_prime_factor(number/i, current_largest_prime_factor_var);
        }
        i = i + 2;
    }
    
    if number > current_largest_prime_factor_var {
        return number;
    } else {
        return current_largest_prime_factor_var;
    };
}