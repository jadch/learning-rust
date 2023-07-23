// https://projecteuler.net/problem=10

/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
Find the sum of all the primes below two million.

SOLUTION: building on "6 * k" method in problem_7 used to generate primes

ANSWER: 142913828922
*/
use crate::problem_5;

pub fn summation_of_primes(limit: i32) -> i64 {
    let mut sum: i64 = 2 + 3 + 5;
    let mut k: i64 = 1;

    while k * 6 + 5 <= limit as i64 {
        if problem_5::is_prime(6 * k +1) {
            sum += 6 * k  +1;
        }
        if problem_5::is_prime(6* k + 5) {
            sum += 6 * k + 5;
        }

        k += 1;
    }

    if problem_5::is_prime(6 * k +1) {
        sum += 6 * k  +1;
    }

    return sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_prime_tests() {
        assert_eq!(summation_of_primes(10), 17);
        assert_eq!(summation_of_primes(2000000), 142913828922);
    }
}
