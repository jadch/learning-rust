// https://projecteuler.net/problem=4

/*
A palindromic number reads the same both ways. The largest palindrome made from the
product of two 2-digit numbers is 9009 = 91 * 99

Find the largest palindrome made from the product of two 3-digit numbers.

SOLUTION: brute force

ANSWER: 906609
 */

pub fn largest_palindrome_product() -> i32 {
    let mut largest_palindrome: i32 = 99;

    for first_num in (100..1000).rev() {
        let offset = 1000 - first_num;
        for second_num in (100..(1000 - offset)).rev() {
            let multiple = second_num * first_num;

            if multiple > largest_palindrome && is_palindrome(multiple) {
                largest_palindrome = multiple;
            }
        }
    }

    return largest_palindrome;
}

fn is_palindrome(number: i32) -> bool {
    let number_str = number.to_string();
    let digits = number_str.len();
    if digits == 1 {
        return true;
    }

    if digits % 2 == 0 {
        let left_part = number_str.clone().split_off(digits/2);
        let right_part = number_str.clone().chars().rev().collect::<String>().split_off(digits/2);

        return left_part == right_part;
    } else {
        let left_part = number_str.clone().split_off((digits- 1)/2);
        let right_part = number_str.clone().chars().rev().collect::<String>().split_off((digits-1)/2);
        
        return left_part == right_part;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_tests() {
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(6), true);
        assert_eq!(is_palindrome(99), true);
        assert_eq!(is_palindrome(999), true);
        assert_eq!(is_palindrome(989), true);
        assert_eq!(is_palindrome(987), false);
        assert_eq!(is_palindrome(1001), true);
        assert_eq!(is_palindrome(1105011), true);
        assert_eq!(is_palindrome(1235321), true);
    }
}
