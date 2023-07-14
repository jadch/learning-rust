// https://projecteuler.net/problem=5

/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10
without any remainder.

What is the smallest positive number that is evenly divisible (with no remainder) by all
of the numbers from 1 to 20 ?

SOLUTION: brute force

ANSWER: 232792560
 */
use num::integer::Roots;

pub fn smallest_multiple(limit: i64) -> i64 {
    let mut numbers_in_range: Vec<i64> = vec![];
    let mut primes_in_range: Vec<i64> = vec![];

    for num in 1..=limit {
        numbers_in_range.push(num);
        if is_prime(num) {
            primes_in_range.push(num)
        }
    }

    let min: i64 = primes_in_range.iter().product();
    let max: i64 = numbers_in_range.iter().product();

    for i in min..=max {
        if divisible_by_range(i, limit) {
            return i;
        }
    }
    return limit;
}

pub fn is_prime(number: i64) -> bool {
    let mut k: i64 = 1;

    if vec![1, 2, 3, 5].iter().any(|&x| x == number) {
        return true;
    }

    if number % 2 == 0 || number % 5 == 0 || number % 3 == 0 {
        return  false;
    }

    while (6 * k + 1) <= number.sqrt() {
        if number % (6 * k + 1) == 0 || number % (6 * k + 5) == 0 {
            return false;
        }
        k += 1;
    }
    return true;
}

fn divisible_by_range(number: i64, range_limit: i64) -> bool {
    for i in 2..=range_limit {
        if number % i > 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smallest_multiple_tests() {
        assert_eq!(smallest_multiple(10), 2520);
        assert_eq!(smallest_multiple(20), 232792560);
    }

    #[test]
    fn is_prime_tests() {
        assert_eq!(is_prime(1), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(17317), true);

        assert_eq!(is_prime(60), false);
        assert_eq!(is_prime(17318), false);
    }
}
