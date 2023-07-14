// https://projecteuler.net/problem=6

/*
The sum of the squares of the first ten natural numbers is:
    1^2 + 2^2 + ... + 10^2 = 385
The square of the sum of the first ten natural numbers is,
    (1 + 2 + ... + 10)^2 = 55^2 = 3025
Hence the difference between the sum of the squares of the first ten natural numbers
and the square of the sum is 3025 - 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural
numbers and the square of the sum.

SOLUTION: brute force

ANSWER: 25164150
*/

pub fn sum_square_difference(limit: i64) -> i64 {
    let numbers: Vec<i64> = (1..=limit).collect();

    let sum_of_squares: i64 = numbers.iter().map(|num| num.pow(2)).sum();
    let square_of_sum: i64 = numbers.iter().sum::<i64>().pow(2);
    
    return square_of_sum - sum_of_squares;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smallest_multiple_tests() {
        assert_eq!(sum_square_difference(10), 2640);
        assert_eq!(sum_square_difference(100), 25164150);
    }
}