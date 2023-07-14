// https://projecteuler.net/problem=7

/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th
prime is 13.

What is the 10'001-st prime number?

SOLUTION: We only have to test (6k + 1) and (6k + 5) to find potential primes

ANSWER: 104'743
*/

use crate::problem_5;

pub fn nth_prime(n:i32) -> i64 {
    let mut current_n: i32 = 3;
    let mut k: i64 = 1;

    while current_n <= n {
        if problem_5::is_prime(6 * k + 1) {
            current_n += 1;
            if current_n == n {
                return 6 * k + 1
            }
        }

        if problem_5::is_prime(6 * k + 5) {
            current_n += 1;
            if current_n == n {
                return 6 * k + 5
            }
        }

        k += 1;
    }

    return 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_prime_tests() {
        assert_eq!(nth_prime(6), 13);
        assert_eq!(nth_prime(7), 17);
        assert_eq!(nth_prime(7), 17);
        assert_eq!(nth_prime(21), 73);
        assert_eq!(nth_prime(90), 463);
        assert_eq!(nth_prime(10001), 104743);
    }
}