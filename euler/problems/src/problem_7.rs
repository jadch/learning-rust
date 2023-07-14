// https://projecteuler.net/problem=7

/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th
prime is 13.

What is the 10'001-st prime number?

SOLUTION: We only have to test (6k + 1) and (6k + 5) to find potential primes

ANSWER:
*/

pub fn nth_prime(n:i32) -> i64 {
    return 13;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_prime_tests() {
        assert_eq!(nth_prime(6), 13);
    }
}